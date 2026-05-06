# Security Policy

## Supported Versions

We release patches for security vulnerabilities in the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of sshp seriously. If you discover a security vulnerability, please follow these steps:

### 1. Do Not Disclose Publicly

Please do not open a public GitHub issue for security vulnerabilities. This helps protect users while we work on a fix.

### 2. Report Privately

Please report security vulnerabilities by emailing: **kaushik@startuphakk.com**

Include the following information in your report:

- Description of the vulnerability
- Steps to reproduce the issue
- Potential impact
- Suggested fix (if you have one)

### 3. Response Timeline

- **Initial Response**: Within 48 hours, we will acknowledge receipt of your report
- **Status Update**: Within 7 days, we will provide an assessment and expected timeline
- **Resolution**: We aim to release a fix within 30 days for critical vulnerabilities

### 4. Disclosure Policy

- We will coordinate with you on the disclosure timeline
- Once a fix is released, we will publicly acknowledge your responsible disclosure (if you wish)
- We request that you do not disclose the vulnerability publicly until we have released a fix

## Security Considerations

### What sshp Does

- **Reads SSH config files**: The tool parses `~/.ssh/config` to extract host entries
- **Does NOT store credentials**: No passwords, private keys, or sensitive data are stored or transmitted
- **Does NOT modify SSH config**: The tool only reads the SSH config file; it never writes to it
- **Executes SSH command**: When a host is selected, the tool executes the standard `ssh` command with the selected hostname

### What You Should Know

1. **SSH Config Access**: sshp requires read access to your SSH config file (`~/.ssh/config`). Ensure this file has appropriate permissions (typically `600` or `644`).

2. **Terminal Access**: The tool uses terminal raw mode for interactive selection. It does not log keystrokes or capture sensitive input beyond host selection.

3. **Command Execution**: The tool executes `ssh <hostname>` using the system's SSH client. It inherits your system's SSH configuration and security settings.

4. **No Network Access**: sshp itself does not make network connections. All SSH connections are handled by your system's SSH client.

### Best Practices

1. **Verify Downloads**: Always download sshp from official sources (GitHub releases or crates.io)
2. **Check Checksums**: Verify SHA256 checksums for downloaded binaries
3. **Build from Source**: For maximum security, build from source after reviewing the code
4. **Keep Updated**: Update to the latest version to receive security patches
5. **Review SSH Config**: Regularly audit your SSH config file for unauthorized entries

## Security Features

- **Wildcard Exclusion**: Automatically excludes wildcard hosts (e.g., `Host *`) to prevent unintended connections
- **No Panic in Library Code**: Error handling uses `Result` types instead of panicking
- **Minimal Dependencies**: Only depends on crossterm for terminal handling
- **No Data Collection**: No telemetry, analytics, or data collection

## Security Updates

Security updates will be released as patch versions and announced via:

- GitHub Security Advisories
- Release notes in CHANGELOG.md
- GitHub Releases page

Subscribe to GitHub notifications for this repository to stay informed about security updates.

## Acknowledgments

We appreciate the security research community and will acknowledge researchers who responsibly disclose vulnerabilities (unless they prefer to remain anonymous).
