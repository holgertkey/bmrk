# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
- CLI bookmark commands: `bm -bm add/remove/list`
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
