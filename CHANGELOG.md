# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- `bm.ps1` — PowerShell wrapper with correct stdout/stderr separation via temp file;
  supports `bm -` (return to previous directory) and passthrough for `--help`, `--version`,
  `-l`, `-a`, `-d`
- `bm -` navigation in `bm.bat` (CMD wrapper) via `BMRK_PREV_DIR` environment variable
- `-v` short flag as alias for `--version`
- `-l / --list` flag to list all bookmarks
- `-a / --add <name> [path]` flag to add a bookmark
- `-d / --del <name>` flag to remove a bookmark

### Fixed
- Mouse (scroll and click) did not work in the disk selection menu (`d`); mouse events now
  route correctly to disk mode — scroll moves the selection, single click highlights a disk,
  double click navigates to it
- `bm.bat`: directory navigation (`bm myproject`, `bm some\path`) was silently doing nothing
  due to `cd` running in a subprocess when called from PowerShell; resolved by providing the
  dedicated `bm.ps1` PowerShell wrapper instead
- Mouse wheel scrolling jumped multiple entries per tick instead of scrolling sequentially;
  fixed by coalescing buffered scroll events in the event drain loop so at most one scroll
  step is applied per render frame
- Default `mouse_scroll_lines` reduced from `3` to `1` for natural one-item-per-tick navigation

### Changed
- `-bm` / `--bm` subcommand replaced with dedicated flags (`-l`, `-a`, `-d`); bookmark
  management commands are now first-class flags instead of positional sub-arguments
- `bm.bat` rewritten: added passthrough handling for flags that should not trigger `cd`
  (`-h`, `--help`, `-v`, `--version`, `-l`, `--list`, `-a`, `--add`, `-d`, `--del`),
  proper exit-code propagation, and `bm -` support
- `docs/installation.md` rewritten for bmrk (previously contained dtree content)
- README: Windows PowerShell installation instructions updated to use `bm.ps1`

## [0.1.0] - 2026-06-04

Initial release of **bmrk** — forked from [dtree](https://github.com/holgertkey/dtree) v1.3.0
and stripped down to a focused bookmark manager and directory navigator.

### Added
- Compact inline TUI (8 rows, no fullscreen takeover)
- Directory tree navigation with expand/collapse
- Bookmark management: create (`m`), select (`'`), delete, filter
- Two-phase search with fuzzy mode (`/query`)
- Disk/drive selection panel (`d`)
- Mouse support: click, double-click, scroll
- CLI bookmark commands: `bm -l`, `bm -a <name>`, `bm -d <name>`
- Direct path/bookmark resolution: `bm myproject`
- TOML configuration with theme presets and custom colors

### Removed (relative to dtree 1.3.0)
- File viewer (split view and fullscreen mode)
- Syntax highlighting (`syntect`)
- Directory size display (`z` key)
- File icons (Nerd Font / emoji)
- Visual selection mode (clipboard copy)
- External editor / hex editor / file manager integration
- Shell wrapper scripts (`dt.bat`, `install-*.sh`, etc.)
- `arboard`, `syntect`, `which`, `chrono`, `once_cell`, `unicode-truncate` dependencies

### Changed
- Binary renamed from `dtree` to `bm`
- Crate renamed from `dtree-tui` to `bmrk`
- Config and bookmarks directory: `dtree/` → `bmrk/`
- Config simplified: removed all file-viewer related fields

---

## Historical (dtree)

The following entries document the dtree history that bmrk is based on.

## [1.3.0] - 2026-05-01

### Added
- **Inline search in compact mode**: `/` search results shown inline, no fullscreen switch
- **Disk/Drive Selection Panel** (`d` key): lists all drives with filesystem type and free/total space

### Fixed
- `z` badge: ESC now cancels size calculation immediately (was blocking UI on large trees)
- Search hang on broad queries: O(n²) deduplication replaced with O(1) `HashSet`, 500-result cap added

## [1.2.0] - 2026-04-28

### Added
- **Rich metadata badge on `z`**: size, file/dir counts, permissions, mtime
- **Bookmarks in compact mode**: `m` and `'` work without switching to fullscreen
- **Middle-truncation** for long filenames (`max_name_length` config option)

## [1.2.0] - 2026-04-15

### Added
- **Compact Inline Mode**: default launch in 8-row inline viewport (fzf-style)

## [1.1.0] - 2025-01-24

### Added
- Initial feature set: tree navigation, file preview, fuzzy search, bookmarks,
  directory sizes, fullscreen viewer, visual selection, binary file support,
  file icons, mouse support, bash/PowerShell integration, TOML configuration

---

[Unreleased]: https://github.com/holgertkey/bmrk/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/holgertkey/bmrk/releases/tag/v0.1.0
