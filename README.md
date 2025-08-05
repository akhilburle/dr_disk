# dr_disk

`dr_disk` is an interactive command-line interface (CLI) tool written in Rust designed to help you analyze and manage disk space efficiently. It provides a clear, color-coded overview of file and directory sizes, making it easy to identify large files and folders that might be good candidates for cleanup.

## Features

-   **Recursive Directory Scanning:** Scans the current directory and its subdirectories to calculate accurate sizes for all files and folders.
-   **Interactive Navigation:** Easily navigate into subdirectories and back up to parent directories directly within the tool.
-   **Size-Based Color-Coding:** Files and folders are color-coded based on their size relative to the current view or the total disk space, helping you quickly spot space hogs:
    -   **Red:** Very large items.
    -   **Yellow:** Moderately large items.
    -   **Green:** Smaller items.
-   **Percentage Display:** Shows the percentage of the relevant total size that each file or folder occupies.
-   **Configurable Color Basis:** Choose whether the color thresholds are based on the total size of the currently displayed items (default) or the total capacity of the disk.
-   **Real-time Progress:** Displays a progress bar during scanning to keep you informed.

## Prerequisites
- **Rust and Cargo**: Required for all platforms. Install via [rustup](https://rustup.rs/):
  ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- After installation, verify with `rustc --version` and `cargo --version`.

- **Windows-Specific**: Rust on Windows uses the MSVC toolchain by default, which requires the Visual Studio Build Tools (free from Microsoft). You can install Rust using Winget (Windows Package Manager) or rustup, and it may automatically prompt you to install the Build Tools.
- **Via Winget (Recommended for Windows)**: Ensure Winget is installed (it's built-in on Windows 10/11; if not, get it from the Microsoft Store). Then run:
  ```
  winget install Rustlang.Rust.MSVC
  ```
  This installs Rust and Cargo, and will guide you to install Visual Studio Build Tools if they're missing (select the "Desktop development with C++" workload during setup for the necessary linker `link.exe` and C++ tools; ~5-6 GB required).
- **Via rustup**: Download and run `rustup-init.exe` from [rustup.rs](https://rustup.rs/). It will prompt for Visual Studio Build Tools if needed.
- Download Build Tools manually from the [Visual Studio downloads page](https://visualstudio.microsoft.com/downloads/) (select "Build Tools for Visual Studio 2022" or later) if prompts fail.
- Alternative: If you prefer to avoid Microsoft tools, switch to the GNU toolchain with `rustup default stable-x86_64-pc-windows-gnu` (requires MinGW installation via MSYS2).

- **macOS/Linux**: No additional tools needed beyond Rust/Cargo.



## Installation

Ensure you meet the [Prerequisites](#prerequisites) before proceeding, especially on Windows where Rust installation may require Visual Studio Build Tools to build binaries.


To install `dr_disk`, you need to have Rust and Cargo installed on your system. If you don't have them, you can install them via `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once Rust and Cargo are installed, you can install `dr_disk` using one of the following methods:

### Homebrew (macOS)

```bash
brew tap akhilburle/tap
brew install dr_disk
```

### From Crates.io (Recommended)

```bash
cargo install dr_disk
```

### From Source

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/akhilburle/dr_disk.git
    cd dr_disk
    ```

2.  **Install from source:**
    ```bash
    cargo install --path .
    ```
   
**Note for Windows Users**: If you encounter errors like "linker `link.exe` not found" during compilation, verify that Visual Studio Build Tools are installed with the C++ workload (see [Prerequisites](#prerequisites)). After installation, restart your terminal and retry. To check if `link.exe` is accessible, run `link.exe /?` in Command Prompt—if it fails, add the tool's directory to your PATH (e.g., `C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.x.x.xxx\bin\Hostx64\x64`).


## Usage

Once installed, you can run `dr_disk` from any directory:

### Basic Usage

To run `dr_disk` in the current directory:

```bash
dr_disk
```

### Command-Line Arguments

-   `--total-disk-color`: Use this flag to base the color thresholds on the total capacity of the disk where the scanned directory resides, instead of the total size of the currently displayed items. This is useful for understanding the impact of files relative to your entire storage.

    ```bash
    dr_disk --total-disk-color
    ```

-   `path`: You can specify a different path to scan instead of the current directory:

    ```bash
    dr_disk /path/to/another/directory
    ```

### Interactive Commands

Once `dr_disk` is running, you can use the following commands in the interactive prompt (`>`):

-   `cd <directory>`: Change the current directory to the specified subdirectory.
-   `..` or `up`: Move up to the parent directory.
-   `q` or `quit`: Exit the application.
-   `help`: Display a list of available commands.

## Color-Coding Explained

The color of each entry (file or folder) indicates its size relative to a total. By default, this total is the sum of all items currently displayed in the list. If you use the `--total-disk-color` flag, the total will be the overall capacity of the disk.

-   **Red:** Items that are a significant portion of the total (e.g., >10% of current view, or >1% of total disk).
-   **Yellow:** Items that are moderately large (e.g., >1% of current view, or >0.1% of total disk).
-   **Green:** Smaller items.

These thresholds are dynamic and adapt to the context (either the current view or the total disk space) to provide meaningful visual cues.

## Troubleshooting

- **Compilation Errors on Windows (e.g., "linker `link.exe` not found")**: This indicates missing Visual Studio Build Tools. Install them as described in [Prerequisites](#prerequisites), ensuring the "Desktop development with C++" workload is selected. Restart your system and verify with `link.exe /?`.

- **Cargo Fails to Build**: Run `cargo clean` to remove old artifacts, then retry. Ensure your Rust installation is up to date with `rustup update`.

- If problems persist, check the [Rust documentation](https://doc.rust-lang.org/book/ch01-01-installation.html) or open an issue on this repository.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
