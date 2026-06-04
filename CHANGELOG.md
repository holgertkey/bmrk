# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- CI/CD pipeline with GitHub Actions
- Automated binary releases for Linux, macOS, and Windows
- Increased test coverage (target: 80%+)
- Performance benchmarks

## [1.3.0] - 2026-05-01

### Added
- **Search in compact mode**: `/` search now works inline within the 8-row compact viewport —
  no fullscreen switch. Results replace the tree body with a header showing count and
  navigation hints. `Enter` jumps to the selected result and returns focus to the tree;
  `Esc` closes results. The old `s` (file viewer) toggle still transitions to fullscreen.
- **Disk/Drive Selection Panel**: Press `d` in tree mode to open a scrollable panel listing all
  available drives and mount points
  - Shows mount point path (e.g., `C:\`, `/`, `/home`)
  - Filesystem type (NTFS, ext4, btrfs, ...)
  - Free space and total space in human-readable format
  - Volume label (if different from mount/drive path)
  - Pre-selects the disk matching the current root path
  - Press `Enter` to navigate to the selected disk/mount root
  - Press `Esc` to close without navigating
  - Uses `sysinfo` crate for cross-platform disk enumeration

### Fixed
- **`z` badge: ESC now cancels size calculation immediately** on large directories.
  Previously `cancel()` called `join()` on the worker thread, blocking the UI until the
  recursive traversal finished (could take several seconds on large trees). Fixed:
  - Added `Arc<AtomicBool>` cancel flag checked at the entry of every recursive call and
    every 100 entries within a directory — the worker exits within milliseconds of ESC.
  - `cancel()` now drops the worker `JoinHandle` without joining; the thread terminates
    on its own once it notices the flag or finds the task channel closed.
  - Permissions and mtime are cached immediately (fast stat) when `calculate_async` is
    called, so the badge shows `[ calc., rw, 01.01.2025 12:00 ]` while the size is
    computing and `[ rw, 01.01.2025 12:00 ]` after cancellation — metadata is never lost.
- **Search hang on broad queries**: Single-character or very common queries (e.g., `r`, `e`)
  matched thousands of entries, causing the UI to freeze. Root cause: O(n²) deduplication
  (`Vec::iter().any()` called for every incoming result from the background thread). Fixed:
  - **O(1) deduplication** via `HashSet<PathBuf>` — constant-time duplicate check regardless of result count
  - **500-result cap** — background thread cancelled once 500 results accumulate, preventing unbounded growth
  - **200-message batch limit** per `poll_results` call — keeps the main thread responsive during active scanning
  - **SkimMatcherV2 created once** per search thread (not once per file) in fuzzy mode

## [1.2.0+007] - 2026-04-28

### Changed
- **Bookmarks work in compact mode**: `m` (create bookmark) and `'` (open bookmarks) now work
  directly in the compact inline view without switching to fullscreen. Creation mode shows the
  existing bookmarks list in the body with an input bar at the bottom. Selection mode replaces the
  tree with the bookmark list and shows context-aware header hints (Tab switches to filter mode).

## [1.2.0] - 2026-04-28

### Added
- **Rich metadata badge on `z`**: The size badge now shows full directory/file info:
  `myfolder  [ 5.9M, 12f 3d, drwxr-xr-x, 24.11.2025 11:28 ]`
  - Size (with `>` prefix if partial)
  - File and subdirectory counts (recursive, dirs only)
  - Permissions (`drwxr-xr-x` on Unix, `rw`/`ro` on Windows)
  - Last-modified timestamp in `DD.MM.YYYY HH:MM` local time
  - Files in split-view show size + permissions + mtime
- **Middle-truncation for long filenames**: Names exceeding `max_name_length` characters are
  shortened by replacing the middle with `...` (e.g. `very_long_project_name.rs` →
  `very_long_proj...name.rs`). Configurable via `max_name_length` in `[appearance]` (default: 30,
  set to 0 to disable).

## [1.2.0] - 2026-04-15

### Added
- **Compact Inline Navigation Mode**: Default `dt` launch now opens in a compact 8-row inline
  viewport instead of taking over the full terminal screen
  - Renders directly in the terminal stream (similar to fzf), leaving scrollback intact
  - Terminal returns to its exact pre-launch state on exit — no visual artifacts
  - `q` / `Enter` exits and navigates to the selected directory (behavior unchanged)
  - `Esc` exits without changing directory (behavior unchanged)
  - `s` transitions seamlessly from compact mode to fullscreen file-viewer mode
  - Full mouse support in compact mode (click to select, scroll to navigate)
  - Pressing `/` + Enter in compact mode switches to fullscreen with search results panel
  - `-v` flag and file-viewer mode (`s`) continue to use fullscreen as before

### Fixed
- Mouse click in compact mode was selecting the item one row above the clicked item
  (off-by-one caused by block border offset not applying in borderless compact layout)

## [1.1.0] - 2025-01-24

### Added
- **Interactive Tree Navigation**: Visual directory tree with expand/collapse functionality
- **File Preview**: Syntax-highlighted preview for 100+ programming languages
- **Fuzzy Search**: Asynchronous two-phase search with intelligent fuzzy matching
  - Normal mode: Substring matching
  - Fuzzy mode: SkimMatcherV2 algorithm with scoring and character highlighting
- **Bookmarks System**: Save and quickly jump to favorite directories
  - Persistent storage in `~/.config/dtree/bookmarks.json`
  - Interactive creation and selection modes
  - Deletion with confirmation
- **Directory Size Calculation**: Asynchronous size calculation with visual indicators
  - Background threads per directory
  - Safety limits (5s timeout, 10K files)
  - Formatted output (K/M/G/T)
- **Fullscreen File Viewer**: Dedicated viewer with advanced features
  - Syntax highlighting with configurable themes
  - Line numbers toggle (`l`)
  - Line wrapping toggle (`w`)
  - HEAD/TAIL mode switching (Home/End keys)
  - File search within content (`/`, `n`/`N` navigation)
  - Jump between files in directory (Ctrl+j/Ctrl+k)
- **Visual Selection Mode**: Vim-style line selection (V key)
  - Keyboard selection (j/k, Page Up/Down, Home/End)
  - Mouse selection support with auto-scroll
  - Copy to clipboard (`y`)
  - Visual feedback with highlighting
- **Binary File Support**: Automatic detection and handling
  - Hex editor integration
  - Informational display for binary files
- **Customizable Configuration**: TOML-based configuration system
  - Theme presets: default, gruvbox, nord, tokyonight, dracula, obsidian
  - Custom color schemes (names, hex, indexed)
  - Configurable keybindings
  - External editor/hex editor/file manager integration
  - Auto-creation of config on first run
- **File Icons**: Support for Nerd Fonts and emoji fallbacks
  - 100+ programming languages
  - Configuration files (Cargo.toml, package.json, etc.)
  - Special directories (.git, node_modules, etc.)
  - Media files
- **Mouse Support**: Full mouse interaction
  - Click to select/expand/collapse
  - Double-click to navigate
  - Drag to resize split view
  - Scroll in file viewer and tree
- **Bash Integration**: Seamless shell integration with `dt` wrapper
  - Direct navigation: `dt /path`
  - Bookmark jumping: `dt myproject`
  - Previous directory: `dt -`
  - File viewing: `dt -v file.txt`
- **Comprehensive Help System**: Interactive help screen (`i` key)

### Performance
- **Zero-Copy Tree Operations**: Uses `Rc<RefCell<>>` for efficient tree manipulation
- **Lazy Loading**: Directories and files loaded only when needed
- **Async Operations**: Non-blocking search and size calculation
- **Optimized Binary**: 2.5 MB with aggressive size optimization
  - LTO enabled
  - Symbol stripping
  - Single codegen unit

### Robustness
- **Comprehensive Terminal Cleanup**: Multi-stage cleanup prevents artifacts
  - Explicit disabling of all 6 mouse tracking modes
  - Double event draining (before and after screen transition)
  - Proper timing delays for terminal processing
  - Handles terminal resize in split view mode without artifacts
- **Graceful Error Handling**: No `std::process::exit()` calls
  - All errors propagate through `anyhow::Result`
  - Single exit point in main()
  - Detailed, user-friendly error messages
  - Config parse errors with fix instructions
- **Panic Recovery**: Panic hook ensures terminal restoration
  - Installed during terminal setup
  - Guarantees cleanup even on crash
- **Event::Resize Handling**: Prevents event accumulation and leakage

### Documentation
- Complete README with installation, usage, and features
- Architecture documentation in `docs/`
- Troubleshooting guide with common issues
- Configuration reference
- Keybindings cheat sheet
- CLI options documentation

### Technical Details
- **Language**: Rust (edition 2021)
- **TUI Framework**: ratatui 0.28
- **Terminal Backend**: crossterm 0.28
- **Lines of Code**: ~6,500
- **Test Coverage**: 33 tests (unit + integration)
- **Dependencies**: Minimal, all up-to-date

### Changed
- **Project Renamed** to `dtree-tui` for crates.io publication
- **Windows Support** added with PowerShell and cmd.exe integration
- **Installation Scripts** for automated setup on all platforms
- **Uninstall Scripts** for clean removal

### Known Limitations
- No file operations (copy, move, delete) - use file manager integration (`o` key)
- No plugin system yet

---

For detailed roadmap and future plans, see documentation in `docs/` directory.

[Unreleased]: https://github.com/holgertkey/dtree/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/holgertkey/dtree/releases/tag/v1.1.0
