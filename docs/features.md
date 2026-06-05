# Features

Complete overview of all dtree features.

## Core Features

### Compact Inline Mode

The default way to launch dtree: an 8-row inline view in the terminal stream, similar to `fzf`.
No alternate screen, no fullscreen takeover — your scrollback history is preserved and the
terminal returns to its exact state on exit.

**Key bindings**: `q`/`Enter` (exit and cd), `Esc` (exit without cd), `s` (switch to fullscreen)

**Features**:
- Occupies only 8 rows with a header (current path + hints) and tree body
- Full navigation: expand/collapse, search, bookmarks — all without switching modes
- Search results appear inline, replacing the tree body; `Esc` returns to tree
- Pressing `s` transitions seamlessly to fullscreen file-viewer mode
- Shell wrapper captures selected path and calls `cd` on your behalf
- Full mouse support (click to select, scroll to navigate)

### Bookmarks System

Save favorite directories and jump to them instantly from anywhere on the command line.

**Key bindings**: `m` (create), `'` (select menu), `bm myproject` (CLI jump)

**Features**:
- Multi-character names (e.g., `webapp-backend`)
- Interactive creation with visual feedback and existing bookmarks list
- Dual-mode selection (navigation + filter — Tab to switch)
- CLI management (`bm -l`, `bm -a <name>`, `bm -d <name>`)
- Direct navigation from command line — no TUI required
- Persistent storage in JSON
- Safe two-phase deletion

[Learn more →](./features/bookmarks.md)

### Disk/Drive Selection

Browse and switch between all available drives and mount points.

**Key bindings**: `d` (open panel)

**Features**:
- Lists all drives (Windows: `C:\`, `D:\`) and mount points (Linux/macOS: `/`, `/home`, `/mnt/usb`)
- Shows filesystem type, free space, total capacity, and volume label
- Pre-selects the disk containing the current root path
- Navigate with `j`/`k`, press `Enter` to switch to selected disk root
- Press `Esc` to close without navigating

**Note**: `d` is context-sensitive — in tree mode it opens the disk panel; inside the bookmark
selection panel it marks the selected bookmark for deletion.

### Interactive Tree Navigation

Visual directory tree with vim-style navigation.

**Key bindings**: `j`/`k` (up/down), `l`/`h` (expand/collapse), `Enter` (change root), `u` (parent)

**Features**:
- Lazy loading of directory contents
- Error tracking for inaccessible directories (⚠ indicator)
- Configurable hidden file visibility
- Symlink support with cycle detection

[Learn more →](./features/navigation.md)

### File Preview and Viewing

Split-panel file preview with syntax highlighting.

**Key bindings**: `s` (toggle preview), `v` (fullscreen), `Ctrl+j`/`k` (scroll)

**Features**:
- Syntax highlighting for 100+ languages
- Configurable themes (dark, light, solarized, etc.)
- Line numbers in fullscreen mode (`l` to toggle)
- Binary file detection with hex editor integration
- HEAD/TAIL modes for large files (>10K lines)
- File search within content (`/` in fullscreen)

[Learn more →](./features/file-viewing.md)

### Search Functionality

Fast, asynchronous search across the entire tree.

**Key bindings**: `/` (enter search), `Tab` (focus), `Enter` (jump to result)

**Features**:
- Two-phase search: quick (loaded nodes) + deep (background)
- Fuzzy matching (start query with `/`)
- Relevance scoring and ranking
- Non-blocking UI during search
- Live progress updates
- Search scope: directories only (tree mode) or files+dirs (file viewer mode)

[Learn more →](./features/search.md)

### Bookmarks System

Save and quickly jump to favorite directories.

**Key bindings**: `m` (create), `'` (select), `bm myproject` (CLI jump)

**Features**:
- Multi-character names (e.g., `webapp-backend`)
- Interactive creation with visual feedback
- Dual-mode selection (navigation + filter)
- CLI management (`bm -l`, `bm -a <name>`, `bm -d <name>`)
- Direct navigation from command line
- Persistent storage in JSON
- Safe two-phase deletion

[Learn more →](./features/bookmarks.md)

### Directory Size Display

Asynchronous directory and file metadata badge on the `z` key.

**Key bindings**: `z` (toggle display)

**Badge format**: `myfolder  [ 5.9M, 12f 3d, drwxr-xr-x, 24.11.2025 11:28 ]`
- Size (with `>` prefix if calculation was interrupted)
- File and subdirectory counts (recursive)
- Permissions (`drwxr-xr-x` on Unix, `rw`/`ro` on Windows)
- Last-modified timestamp in `DD.MM.YYYY HH:MM` local time
- Files in split-view show `[ size, permissions, mtime ]`

**Features**:
- Async background calculation — UI stays responsive while sizes compute
- Safety limits (5s timeout, 10K files max per directory)
- Visual indicators: "calc." (in progress), ">" prefix (partial result)
- Format: K/M/G/T (kilobytes to terabytes)
- Individual file sizes in file viewer mode
- Result caching per session

[Learn more →](./features/sizes.md)

## Additional Features

### File Type Icons

Visual file type identification with icons.

**Configuration**: `show_icons = true` in config.toml

**Features**:
- Nerd Fonts support (1000+ icons)
- Language-specific icons (Rust, Python, JS, etc.)
- Special icons for config files
- Directory-specific icons (.git, node_modules, etc.)
- Emoji fallback if Nerd Fonts unavailable

**Requirements**:
- Nerd Font installed and configured in terminal

### Syntax Highlighting

Code preview with syntax highlighting.

**Configuration**: `enable_syntax_highlighting = true` in config.toml

**Features**:
- 100+ languages supported
- Multiple themes (base16-ocean, GitHub, Solarized, etc.)
- Lazy loading for performance
- Works in split view and fullscreen mode
- Can be disabled via config

### Binary File Support

Graceful handling of binary files.

**Features**:
- Automatic NULL byte detection
- Informational message instead of raw content
- File type, size, and permissions display
- Hex editor integration (press `e`)
- Configurable hex editor (`hex_editor = "hexyl"`)
- Supported types: executables, images, archives, videos, PDFs, databases

### External Program Integration

Open files and directories in external programs.

**Key bindings**: `e` (editor), `o` (file manager)

**Features**:
- Configurable editor (default: nano)
- Configurable file manager (default: mc)
- Configurable hex editor (default: hexyl)
- Pre-launch validation (checks if program exists)
- Proper terminal handling via /dev/tty
- Path escaping for spaces and special characters

### Clipboard Integration

Copy file and directory paths to clipboard.

**Key bindings**: `c` (copy path)

**Features**:
- Works with files and directories
- System clipboard integration
- Requires xclip on Linux (usually pre-installed)

### Mouse Support

Full mouse interaction support.

**Features**:
- Click to select items
- Double-click to expand/collapse directories
- Scroll to navigate tree or preview
- Drag vertical divider to resize panels
- Drag horizontal divider to resize bottom panel
- Shift+Mouse for text selection in fullscreen

### Configuration System

Extensive customization via TOML config.

**File**: `~/.config/dtree/config.toml`

**Features**:
- Auto-creation on first run
- Extensive comments in default config
- Color customization (names, hex, indexed)
- Layout settings (split position)
- Behavior settings (limits, defaults)
- External program configuration
- Theme selection

[Learn more →](./configuration.md)

### Bash Integration

Seamless shell workflow integration.

**Features**:
- `dt` wrapper function for automatic cd
- Direct navigation: `dt /path` or `dt bookmark`
- Return to previous directory: `dt -`
- Bookmark management from CLI: `bm -l`, `bm -a`, `bm -d`
- Clean separation of TUI (stderr) and output (stdout)

[Learn more →](./bash-integration.md)

## Fullscreen File Viewer Features

When viewing a file with `v`:

### Navigation Modes

- **Normal scrolling**: `j`/`k` (line), `Page Up`/`Down` (page)
- **File jumping**: `Ctrl+j`/`k` (next/previous file in directory)
- **HEAD mode**: First 10,000 lines (press `Home`)
- **TAIL mode**: Last 10,000 lines (press `End`)

### Display Options

- **Line numbers**: Toggle with `l` key
- **Syntax highlighting**: Automatic based on file extension
- **Word wrap**: Lines truncated to terminal width

### File Search

- **Search mode**: Press `/` to search within file
- **Navigation**: `n` (next match), `N` (previous)
- **Match counter**: Shows "Match 3/15" in title
- **Auto-scroll**: Centers matched line
- **Clear**: Press `Esc` to clear search

### Text Selection

#### Visual Selection Mode (Vim-style)

Interactive line-based text selection with keyboard or mouse.

**Key bindings**: `V` (enter mode), `j`/`k` (expand), `y` (copy), `Esc` (cancel)

**Features**:
- Vim-style line selection with keyboard navigation
- Select large blocks spanning multiple pages
- Auto-scroll when cursor reaches screen edge
- Visual feedback: gray background (selection), blue (cursor)
- Status bar shows selection size (e.g., "VISUAL: 25 lines")
- Copy entire selection to clipboard with `y`
- Bidirectional selection (up or down from start)
- Mouse wheel support for expanding selection

**Navigation in Visual Mode**:
- `j`/`k` or `↓`/`↑`: Expand selection
- `Page Up`/`Down`: Jump by page
- `Home`/`End`: Jump to file start/end
- `Mouse Scroll`: Move cursor with auto-scroll

**Use cases**:
- Copying log file sections for analysis
- Extracting large code blocks
- Selecting configuration sections
- Multi-page text selection

#### Mouse Selection (Traditional)

- **Select**: Shift+Click+Drag
- **Copy**: Ctrl+Shift+C (terminal shortcut)
- **Limited to visible area** (use Visual Mode for large selections)

### Binary File Handling

- **Detection**: Automatic (checks for NULL bytes)
- **Display**: Shows informational message
- **Viewing**: Press `e` to open in hex editor

## Search Features

### Search Modes

#### Normal Search

Substring matching (case-insensitive):

```
/config     Finds: "config.toml", "my-config", "configure.sh"
```

#### Fuzzy Search

Intelligent fuzzy matching with scoring:

```
/cfg        Finds: "config.toml", "configure.sh", "cargo.toml"
/src/mn     Finds: "src/main.rs", "src/menu.rs"
```

- Activated by starting query with `/`
- Results ranked by relevance score
- Match indices tracked for highlighting
- Displays scores like `[95]` next to results

### Search Scope

- **Tree mode** (default): Searches directories only
- **File viewer mode** (`s` enabled): Searches both files and directories

### Search Performance

- **Phase 1**: Instant search through already-loaded nodes
- **Phase 2**: Background search through entire tree
- **Non-blocking**: UI remains responsive
- **Progress**: Shows "Scanned: 1234 directories"
- **Cancellable**: Press `Esc` to stop background search
- **500-result cap**: Background thread cancelled once 500 results accumulate — prevents UI stall on broad queries (e.g. single-letter searches)
- **O(1) deduplication**: HashSet-based duplicate tracking — constant speed regardless of result count

### Search Results

- **Panel**: Fullscreen mode — bottom panel; compact mode — replaces tree body inline
- **Navigation**: `j`/`k` to navigate, `Enter` to jump
- **Focus**: `Tab` to switch between tree and results (fullscreen); auto-focus in compact
- **Highlighting**: Matches highlighted in fuzzy mode
- **Resizable**: Drag top border to adjust panel height (fullscreen only)

## Performance Features

### Zero-Copy Tree Operations

- Uses `Rc<RefCell<>>` for shared ownership
- Flat list stores references, not clones
- O(1) expand/collapse operations

### Lazy Loading

- Directories loaded only when expanded
- Files loaded only when file viewer enabled
- Syntax highlighting loaded once on first use

### Asynchronous Operations

- Search runs in background thread
- Size calculation runs in background threads
- UI remains responsive during long operations

### Limits and Timeouts

- `max_file_lines = 10000` - File preview limit
- 5-second timeout for size calculation
- 10,000 file limit for size calculation
- 100ms event polling for responsiveness

## Upcoming Features

See [Roadmap](../CLAUDE.md) for planned features:

- Navigation history (back/forward navigation)
- Advanced filtering (.gitignore support, custom patterns)
- Pre-defined color themes
- Performance monitoring and debug mode
- Custom keybindings
- Plugin system

## Feature Comparison

| Feature             | dtree | tree | ranger | nnn | lf  |
|---------------------|-------|------|--------|-----|-----|
| Tree View           | ✅    | ✅   | ✅     | ✅  | ✅  |
| File Preview        | ✅    | ❌   | ✅     | ⚠️  | ✅  |
| Syntax Highlighting | ✅    | ❌   | ✅     | ❌  | ❌  |
| Fuzzy Search        | ✅    | ❌   | ❌     | ✅  | ⚠️  |
| Async Search        | ✅    | ❌   | ❌     | ❌  | ❌  |
| Bookmarks           | ✅    | ❌   | ✅     | ✅  | ✅  |
| Directory Sizes     | ✅    | ✅   | ❌     | ⚠️  | ❌  |
| Mouse Support       | ✅    | ❌   | ✅     | ❌  | ❌  |
| File Operations     | ❌    | ❌   | ✅     | ✅  | ✅  |
| Shell Integration   | ✅    | ❌   | ✅     | ✅  | ✅  |

**Legend**: ✅ Full support, ⚠️ Partial support, ❌ Not supported

**Note**: dtree is focused on viewing and navigation, not file operations (copy, move, delete). Use integrated file manager (`o` key) for file operations.

## Next Steps

- [Navigation](./features/navigation.md) - Detailed navigation guide
- [File Viewing](./features/file-viewing.md) - File viewer features
- [Search](./features/search.md) - Search system details
- [Bookmarks](./features/bookmarks.md) - Bookmark system guide
- [Directory Sizes](./features/sizes.md) - Size calculation details
