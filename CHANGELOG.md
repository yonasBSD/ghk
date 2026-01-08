# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0] - 2026-01-08

### Added
- Initial release
- Core commands: `init`, `login`, `logout`, `create`, `push`, `pull`, `clone`, `status`
- Setup wizard with auto-install for Linux (apt/dnf/pacman/zypper), macOS (brew), Windows (winget)
- User management: `user list`, `user switch`
- Utility commands: `undo`, `history`, `open`, `diff`, `branch`
- Project helpers: `ignore` (gitignore templates), `license` (MIT/Apache/GPL/Unlicense)
- Settings: `config` command to view/edit preferences
- Shell completions: `completions bash/zsh/fish`
- Progress spinners for long operations
- Colored output with `--no-color` fallback
- Quiet mode with `-q/--quiet` flag
- First-run welcome message
- Offline detection before network operations
- SSH key detection in setup
- Command aliases: `save`→`push`, `sync`→`pull`, `download`→`clone`, `log`→`history`
