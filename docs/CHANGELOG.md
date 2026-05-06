# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-04-30

### Added
- Interactive SSH host selection from `~/.ssh/config`
- Real-time search with substring matching (case-insensitive)
- Colorful terminal UI with blue selection highlight
- Search query highlighting in yellow
- List grows from bottom to top (fzf-style)
- Arrow key navigation (↑/↓) for browsing hosts
- Auto-connect to selected host on Enter
- Quit functionality with Esc or Ctrl+C
- Automatic exclusion of wildcard hosts (containing `*`)
- Backspace support for editing search queries
- Cross-platform terminal support via crossterm
- Zero-configuration setup - works with existing SSH config

[Unreleased]: https://github.com/kaushiktadhani/sshp/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/kaushiktadhani/sshp/releases/tag/v0.1.0
