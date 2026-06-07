# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-06-07

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
- `go_to_parent`, `go_back`, `quit`, `exit` keybinding fields — all four previously hardcoded navigation and exit keys are now remappable in `[keybindings]`. Defaults match the prior behavior (`u`, `Backspace`, `q`, `Esc`). Each field accepts a list so multiple keys can be bound to the same action.
- `header_path_color` config field: color for the path, count, and mode labels in the header row (default: `"cyan"`).
- `header_hints_color` config field: color for the key hint text in the header row (default: `"darkgray"`). Both fields are supported in all six built-in theme presets.
- `icons` setting in `[appearance]` config: `"unicode"` (default, `▼`/`▶`) or `"ascii"` (`v`/`>`).
- **Add bookmark** menu: header now shows the path of the folder being bookmarked instead of the navigation root.
- **Add bookmark** menu: existing bookmark keys are listed on a single comma-separated line (wraps at `max_name_length` columns) instead of a multi-line list.
- **Add bookmark** menu: `Ctrl+j` / `Ctrl+k` scrolls the directory tree (same as mouse wheel), making it easy to pick a different target folder without leaving creation mode.
- Navigation history (`Backspace` key): each `Enter`, bookmark jump, disk selection, and `u` push the current root to a 50-entry history stack; `Backspace` pops and returns to the previous root. Failed navigations do not push to history.
- Tree view header shows item position counter `(current/total)` in the same style as the path.
- `bm.ps1` — PowerShell wrapper with correct stdout/stderr separation via temp file; supports `bm -` (return to previous directory) and passthrough for `--help`, `--version`, `-l`, `-a`, `-d`.
- `bm -` navigation in `bm.bat` (CMD wrapper) via `BMRK_PREV_DIR` environment variable.
- `-v` short flag as alias for `--version`.
- `-l / --list` flag to list all bookmarks.
- `-a / --add <name> [path]` flag to add a bookmark.
- `-d / --del <name>` flag to remove a bookmark.

### Changed
- Binary renamed from `dtree` to `bmrk`; shell wrapper is `bm`.
- Crate renamed from `dtree-tui` to `bmrk`.
- Config and bookmarks directory: `dtree/` → `bmrk/`.
- Config simplified: removed all file-viewer related fields.
- Header icon (`▼`/`v`) now uses `directory_color` instead of a hardcoded green.
- `max_name_length` default value changed from `30` to `80`.
- `u` goes to the parent directory (change root); `Backspace` goes back (undo last navigation).
- Leaf directories (no visible children) no longer show the expand arrow and cannot be expanded.
- `h` / `←` follows ranger-style behavior: collapses an expanded directory when on it; if already collapsed moves selection to the parent node and collapses it.
- `-bm` / `--bm` subcommand replaced with dedicated flags (`-l`, `-a`, `-d`).
- `bm.bat` rewritten: added passthrough for all non-`cd` flags, proper exit-code propagation, and `bm -` support.
- `docs/installation.md` rewritten for bmrk.
- README: Windows PowerShell installation instructions updated to use `bm.ps1`.

### Fixed
- **Add bookmark** menu: spaces are now ignored in the bookmark name input field.
- `bm -` did not work in `cmd.exe`: `cd /d` inside a parenthesized `if` block expanded `%BMRK_TMP%` at parse time; restructured with `goto` so each line is parsed at execution time.
- Mouse (scroll and click) did not work in the disk selection menu; mouse events now route correctly to disk mode.
- `q` in disk selection mode was ignored; now exits the TUI and changes the shell to the selected disk's root.
- `bm.bat`: directory navigation was silently doing nothing when called from PowerShell; resolved by providing the dedicated `bm.ps1` wrapper.
- Mouse wheel scrolling jumped multiple entries per tick; fixed by coalescing buffered scroll events so at most one scroll step is applied per render frame.
- Default `mouse_scroll_lines` reduced from `3` to `1` for natural one-item-per-tick navigation.

### Removed (relative to dtree 1.3.0)
- File viewer (split view and fullscreen mode).
- Syntax highlighting (`syntect`).
- Directory size display (`z` key).
- File icons (Nerd Font / emoji).
- Visual selection mode (clipboard copy).
- External editor / hex editor / file manager integration.
- Shell wrapper scripts (`dt.bat`, `install-*.sh`, etc.).
- `arboard`, `syntect`, `which`, `chrono`, `once_cell`, `unicode-truncate` dependencies.
- `border_color`, `main_border_color`, `panel_border_color`, `background_color`, `file_search_highlight_color` color fields — defined but never applied to any widget in the compact UI.

---

## Historical (dtree)

The following entries document the dtree history that bmrk is based on.

## [1.3.0] - 2026-05-01

### Added
- **Inline search in compact mode**: `/` search results shown inline, no fullscreen switch.
- **Disk/Drive Selection Panel** (`d` key): lists all drives with filesystem type and free/total space.

### Fixed
- `z` badge: ESC now cancels size calculation immediately (was blocking UI on large trees).
- Search hang on broad queries: O(n²) deduplication replaced with O(1) `HashSet`, 500-result cap added.

## [1.2.0] - 2026-04-28

### Added
- **Rich metadata badge on `z`**: size, file/dir counts, permissions, mtime.
- **Bookmarks in compact mode**: `m` and `'` work without switching to fullscreen.
- **Middle-truncation** for long filenames (`max_name_length` config option).

## [1.2.0] - 2026-04-15

### Added
- **Compact Inline Mode**: default launch in 8-row inline viewport (fzf-style).

## [1.1.0] - 2025-01-24

### Added
- Initial feature set: tree navigation, file preview, fuzzy search, bookmarks,
  directory sizes, fullscreen viewer, visual selection, binary file support,
  file icons, mouse support, bash/PowerShell integration, TOML configuration.

---

[Unreleased]: https://github.com/holgertkey/bmrk/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/holgertkey/bmrk/releases/tag/v0.1.0
