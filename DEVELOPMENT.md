# DEVELOPMENT.md - Release Process for `dr_disk`

This document outlines the steps required to prepare and release new versions of `dr_disk` to various package managers and `crates.io`.

## 1. Versioning

Before starting any release process, ensure that the `version` in `Cargo.toml` is updated to the new target version (e.g., `0.1.1`).

## 2. Release to `crates.io`

This makes `dr_disk` available via `cargo install dr_disk`.

1.  **Ensure `Cargo.toml` is up-to-date:**
    *   `name`, `version`, `description`, `license`, `repository`, `keywords`, `categories`, `readme` fields should be accurate.

2.  **Login to `crates.io` (if not already logged in):**
    ```bash
    cargo login <YOUR_CRATES_IO_API_TOKEN>
    ```
    (Get your API token from [crates.io/me](https://crates.io/me)).

3.  **Run `cargo publish`:**
    Navigate to the root of the `dr_disk` project and execute:
    ```bash
    cargo publish
    ```
    This will build, package, and upload your crate to `crates.io`.

## 3. Release to Homebrew (macOS)

This makes `dr_disk` available via `brew install dr_disk` through your tap.

1.  **Create a Git Tag:**
    Tag the commit corresponding to the version you just published to `crates.io`.
    ```bash
    git tag -a vX.Y.Z -m "Release vX.Y.Z"
    ```
    (Replace `X.Y.Z` with your actual version number, e.g., `v0.1.0`).

2.  **Push the Tag to GitHub:**
    ```bash
    git push origin vX.Y.Z
    ```

3.  **Create a GitHub Release:**
    *   Go to your `dr_disk` repository on GitHub: `https://github.com/akhilburle/dr_disk/releases`.
    *   Click "Draft a new release" or "Create a new release".
    *   Select the `vX.Y.Z` tag you just pushed.
    *   Fill in the release title (e.g., "dr_disk vX.Y.Z - Release Notes").
    *   Add a description (e.g., copy from `README.md` features or a changelog).
    *   Click "Publish release". This will generate source code archives (tarball, zip) that Homebrew will use.

4.  **Get the SHA256 Checksum:**
    Download the source code tarball for your new release and calculate its SHA256 checksum. The URL will be in the format `https://api.github.com/repos/akhilburle/dr_disk/tarball/vX.Y.Z`.
    ```bash
    curl -L https://api.github.com/repos/akhilburle/dr_disk/tarball/vX.Y.Z -o dr_disk-vX.Y.Z.tar.gz
    shasum -a 256 dr_disk-vX.Y.Z.tar.gz
    ```
    Copy the checksum output.

5.  **Update the Homebrew Formula:**
    *   Go to your `homebrew-tap` repository (e.g., `akhilburle/homebrew-tap`).
    *   Navigate to `Formula/dr_disk.rb`.
    *   Edit the `dr_disk.rb` file:
        *   Update the `url` to point to the new tag: `url "https://api.github.com/repos/akhilburle/dr_disk/tarball/vX.Y.Z"`
        *   Update the `sha256` with the new checksum you obtained.
    *   Commit and push these changes to your `homebrew-tap` repository.

## 4. Testing New Releases

It's crucial to test the installation of your new release in an isolated environment.

### 4.1. Testing `crates.io` Release

1.  **Force Reinstall:**
    ```bash
    cargo install --force dr_disk
    ```
    This will force Cargo to download and reinstall the latest version of `dr_disk` from `crates.io`.

2.  **Verify Installation:**
    ```bash
    dr_disk --help
    # Or run a simple command to ensure functionality
    dr_disk .
    ```

### 4.2. Testing Homebrew Release

1.  **Untap and Retap (for a clean test):**
    ```bash
    brew untap akhilburle/tap
    brew tap akhilburle/tap
    ```

2.  **Uninstall (if previously installed):**
    ```bash
    brew uninstall dr_disk
    ```

3.  **Install the new version:**
    ```bash
    brew install dr_disk
    ```

4.  **Verify Installation:**
    ```bash
    dr_disk --help
    # Or run a simple command to ensure functionality
    dr_disk .
    ```

### 4.3. Isolated Testing with Docker (Advanced)

For more robust testing, especially for `apt` and `yum` releases, you can use Docker containers to simulate clean environments.

**General Steps:**

1.  **Pull a clean image:**
    ```bash
    docker pull ubuntu:latest   # For APT testing
    docker pull fedora:latest   # For YUM/DNF testing
    ```

2.  **Run the container:**
    ```bash
    docker run -it ubuntu:latest /bin/bash
    # Inside the container, install necessary dependencies (e.g., curl, build-essential for Rust)
    # Then, attempt to install dr_disk using the respective package manager commands.
    ```

3.  **Verify installation inside the container.**

## 5. Future: Release to APT (Debian/Ubuntu) and YUM/DNF (RHEL/CentOS/Fedora)

Instructions for publishing to `apt` and `yum`/`dnf` will be added here once those processes are established. These typically involve creating `.deb` and `.rpm` packages, respectively, and setting up package repositories (e.g., PPAs for Ubuntu, COPR for Fedora/CentOS).
