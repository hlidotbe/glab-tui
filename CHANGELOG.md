# Changelog

All notable changes to this project will be documented in this file.

## [0.2.1] - 2026-06-07

### Added
- **New MR creation from issue**: Branch selector with auto-create, slug-based source branch, auto-push before PR creation.
- **Reopen/close issues and MRs.**
- **Persistent offline caching** for all data tabs (issues, MRs, pipelines, runners, releases, todos, milestones).
- **1-minute auto-refresh** of the active tab.
- **Inline command logs** and a scrollable **Terminal tab** showing CLI command history.
- **Creation forms** for issues, MRs, and pipeline triggers.
- **Edit menus** with `$EDITOR` integration for descriptions and freeform fields.
- **Pipeline/JD job trace viewer** with scroll support and open-in-editor.
- **Self-updater** via `--update` / `-u` flag (GitHub releases).
- **Security audit** CI workflow (`cargo audit`).

### Fixed
- UI table overflow: main content pane now respects the terminal pane's reserved height.
- Windows: `NamedTempFile` handle locking — editor temp files use `into_temp_path()` to release the handle before spawning.
- Windows: removed `cmd /c` wrapper from editor spawn — Rust's command-line builder was double-escaping path quotes.
- GitHub mode: labels, milestones, description editing, and PR-from-issue creation.
- Fuzzy search: disabled fuzzy matching on all tabs except MRs; "Create New" option moved to top of selector.
- Self-updater: works correctly on both Linux and Windows.
- Various UI panics on empty lists, ellipsis padding, and rendering edge cases.

### Changed
- Refactored editor integration: extracted `Cli` / `UpdateCmd` helper structs for clean GitHub/GitLab CLI flag mapping.
- CI workflows now trigger only on `main` (dev branch triggers removed post-merge).

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
