use clap::Parser;
use colored::*;
use filesize::PathExt;
use human_format::Formatter;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::thread;

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
        current_path: args.path,
        total_disk_space,
    };

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
        } else if !input.is_empty() {
            println!("Unknown command: {}. Type 'help' for a list of commands.", input);
        }
    }

    Ok(())
}

fn get_dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().size_on_disk().unwrap_or(0))
        .sum()
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
            let size = if path.is_dir() {
                get_dir_size(path)
            } else {
                path.size_on_disk().unwrap_or(0)
            };
            bar_clone.inc(1);
            (path.to_path_buf(), size)
        });
        handles.push(handle);
    }

    let mut entries = vec![];
    for handle in handles {
        entries.push(handle.join().unwrap());
    }

    bar.finish_with_message("Scan complete!");

    entries.sort_by(|a, b| b.1.cmp(&a.1));

    println!("{:<50} {:>15} {:>10}", "Path", "Size", "%");
    println!("{}", "-".repeat(76));

    let (red_threshold, yellow_threshold, total_for_percentage) = if let Some(total_disk_space) = state.total_disk_space {
        (total_disk_space / 100, total_disk_space / 1000, total_disk_space) // 1% and 0.1% of total disk space
    } else {
        let total_displayed_size: u64 = entries.iter().map(|(_, size)| size).sum();
        (total_displayed_size / 10, total_displayed_size / 100, total_displayed_size) // 10% and 1% of current view
    };

    for (path, size) in &entries {
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

        println!(
            "{:<50} {:>15} {:>10}",
            path_display,
            size_str.to_string().color(color),
            percentage_str.to_string().color(color)
        );
    }

    Ok(())
}