# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-06-03

### Added
- **Dual-Engine GitHub & GitLab Support**: glab-tui now automatically detects if a project is hosted on GitHub or GitLab, translating TUI views and actions to `gh` or `glab` CLI commands under the hood.
- **CLI Configuration Options**: Added option flags `--repo <namespace>` (to override project context) and `--dir <path>` (to target a custom repository directory) on launch.
- **Columns Config Modal Overlay**: Replaced the sidebar panel with a centered columns checkbox toggler popup overlay, triggered by pressing `Tab` or `t`.
- **Hashed Multi-colored Labels**: Implemented individual label coloring based on a hashed color scheme in the Issues and Merge Requests tables, preserving fuzzy-search query highlights.
- **Runner Diagnostics Dashboard**: Integrated simulated performance statistics, utilizing gauges, utilization percentages, queue depths, and average queue wait times.

### Changed
- Expanded the Navigation sidebar pane to take full vertical height when columns config panel is hidden.
- Updated the Keyboard Shortcuts help menu to reflect the new `Tab`/`t` column toggle binding.
- Auto-formatted and cleaned up import structures across all code modules to fix compiler lint warnings.
