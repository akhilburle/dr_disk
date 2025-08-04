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

## Installation

To install `dr_disk`, you need to have Rust and Cargo installed on your system. If you don't have them, you can install them via `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once Rust and Cargo are installed, you can install `dr_disk` directly from crates.io (once published) or from the source code:

### From Crates.io (Recommended, once published)

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

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.