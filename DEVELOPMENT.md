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

## 4. Release to APT (Debian/Ubuntu)

This makes `dr_disk` available via `apt install dr-disk` on Debian/Ubuntu-based systems.

### 4.1. Prepare the `debian/` Directory

All APT packaging steps should be performed inside a Debian/Ubuntu environment, ideally using Docker.

1.  **Start a Docker Container:**
    ```bash
    docker run -it --rm -v /path/to/your/dr_disk:/dr_disk ubuntu:latest /bin/bash
    ```
    (Replace `/path/to/your/dr_disk` with the actual path to your `dr_disk` project on your host machine.)

2.  **Inside the Docker Container, navigate to your project:**
    ```bash
    cd /dr_disk
    ```

3.  **Install Debian Packaging Tools and Rust Toolchain:**
    ```bash
    apt update && apt install -y dpkg-dev debhelper dh-cargo build-essential curl
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    . $HOME/.cargo/env
    ```

4.  **Create `debian/` Directory:**
    ```bash
    mkdir debian
    ```

5.  **Create `debian/control`:**
    ```
    Source: dr-disk
    Section: utils
    Priority: optional
    Maintainer: Your Name <your.email@example.com>
    Build-Depends: debhelper-compat (= 13), dh-cargo, rustc, cargo
    Standards-Version: 4.6.0
    Homepage: https://github.com/akhilburle/dr_disk

    Package: dr-disk
    Architecture: any
    Depends: ${shlibs:Depends}, ${misc:Depends}
    Description: An interactive Rust CLI tool for disk usage analysis.
     .
     dr_disk helps identify and clean up large files by recursively scanning
     directories, providing size-based color-coding, percentage display,
     and basic navigation.
    ```
    (Replace `Your Name <your.email@example.com>` with your actual name and email.)

6.  **Create `debian/changelog`:**
    ```
    dr-disk (X.Y.Z-1) unstable; urgency=medium

      * Initial release.

     -- Your Name <your.email@example.com>  Mon, 05 Aug 2025 10:00:00 +0000
    ```
    (Replace `X.Y.Z` with your actual version number, and `Your Name <your.email@example.com>` with your actual name and email, and adjust the date/time.)

7.  **Create `debian/compat`:**
    ```
    13
    ```

8.  **Create `debian/copyright`:**
    ```
    Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
    Upstream-Name: dr_disk
    Upstream-Contact: Your Name <your.email@example.com>
    Source: https://github.com/akhilburle/dr_disk

    Files: *
    Copyright: 2025 Your Name
    License: MIT

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
    ```
    (Replace `Your Name <your.email@example.com>` and `Your Name` with your actual name and email.)

9.  **Create `debian/rules`:**
    ```
    #!/usr/bin/make -f

    %:
    	dh $@ --with cargo
    ```
    (Make this file executable after creation: `chmod +x debian/rules`)

10. **Create `debian/install`:**
    ```
    target/release/dr_disk usr/bin/
    ```

### 4.2. Build the `.deb` Package

Inside the Docker container, from the root of your project (`/dr_disk`):

```bash
dpkg-buildpackage -us -uc
```

This will generate several files in the parent directory (`/` in the container), including:
*   `dr-disk_X.Y.Z-1_arm64.deb` (the actual Debian package)
*   `dr-disk_X.Y.Z-1.dsc` (Debian source control file)
*   `dr-disk_X.Y.Z.orig.tar.gz` (original source tarball)
*   `dr-disk_X.Y.Z-1.debian.tar.xz` (Debian-specific changes)
*   `dr-disk_X.Y.Z-1_arm64.changes` (changes file for uploading to repositories)

### 4.3. Distribute via PPA (Personal Package Archive)

This process is done outside the Docker container, on your host machine, after you have built the `.deb` package and its associated source files.

1.  **Create a Launchpad Account:** If you don't have one, sign up at [Launchpad](https://launchpad.net/).

2.  **Create a PPA:**
    *   Log in to Launchpad.
    *   Go to your personal page.
    *   Click on "Create a new PPA".
    *   Give it a descriptive name (e.g., `dr-disk`).

3.  **Sign the `.changes` file:**
    You need to sign the `.changes` file generated by `dpkg-buildpackage` with your GPG key. If you don't have a GPG key, you'll need to generate one (`gpg --full-generate-key`).

    ```bash
    debsign -k <YOUR_GPG_KEY_ID> dr-disk_X.Y.Z-1_arm64.changes
    ```
    (Replace `<YOUR_GPG_KEY_ID>` with your GPG key ID, and `dr-disk_X.Y.Z-1_arm64.changes` with the actual filename.)

4.  **Upload to your PPA:**
    Use `dput` to upload the signed `.changes` file (which references the `.deb` and source files) to your PPA.

    ```bash
    dput ppa:<your-launchpad-id>/<your-ppa-name> dr-disk_X.Y.Z-1_arm64.changes
    ```
    (Replace `<your-launchpad-id>` and `<your-ppa-name>` with your actual Launchpad ID and PPA name.)

    Launchpad will then build the packages for various Ubuntu versions. This can take some time.

### 4.4. Testing APT Release

Once your package is built on Launchpad, you can test it in a clean Ubuntu environment (e.g., another Docker container):

1.  **Start a clean Ubuntu Docker Container:**
    ```bash
    docker run -it --rm ubuntu:latest /bin/bash
    ```

2.  **Inside the container, add your PPA:**
    ```bash
    apt update
    apt install -y software-properties-common
    add-apt-repository ppa:<your-launchpad-id>/<your-ppa-name>
    apt update
    ```

3.  **Install `dr-disk`:**
    ```bash
    apt install dr-disk
    ```

4.  **Verify Installation:**
    ```bash
    dr_disk --help
    ```

## 5. Future: Release to YUM/DNF (RHEL/CentOS/Fedora)

Instructions for publishing to `yum`/`dnf` will be added here once that process is established. This typically involves creating `.rpm` packages and setting up a COPR repository.