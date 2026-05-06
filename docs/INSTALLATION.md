# Installation Guide

This guide covers all methods of installing sshp on your system.

## Table of Contents

- [Quick Install (Recommended)](#quick-install-recommended)
- [Installation Methods](#installation-methods)
  - [Shell Script (macOS/Linux)](#shell-script-macoslinux)
  - [PowerShell Script (Windows)](#powershell-script-windows)
  - [Manual Binary Download](#manual-binary-download)
  - [Build from Source](#build-from-source)
- [Uninstall](#uninstall)

---

## Quick Install (Recommended)

### macOS / Linux

```bash
curl -sSf https://raw.githubusercontent.com/kaushiktadhani/sshp/main/scripts/install.sh | sh
```

### Windows

```powershell
irm https://raw.githubusercontent.com/kaushiktadhani/sshp/main/scripts/install.ps1 | iex
```

---

## Installation Methods

### Shell Script (macOS/Linux)

**Step 1: Run the installer**

```bash
curl -sSf https://raw.githubusercontent.com/kaushiktadhani/sshp/main/scripts/install.sh | sh
```

The installer will:
- Download the latest version of sshp for your platform
- Verify the download with SHA256 checksum
- Install to `/usr/local/bin` (if writable) or `~/.local/bin`
- Show a warning if the install directory is not in your PATH

**Step 2: Verify installation**

```bash
which sshp
```

**Expected output:**
```
/usr/local/bin/sshp
# or
/Users/yourusername/.local/bin/sshp
```

**If you see "sshp not found":**
The install directory is not in your PATH. Continue to Step 3.

**Step 3: Add to PATH (if needed)**

If the installer showed a PATH warning or `which sshp` returned nothing, add the install directory to your shell configuration:

**For zsh users (macOS default):**
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

**For bash users:**
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**Note:** Use the exact directory path shown in the installer warning message.

**Step 4: Test it**

```bash
sshp
```

**Expected behavior:**
- If you have an `~/.ssh/config` file with hosts, you'll see a searchable list
- If you don't have an SSH config yet, see the [Configuration section](../README.md#configuration)

---

### PowerShell Script (Windows)

**Step 1: Run the installer**

Open PowerShell and run:

```powershell
irm https://raw.githubusercontent.com/kaushiktadhani/sshp/main/scripts/install.ps1 | iex
```

The installer will:
- Download the latest version of sshp for Windows (x86_64)
- Verify the download with SHA256 checksum
- Install to `%USERPROFILE%\.local\bin\sshp.exe`
- Automatically add the directory to your user PATH

**Step 2: Restart your terminal**

Close and reopen PowerShell for the PATH change to take effect.

**Step 3: Verify installation**

```powershell
Get-Command sshp
```

**Expected output:**
```
CommandType     Name          Version    Source
-----------     ----          -------    ------
Application     sshp.exe      0.0.0.0    C:\Users\YourName\.local\bin\sshp.exe
```

**Step 4: Test it**

```powershell
sshp
```

**Expected behavior:**
- If you have an `~/.ssh/config` file with hosts, you'll see a searchable list
- If you don't have an SSH config yet, see the [Configuration section](../README.md#configuration)

---

### Manual Binary Download

If you prefer to install manually or the scripts don't work for your system:

**Step 1: Download the binary**

Go to the [Releases page](https://github.com/kaushiktadhani/sshp/releases) and download the file for your platform:

| Platform | File |
|----------|------|
| macOS (Apple Silicon) | `sshp-v*-aarch64-apple-darwin.tar.gz` |
| macOS (Intel) | `sshp-v*-x86_64-apple-darwin.tar.gz` |
| Linux x86_64 | `sshp-v*-x86_64-unknown-linux-gnu.tar.gz` |
| Linux ARM64 | `sshp-v*-aarch64-unknown-linux-gnu.tar.gz` |
| Windows x86_64 | `sshp-v*-x86_64-pc-windows-msvc.zip` |

**Step 2: Extract the archive**

**macOS/Linux:**
```bash
# Extract the .tar.gz file
tar -xzf sshp-v*-your-platform.tar.gz

# This creates a binary called 'sshp'
```

**Windows:**
```powershell
# Right-click the .zip file and select "Extract All"
# Or use PowerShell:
Expand-Archive sshp-v*-x86_64-pc-windows-msvc.zip -DestinationPath .
```

**Step 3: Move to a directory in your PATH**

**macOS/Linux:**
```bash
# Move to /usr/local/bin (requires sudo)
sudo mv sshp /usr/local/bin/

# Or move to ~/.local/bin (no sudo needed)
mkdir -p ~/.local/bin
mv sshp ~/.local/bin/

# Make executable
chmod +x ~/.local/bin/sshp

# Add to PATH if using ~/.local/bin
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

**Windows:**
```powershell
# Create directory
New-Item -ItemType Directory -Path "$env:USERPROFILE\.local\bin" -Force

# Move binary
Move-Item sshp.exe "$env:USERPROFILE\.local\bin\"

# Add to PATH
$UserPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
[System.Environment]::SetEnvironmentVariable("Path", "$UserPath;$env:USERPROFILE\.local\bin", "User")

# Restart your terminal
```

**Step 4: Verify**

```bash
# macOS/Linux
which sshp

# Windows (PowerShell)
Get-Command sshp
```

---

### Build from Source

If you're a Rust developer or want to build the latest development version:

**Step 1: Install Rust**

If you don't have Rust installed:

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**
Download and run [rustup-init.exe](https://rustup.rs/)

**Step 2: Clone and build**

```bash
# Clone the repository
git clone https://github.com/kaushiktadhani/sshp.git
cd sshp

# Build release version
cargo build --release

# The binary will be at: target/release/sshp
```

**Step 3: Install the binary**

```bash
# Install to ~/.cargo/bin (should already be in PATH if you installed Rust)
cargo install --path .

# Or manually copy the binary
cp target/release/sshp ~/.local/bin/
```

**Step 4: Run it**

```bash
sshp
```

---

## Uninstall

### macOS/Linux

If you used the installer script:

```bash
# Find where sshp is installed
which sshp

# Remove it (usually one of these)
rm ~/.local/bin/sshp
# or
sudo rm /usr/local/bin/sshp
```

**Optional:** Remove from PATH in `~/.zshrc` or `~/.bashrc`

Delete this line if you added it:
```bash
export PATH="$HOME/.local/bin:$PATH"
```

Then reload your shell:
```bash
source ~/.zshrc  # or source ~/.bashrc
```

---

### Windows

If you used the PowerShell installer:

```powershell
# Remove the binary
Remove-Item "$env:USERPROFILE\.local\bin\sshp.exe"
```

**Optional:** Remove from PATH

1. Press `Win + X` and select "System"
2. Click "Advanced system settings"
3. Click "Environment Variables"
4. Under "User variables", select "Path" and click "Edit"
5. Find and remove the entry: `%USERPROFILE%\.local\bin`
6. Click OK to save

---

## Need Help?

If you encounter any issues during installation, check the [Troubleshooting Guide](TROUBLESHOOTING.md).
