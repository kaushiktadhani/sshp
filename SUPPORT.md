# Support

Need help with sshp? This document provides resources and guidelines for getting support.

## Quick Links

- **Documentation**: See [docs/](docs/) folder for detailed guides
- **Troubleshooting**: Check [TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md) for common issues
- **Installation**: See [INSTALLATION.md](docs/INSTALLATION.md) for platform-specific instructions
- **Contributing**: See [CONTRIBUTING.md](docs/CONTRIBUTING.md) to contribute to the project

## Getting Help

### 1. Check Existing Resources

Before opening an issue, please check:

- [README.md](README.md) - Overview and quick start guide
- [Troubleshooting Guide](docs/TROUBLESHOOTING.md) - Common issues and solutions
- [Existing Issues](https://github.com/kaushiktadhani/sshp/issues) - Someone may have already reported your issue

### 2. GitHub Issues

For bug reports and feature requests, please use GitHub Issues:

**[Open an Issue](https://github.com/kaushiktadhani/sshp/issues/new/choose)**

We have templates for:
- 🐛 Bug reports
- ✨ Feature requests

Please use the appropriate template and provide as much detail as possible.

### 3. GitHub Discussions

For questions, ideas, and general discussion:

**[Start a Discussion](https://github.com/kaushiktadhani/sshp/discussions)**

Good topics for discussions:
- How-to questions
- Usage tips and tricks
- Sharing your experience with sshp
- Ideas that aren't quite feature requests yet

## Frequently Asked Questions

### General Questions

**Q: Does sshp work on Windows?**
A: Yes! sshp supports Windows, macOS, and Linux. See [INSTALLATION.md](docs/INSTALLATION.md) for platform-specific instructions.

**Q: Does sshp modify my SSH config file?**
A: No, sshp only reads your SSH config file. It never modifies it.

**Q: Can I use sshp as a library in my Rust project?**
A: Yes! See the [examples/](examples/) directory for usage examples.

**Q: Why don't I see all my hosts?**
A: sshp automatically excludes wildcard hosts (containing `*`). Only specific host entries are shown.

### Installation Issues

**Q: I get "command not found" after installation**
A: Make sure the installation directory is in your PATH. See [INSTALLATION.md](docs/INSTALLATION.md) for PATH setup instructions.

**Q: The binary won't run on macOS (security warning)**
A: This is a Gatekeeper warning. See the [macOS troubleshooting section](docs/TROUBLESHOOTING.md#macos-gatekeeper-warning).

### Usage Questions

**Q: How do I search for hosts?**
A: Just start typing when the selector is open. The list filters in real-time.

**Q: What if I have hundreds of hosts?**
A: sshp handles large config files efficiently. Use the search feature to quickly narrow down the list.

**Q: Can I cancel the selection?**
A: Yes, press `Esc` or `Ctrl+C` to exit without connecting.

## Reporting Bugs

When reporting bugs, please include:

1. **sshp version**: Run `sshp --version` (or check your installation method)
2. **Operating system**: Including version (e.g., "macOS 14.0", "Ubuntu 22.04")
3. **Expected behavior**: What you expected to happen
4. **Actual behavior**: What actually happened
5. **Steps to reproduce**: Detailed steps to reproduce the issue
6. **Error messages**: Any error messages or screenshots

Example bug report:
```
sshp version: 0.1.0
OS: Ubuntu 22.04
Issue: Hosts with special characters in names don't display correctly

Steps to reproduce:
1. Add a host named "my-server@prod" to ~/.ssh/config
2. Run sshp
3. Observe the display

Expected: Host name should be shown as "my-server@prod"
Actual: Host name shows as "my-server" (truncated)
```

## Feature Requests

When requesting features:

1. **Check the roadmap**: See [ROADMAP.md](ROADMAP.md) - it might already be planned
2. **Describe the use case**: Why is this feature needed?
3. **Provide examples**: Show how it would work
4. **Consider alternatives**: Have you tried other approaches?

## Security Issues

**Do not open public issues for security vulnerabilities.**

Please report security issues privately by following the [Security Policy](SECURITY.md).

## Community Guidelines

Please be respectful and constructive in all interactions. We follow the Contributor Covenant code of conduct.

## Commercial Support

For commercial support, consulting, or custom development, please contact:

**Email**: kaushik@startuphakk.com

## Contributing

Want to help improve sshp? See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines on:

- Setting up your development environment
- Running tests
- Submitting pull requests
- Code style guidelines

## Response Times

This is an open source project maintained by volunteers. Response times may vary:

- Critical security issues: 48 hours
- Bugs: 3-7 days
- Feature requests: 1-2 weeks
- General questions: Best effort

Thank you for using sshp!
