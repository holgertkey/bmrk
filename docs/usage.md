# Basic Usage

This guide covers the fundamental operations in bmrk.

## Launching bmrk

### From Current Directory

```bash
bm
```

Opens the TUI showing the current directory tree.

### From Specific Directory

```bash
bm /path/to/directory
```

Opens the TUI with the specified directory as root.

### Direct Navigation (No TUI)

```bash
bm myproject
```

If `myproject` is a saved bookmark, navigates there immediately without opening the TUI.

## Understanding the Interface

bmrk runs inline in 8 rows — it does not take over the terminal screen.

```
 /home/user/projects              jk:nav  l:expand  /:search  ':bmarks  q:exit
 ▼ projects/
   ► src/
   ► tests/
   ► docs/
```

- Header row: current root path + key hints
- Tree rows: directory tree with expand/collapse indicators
- `►` collapsed directory (has subdirectories)
- `▼` expanded directory
- `⚠` directory that could not be read

## Basic Navigation

### Moving the Cursor

```
j or ↓        Move down one item
k or ↑        Move up one item
```

### Expanding and Collapsing

```
l or →        Expand directory (show subdirectories)
h or ←        Collapse expanded directory; if already collapsed, go to parent
```

### Changing Root

```
Enter         Change root to selected directory
u             Go to parent directory (change root)
Backspace     Go back (undo last navigation — pops history)
```

When you press `Enter` on a directory, it becomes the new root of the tree.
`Backspace` undoes any previous navigation (Enter, bookmark jump, disk change, or `u`). History holds up to 50 entries.

### Example Navigation Flow

```bash
# Start from home
bm ~

# Navigate to a subdirectory:
# j/k to move, l to expand, Enter to enter
# u to go up to parent, Backspace to go back
```

## Exiting

```
q             Exit and cd to selected directory (with bm wrapper)
Esc           Exit without directory change
```

When using the `bm` shell wrapper, pressing `q` prints the selected path to stdout and the
wrapper calls `cd`. Without the wrapper, the path is printed but no `cd` occurs.

## Searching

Search operates on **folder names only** — file names and full paths are not searched.

### Entering Search Mode

```
/             Enter search mode
```

### Normal Search

Type your query and press `Enter`. Results appear inline, replacing the tree body.

```
/config      Enter search mode
config       Type query — finds all folders whose name contains "config"
Enter        Execute search
```

Matching is case-insensitive. Results are capped at 500 entries.

### Fuzzy Search

Start the query with `/` to activate fuzzy matching:

```
/             Enter search mode
/cnfg         Fuzzy query — finds "config", "configure", etc.
Enter         Execute search
```

Results are ranked by relevance score.

### Navigating Results

```
j/k or ↓/↑    Navigate through results
Enter         Jump to selected result in tree
q             Exit and cd to selected result's directory
Tab           Switch focus between tree and results
Esc           Close search results
```

## Bookmarks

### Creating Bookmarks

```
m             Enter bookmark creation mode
myproject     Type bookmark name
Enter         Save bookmark
Esc           Cancel
```

Bookmarks save directories only. If the cursor is on a file, the parent directory is saved.

### Using Bookmarks Inside bmrk

```
'             Open bookmark selection (apostrophe/tick)
```

Two modes:

**Navigation mode** (default):
```
j/k           Navigate bookmarks
Enter         Jump to selected bookmark
q             Exit and cd to selected bookmark
d             Mark for deletion (press twice to confirm)
Tab           Switch to filter mode
Esc           Close
```

**Filter mode**:
```
type text     Filter bookmarks by name or path
Tab           Switch back to navigation mode
Enter         Jump to selected bookmark
Esc           Close
```

### Using Bookmarks from the Command Line

```bash
# Jump to bookmark (navigates directly, no TUI)
bm myproject

# List bookmarks
bm -l

# Add bookmark for current directory
bm -a work

# Add bookmark for a specific path
bm -a work /path/to/work

# Remove bookmark
bm -d work
```

## Disk Selection

Press `d` in tree mode to open the disk/drive panel.

```
d             Open disk selection panel
j/k           Navigate disks
Enter         Switch to selected disk root
Esc           Close without navigating
```

The panel shows each disk with its mount point, filesystem type, free space, and total size.

**Note**: `d` is context-sensitive — in tree mode it opens the disk panel; inside bookmark selection it marks the selected bookmark for deletion.

## Mouse Support

### Clicking

```
Click         Select item
Double-click  Expand/collapse directory
```

### Scrolling

```
Scroll wheel  Navigate tree
```

## Tips for Efficient Usage

### Tip 1: Stay in Keyboard Mode

Everything is reachable without a mouse:

- Navigate with `hjkl` (Vim-style)
- Search with `/`
- Bookmark with `m` and `'`

### Tip 2: Use Bookmarks for Projects

Create bookmarks for frequently-accessed directories:

```bash
bm ~/projects/webapp
# Press 'm', name it 'webapp'

# Later:
bm webapp   # Instant navigation
```

### Tip 3: Search Folders Only

The search feature searches the entire tree including collapsed directories:

```
/config       Finds all folders with "config" in the name
//.git        Fuzzy search for ".git" directories
```

## Common Workflows

### Quick Navigation

```bash
bm            # Open from anywhere
'             # Open bookmarks
j/j           # Select bookmark
Enter         # Jump to location
q             # Exit and cd there
```

### Finding a Nested Directory

```bash
bm            # Open from anywhere
/config       # Search for "config" folders
j/k           # Navigate to the one you want
q             # Exit and cd there
```

## Next Steps

- [Key Bindings](./keybindings.md) — complete keybinding reference
- [Features](./features.md) — detailed feature documentation
- [Configuration](./configuration.md) — customize bmrk
