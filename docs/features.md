# Features

Complete overview of all bmrk features.

## Compact Inline Mode

bmrk runs inline in the terminal stream — it does not take over the screen.

**No alternate screen**: Your scroll history is preserved and the terminal returns
to its exact pre-launch state on exit, with no artifacts.

**8 rows**: One header row (current path + key hints) and up to 7 tree rows.

**Shell wrapper**: The `bm` wrapper captures the selected path from stdout and calls
`cd` on your behalf — the same approach used by `fzf`, `zoxide`, and `autojump`.

## Interactive Tree Navigation

Visual directory tree with vim-style navigation.

**Key bindings**: `j`/`k` (up/down), `l` (expand), `h` (collapse/parent), `Enter` (change root), `u` (parent), `Backspace` (go back)

**Features**:
- Lazy loading of directory contents (loaded only when expanded)
- Inaccessible directories shown with `⊘` icon in `error_color` (detected eagerly when parent expands, not only after trying to enter)
- Configurable hidden file visibility (`show_hidden` in config)
- Symlink support with cycle detection (`follow_symlinks` in config)
- Navigation history with undo (`u`) — up to 50 entries

## Search Functionality

Fast, asynchronous search across the entire directory tree.

**Key binding**: `/` (enter search), `Enter` (execute), `Tab` (focus toggle), `q` (exit+cd)

**Search scope**: Folder names only — not file names, not full paths, not content.

**Features**:
- Two-phase search: quick (already-loaded nodes, instant) + deep (background thread, full tree)
- Normal mode: case-insensitive substring matching
- Fuzzy mode: intelligent matching with scoring (activate by starting query with `/`)
- Non-blocking UI during background search
- Live progress: "Scanned: N directories"
- Results capped at 500 to prevent UI stall on broad queries

**Results navigation**: `j`/`k` to navigate, `Enter` to jump to result in tree, `q` to exit and cd there.

## Bookmarks System

Save favorite directories and jump to them instantly.

**Key bindings**: `m` (create), `'` (select menu), `bm myproject` (CLI jump)

**Features**:
- Multi-character names (e.g., `webapp-backend`)
- Interactive creation with existing bookmarks list shown while typing
- Dual-mode selection (navigation + filter — `Tab` to switch)
- CLI management: `bm -l`, `bm -a <name>`, `bm -d <name>`
- Direct navigation from command line — no TUI required
- Persistent storage in JSON
- Safe two-phase deletion (press `d` twice to confirm)
- `q` exits bmrk and cds to the selected bookmark

## Disk/Drive Selection

Browse and switch between all available drives and mount points.

**Key binding**: `d` (open panel)

**Features**:
- Lists all drives (Windows: `C:\`, `D:\`) and mount points (Linux/macOS: `/`, `/home`, `/mnt/usb`)
- Shows filesystem type, free space, and total capacity
- Pre-selects the disk containing the current root path
- Navigate with `j`/`k`, `Enter` to switch to selected disk root, `Esc` to close

**Note**: `d` is context-sensitive — in tree mode it opens the disk panel; inside bookmark selection it marks the selected bookmark for deletion.

## Mouse Support

Mouse works in all interactive panels — tree, bookmark selection, and disk selection.

**Features**:
- Click to select the item under the cursor
- Double-click to expand/collapse directories (tree), navigate to bookmark, or navigate to disk root
- Scroll wheel to navigate the active list
- **Minimal scroll**: the view only shifts when the selection leaves the visible area, so a
  single click followed by a double-click always lands on the same row

## Configuration System

TOML configuration file, auto-created on first run.

**File location**:
- Linux/macOS: `~/.config/bmrk/config.toml`
- Windows: `%APPDATA%\bmrk\config.toml`

**Sections**:
- `[appearance]` — theme, colors, icon style, max name length
- `[behavior]` — show hidden files, follow symlinks, mouse timing
- `[keybindings]` — customize keyboard shortcuts

See [Configuration](./configuration.md) for the full reference.

## Shell Integration

**Features**:
- `bm` wrapper function for automatic `cd`
- Direct navigation: `bm /path` or `bm bookmark`
- Return to previous directory: `bm -`
- Bookmark management from CLI: `bm -l`, `bm -a`, `bm -d`
- Clean separation of TUI output (stderr) and path output (stdout)

See [Installation](./installation.md) for wrapper setup instructions.
