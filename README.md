# sshp - SSH Picker

<div align="center">

**A blazingly fast, interactive SSH host selector**

[![CI](https://github.com/kaushiktadhani/sshp/workflows/CI/badge.svg)](https://github.com/kaushiktadhani/sshp/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![GitHub release](https://img.shields.io/github/v/release/kaushiktadhani/sshp)](https://github.com/kaushiktadhani/sshp/releases)

[Features](#features) • [Quick Start](#quick-start) • [Usage](#usage) • [Documentation](#documentation)

</div>

---

## What is sshp?

**sshp** (SSH Picker) is a fast, interactive terminal tool that helps you browse and connect to your SSH hosts. Instead of typing `ssh user@long-hostname.com` or remembering host names, sshp reads your `~/.ssh/config` file and presents all your hosts in a searchable list. Just type to filter, press Enter to connect.

Think of it like **fzf for SSH** - designed specifically for managing server connections.

## Features

- 🚀 **Lightning fast** - Built in Rust for optimal performance
- 🔍 **Real-time search** - Filter hosts as you type (case-insensitive)
- 🎨 **Beautiful UI** - Colorful, modern terminal interface
- ⌨️  **Intuitive controls** - Arrow keys for navigation, Backspace to edit
- 🔌 **Auto-connect** - Instantly SSH into selected host on Enter
- 📝 **Zero configuration** - Works with your existing `~/.ssh/config`
- 💡 **Smart highlighting** - Search matches are highlighted in yellow
- 🎯 **Wildcard exclusion** - Automatically hides wildcard host entries

## Quick Start

### 🍎 macOS

```bash
curl -sSf https://raw.githubusercontent.com/kaushiktadhani/sshp/main/scripts/install.sh | sh
```

### 🐧 Linux

```bash
curl -sSf https://raw.githubusercontent.com/kaushiktadhani/sshp/main/scripts/install.sh | sh
```

### 🪟 Windows

```powershell
irm https://raw.githubusercontent.com/kaushiktadhani/sshp/main/scripts/install.ps1 | iex
```

**After installation:**
```bash
# Verify installation
which sshp  # macOS/Linux
Get-Command sshp  # Windows

# Run sshp
sshp
```

**Note:** If you get "command not found", see the [Installation Guide](docs/INSTALLATION.md) for PATH setup instructions.

## Usage

### Running sshp

Simply run `sshp` in your terminal:

```bash
sshp
```

### Controls

| Key | Action |
|-----|--------|
| `Type` | Search and filter hosts in real-time |
| `↑` / `↓` | Navigate through the filtered list |
| `Enter` | Connect to the selected host |
| `Backspace` | Delete characters from your search |
| `Esc` / `Ctrl+C` | Quit without connecting |

### Demo Example

Here's what you'll see when you run sshp:

```bash
$ sshp
SSH Host Selector
─────────────────────────────────────────────────
    production-server
    staging-server
  ▶ my-dev-box
Search: dev_
Use ↑/↓ to navigate, type to search, Enter to connect, Esc/Ctrl+C to quit
```

**What happens:**
1. All hosts from `~/.ssh/config` are displayed
2. As you type "dev", only matching hosts are shown
3. The selected host (marked with ▶) is highlighted in blue
4. Your search text "dev" is highlighted in yellow in each result
5. Press Enter to run `ssh my-dev-box`

## Configuration

### SSH Config File

**sshp** reads your existing SSH config file at `~/.ssh/config`. No additional configuration needed!

**If you don't have an SSH config yet:**

```bash
# Create the .ssh directory if it doesn't exist
mkdir -p ~/.ssh

# Create a config file
touch ~/.ssh/config

# Set proper permissions (important for SSH security)
chmod 600 ~/.ssh/config
```

### Example SSH Config

Here's a sample `~/.ssh/config` to get you started:

```ssh-config
Host production-server
    HostName 203.0.113.10
    User admin
    Port 22

Host staging-server
    HostName staging.example.com
    User deploy
    IdentityFile ~/.ssh/staging_key

Host my-dev-box
    HostName 192.168.1.100
    User developer

Host personal-vps
    HostName personal.example.com
    User root
    Port 2222
```

**Important:** Wildcard hosts (containing `*`) are automatically excluded from the sshp list. For example:

```ssh-config
Host *.internal.company.com
    User employee
    IdentityFile ~/.ssh/company_key
```

This wildcard entry won't appear in sshp, which is intentional - it's a template, not a specific host.

## Quick Shortcut (Optional)

**For even faster access**, create a short alias:

### macOS/Linux

```bash
# Add to ~/.zshrc or ~/.bashrc
alias s='sshp'

# Reload your shell
source ~/.zshrc  # or source ~/.bashrc
```

Now just type **`s`** and press Enter to launch sshp!

### Windows (PowerShell)

```powershell
# Check if you have a PowerShell profile
Test-Path $PROFILE

# If false, create it
New-Item -Path $PROFILE -Type File -Force

# Edit your profile
notepad $PROFILE

# Add this line:
Set-Alias -Name s -Value sshp

# Save and restart PowerShell
```

## How It Works

1. **Parses your `~/.ssh/config` file** to extract all `Host` entries
2. **Excludes wildcard hosts** (containing `*`) since they're templates, not specific hosts
3. **Displays them in an interactive list** with real-time search filtering
4. **Highlights your search** in yellow to show matching text
5. **Connects to your selection** by running `ssh <hostname>` when you press Enter

## Requirements

### Runtime Requirements
- **SSH config file** at `~/.ssh/config` (sshp reads this to find your hosts)
- **Terminal with ANSI color support** (most modern terminals support this)
- **SSH client** installed (macOS/Linux have this by default; Windows needs OpenSSH)

### Build Requirements (only if building from source)
- **Rust 1.70 or later** with `cargo` installed

## Why sshp?

If you manage multiple servers and find yourself:

- ❌ Constantly typing `ssh user@long-hostname-here.com`
- ❌ Forgetting host names in your SSH config
- ❌ Running `cat ~/.ssh/config` to remember your hosts
- ❌ Wanting a faster way to browse and connect to servers

Then **sshp is for you!**

✅ Just type `sshp`, filter by name, press Enter
✅ Fuzzy search works instantly as you type
✅ Beautiful terminal UI with colors and highlighting
✅ Works with your existing SSH setup - no extra config needed

**It's like fzf but specifically designed for SSH connections.**

## Documentation

- **[Installation Guide](docs/INSTALLATION.md)** - Detailed installation instructions for all platforms
- **[Troubleshooting](docs/TROUBLESHOOTING.md)** - Common issues and solutions
- **[Changelog](docs/CHANGELOG.md)** - Version history and changes
- **[Contributing](docs/CONTRIBUTING.md)** - How to contribute to the project

## Contributing

Contributions are welcome! Whether it's bug fixes, new features, or documentation improvements.

See the [Contributing Guide](docs/CONTRIBUTING.md) for details on how to get started.

### Quick Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/sshp.git
cd sshp

# Build and run in development mode
cargo run

# Run tests
cargo test

# Build optimized release
cargo build --release
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [fzf](https://github.com/junegunn/fzf) by Junegunn Choi - the gold standard for fuzzy finding
- Built with [crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal manipulation
- Thanks to all contributors and users who provide feedback and improvements

---

<div align="center">

Made with ❤️ and Rust

⭐ **Star this repo if you find it useful!**

[Report a Bug](https://github.com/kaushiktadhani/sshp/issues) • [Request a Feature](https://github.com/kaushiktadhani/sshp/issues) • [View Releases](https://github.com/kaushiktadhani/sshp/releases)

</div>
