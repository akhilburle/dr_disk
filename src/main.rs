use clap::Parser;
use colored::*;
use filesize::PathExt;
use human_format::Formatter;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::thread;
use chrono::{DateTime, Local};
use std::fs;
use std::time::SystemTime;

use sysinfo::{Disk, Disks};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the directory to scan
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Use total disk space for color thresholds instead of current view's total size
    #[arg(long)]
    total_disk_color: bool,

    /// Run the tool once and exit, without entering interactive mode.
    #[arg(long)]
    once: bool,
}

struct State {
    current_path: PathBuf,
    total_disk_space: Option<u64>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut total_disk_space: Option<u64> = None;
    if args.total_disk_color {
        let disks = Disks::new_with_refreshed_list();
        let canonical_path = args.path.canonicalize()?;

        let mut best_match: Option<(&Disk, usize)> = None;
        for disk in &disks {
            let mount_point = disk.mount_point();
            if canonical_path.starts_with(mount_point) {
                let match_len = mount_point.as_os_str().len();
                if best_match.is_none() || match_len > best_match.unwrap().1 {
                    best_match = Some((disk, match_len));
                }
            }
        }

        total_disk_space = if let Some((disk, _)) = best_match {
            Some(disk.total_space())
        } else {
            anyhow::bail!("Could not determine disk space for the given path. Try running without --total-disk-color.");
        };
    }

    let mut state = State {
        current_path: args.path.canonicalize()?,
        total_disk_space,
    };

    if args.once {
        if !state.current_path.is_dir() {
            anyhow::bail!(
                "Provided path is not a directory: {}",
                state.current_path.display()
            );
        }
        scan_and_display(&state)?;
    } else {
        loop {
            if !state.current_path.is_dir() {
                anyhow::bail!(
                    "Provided path is not a directory: {}",
                    state.current_path.display()
                );
            }

            scan_and_display(&state)?;

            print!("> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input == "q" || input == "quit" {
                break;
            } else if input == ".." || input == "up" {
                if let Some(parent) = state.current_path.parent() {
                    state.current_path = parent.to_path_buf();
                }
            } else if input.starts_with("cd ") {
                let new_dir = input.split_at(3).1;
                let new_path = state.current_path.join(new_dir);
                if new_path.is_dir() {
                    state.current_path = new_path.canonicalize()?;
                } else {
                    println!("Directory not found: {}", new_dir);
                }
            } else if input == "help" {
                println!("Commands: cd <dir>, .., up, q, quit, help");
            }
        }
    }

    Ok(())
}

fn get_dir_size_and_modified(path: &Path) -> (u64, Option<SystemTime>) {
    let mut total_size = 0;
    let mut latest_modified: Option<SystemTime> = None;

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        if entry_path.is_file() {
            total_size += entry_path.size_on_disk().unwrap_or(0);
        }

        if let Ok(metadata) = fs::metadata(entry_path) {
            if let Ok(modified) = metadata.modified() {
                if latest_modified.is_none() || modified > latest_modified.unwrap() {
                    latest_modified = Some(modified);
                }
            }
        }
    }
    (total_size, latest_modified)
}

fn scan_and_display(state: &State) -> anyhow::Result<()> {
    let entries_to_scan: Vec<_> = WalkDir::new(&state.current_path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .collect();

    let bar = ProgressBar::new(entries_to_scan.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")?
            .progress_chars("##-"),
    );
    bar.set_message(format!("Calculating sizes for {}...", state.current_path.display()));

    let mut handles = vec![];
    for entry in entries_to_scan {
        let bar_clone = bar.clone();
        let handle = thread::spawn(move || {
            let path = entry.path();
            let (size, modified_time) = if path.is_dir() {
                get_dir_size_and_modified(path)
            } else {
                (path.size_on_disk().unwrap_or(0), fs::metadata(&path).and_then(|m| m.modified()).ok())
            };
            bar_clone.inc(1);
            (path.to_path_buf(), size, modified_time)
        });
        handles.push(handle);
    }

    let mut entries = vec![];
    for handle in handles {
        entries.push(handle.join().unwrap());
    }

    bar.finish_with_message("Scan complete!");

    entries.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Summary for: {}", state.current_path.display().to_string().bold());
    println!("{:<40} {:>15} {:>10} {:>20}", "Path", "Size", "%", "Last Touched");
    println!("{}", "-".repeat(90));

    let (red_threshold, yellow_threshold, total_for_percentage) = if let Some(total_disk_space) = state.total_disk_space {
        (total_disk_space / 100, total_disk_space / 1000, total_disk_space) // 1% and 0.1% of total disk space
    } else {
        let total_displayed_size: u64 = entries.iter().map(|(_, size, _)| size).sum();
        (total_displayed_size / 10, total_displayed_size / 100, total_displayed_size) // 10% and 1% of current view
    };

    let now = Local::now();

    for (path, size, modified_time) in &entries {
        let size_str = Formatter::new().format(*size as f64);
        let percentage = (*size as f64 / total_for_percentage as f64) * 100.0;
        let percentage_str = format!("{:.2}", percentage);

        let color = if *size > red_threshold {
            "red"
        } else if *size > yellow_threshold {
            "yellow"
        } else {
            "green"
        };

        let path_display = if path.is_dir() {
            format!("{}/", path.file_name().unwrap().to_str().unwrap()).blue()
        } else {
            path.file_name().unwrap().to_str().unwrap().normal()
        };

        let modified_str = if let Some(time) = modified_time {
            let datetime: DateTime<Local> = (*time).into();
            let duration = now.signed_duration_since(datetime);

            if duration.num_days() > 0 {
                format!("{} days ago", duration.num_days())
            } else if duration.num_hours() > 0 {
                format!("{} hours ago", duration.num_hours())
            } else if duration.num_minutes() > 0 {
                format!("{} minutes ago", duration.num_minutes())
            } else {
                "just now".to_string()
            }
        } else {
            "N/A".to_string()
        };

        println!(
            "{:<40} {:>15} {:>10} {:>20}",
            path_display,
            size_str.to_string().color(color),
            percentage_str.to_string().color(color),
            modified_str
        );
    }

    Ok(())
}