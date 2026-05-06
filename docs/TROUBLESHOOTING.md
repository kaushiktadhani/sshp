# Troubleshooting Guide

This guide covers common issues you might encounter when installing or using sshp.

## Table of Contents

- [macOS Issues](#macos)
- [Linux Issues](#linux)
- [Windows Issues](#windows)
- [All Platforms](#all-platforms)

---

## macOS

### Problem: "Developer cannot be verified" warning

macOS Gatekeeper may block the downloaded binary because it's not signed.

**Solution:**
```bash
# Remove the quarantine attribute
xattr -d com.apple.quarantine ~/.local/bin/sshp
# or
xattr -d com.apple.quarantine /usr/local/bin/sshp
```

**Alternative:**
1. Try to run `sshp`
2. When the warning appears, click "Cancel"
3. Go to System Preferences → Security & Privacy → General
4. Click "Allow Anyway" next to the sshp message
5. Run `sshp` again and click "Open"

---

### Problem: `sshp: command not found`

The install directory is not in your PATH.

**Solution:**

First, find where sshp is installed:
```bash
ls -l /usr/local/bin/sshp
ls -l ~/.local/bin/sshp
```

**If it's in `~/.local/bin`:**
```bash
# For zsh (macOS default)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

# Verify
which sshp
```

**Expected output after fixing:**
```
/Users/yourusername/.local/bin/sshp
```

---

### Problem: `Permission denied`

The binary doesn't have execute permissions.

**Solution:**
```bash
chmod +x ~/.local/bin/sshp
# or
chmod +x /usr/local/bin/sshp
```

---

## Linux

### Problem: `sshp: command not found`

The install directory is not in your PATH.

**Solution:**

First, find where sshp is installed:
```bash
ls -l /usr/local/bin/sshp
ls -l ~/.local/bin/sshp
```

**If it's in `~/.local/bin`:**
```bash
# For bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# For zsh
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

# Verify
which sshp
```

**Expected output after fixing:**
```
/home/yourusername/.local/bin/sshp
```

---

### Problem: `Permission denied`

The binary doesn't have execute permissions.

**Solution:**
```bash
chmod +x ~/.local/bin/sshp
# or
chmod +x /usr/local/bin/sshp
```

---

## Windows

### Problem: PowerShell execution policy error

**Error message:**
```
irm : Cannot run script because running scripts is disabled on this system.
```

**Solution:**

Enable script execution for the current user:
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

Then run the installer again:
```powershell
irm https://raw.githubusercontent.com/kaushiktadhani/sshp/main/scripts/install.ps1 | iex
```

---

### Problem: `sshp : The term 'sshp' is not recognized`

The PATH environment variable hasn't been updated in your current terminal session.

**Solution:**

1. **Restart your terminal** (close and reopen PowerShell)
2. Try running `sshp` again

**If still not working:**

Check if the PATH was updated:
```powershell
$env:Path -split ';' | Select-String "\.local\\bin"
```

**Expected output:**
```
C:\Users\YourName\.local\bin
```

**If nothing is returned**, manually add to PATH:
```powershell
$UserPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
[System.Environment]::SetEnvironmentVariable("Path", "$UserPath;$env:USERPROFILE\.local\bin", "User")
```

Then restart your terminal.

---

## All Platforms

### Problem: No hosts showing up / "No SSH hosts found"

**Possible causes:**
1. You don't have an SSH config file
2. Your SSH config only has wildcard hosts (containing `*`)
3. Your SSH config file has no `Host` entries

**Solution:**

**Check if the config file exists:**
```bash
# macOS/Linux
ls -la ~/.ssh/config

# Windows (PowerShell)
Test-Path ~/.ssh/config
```

**If the file doesn't exist:**
```bash
# macOS/Linux
mkdir -p ~/.ssh
touch ~/.ssh/config
chmod 600 ~/.ssh/config

# Windows (PowerShell)
New-Item -ItemType Directory -Path "$env:USERPROFILE\.ssh" -Force
New-Item -ItemType File -Path "$env:USERPROFILE\.ssh\config" -Force
```

**If the file exists but is empty**, add some hosts:
```bash
# macOS/Linux
nano ~/.ssh/config

# Windows
notepad ~/.ssh/config
```

Add at least one host entry:
```ssh-config
Host example-server
    HostName example.com
    User myuser
```

**Check for wildcard exclusion:**

If your config only has entries like this:
```ssh-config
Host *.example.com
    User developer
```

sshp will exclude them because wildcards are templates, not specific hosts. Add specific host entries.

---

### Problem: sshp shows hosts but SSH connection fails

**Error:** `ssh: Could not resolve hostname...` or `Permission denied`

**This is an SSH configuration issue, not an sshp issue.**

**Solution:**

1. Test the SSH connection manually:
```bash
ssh your-host-name
```

2. Common SSH config issues:
   - Wrong `HostName` (check IP address or domain)
   - Wrong `User` (check username)
   - Missing or wrong `IdentityFile` (check SSH key path)
   - Firewall or network issues

3. Verify your SSH config syntax:
```bash
# macOS/Linux
cat ~/.ssh/config

# Windows (PowerShell)
Get-Content ~/.ssh/config
```

Each host entry should look like:
```ssh-config
Host my-server
    HostName 192.168.1.100
    User username
    Port 22
```

---

## Still Having Issues?

If you're still experiencing problems:

1. Check the [GitHub Issues](https://github.com/kaushiktadhani/sshp/issues) to see if someone else has encountered the same problem
2. [Open a new issue](https://github.com/kaushiktadhani/sshp/issues/new/choose) with details about your problem
3. Include:
   - Your operating system and version
   - The exact command you ran
   - The complete error message
   - Your SSH config (remove sensitive information)
