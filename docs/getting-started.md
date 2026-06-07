# Getting Started

Get up and running with bmrk in five minutes.

## Quick Start

After installation, launch the interactive tree from the current directory:

```bash
bm
```

Navigate with `j`/`k` (or arrow keys), press `q` to exit and cd to the selected directory,
or `Esc` to exit without changing directory.

## First Launch

When you launch bmrk for the first time:

1. The TUI opens showing the current directory tree (8 rows inline)
2. A configuration file is auto-created at the config location
3. Use arrow keys or `j`/`k` to navigate
4. Press `Enter` to enter a directory (makes it the new root)
5. Press `q` to exit and cd there, or `Esc` to exit without navigating

## Basic Navigation

```
j or ↓        Move down
k or ↑        Move up
l or →        Expand directory
h or ←        Collapse directory; go to parent if already collapsed
Enter         Change root to selected directory
u             Go to parent directory
Backspace     Go back (undo last navigation)
q             Exit and cd to selected directory
Esc           Exit without changing directory
```

## Searching

```
/             Enter search mode
              Type your query (folder names only)
Enter         Execute search
j/k           Navigate results
Enter         Jump to selected result in tree
q             Exit and cd to selected result
Esc           Close search
```

Search matches folder names only — not file names, not full paths.

## Bookmarks

```
m             Create bookmark for current directory
'             Open bookmark selection menu (apostrophe)
```

Inside bookmark selection:

```
j/k           Navigate bookmarks
Enter         Jump to selected bookmark
q             Exit and cd to selected bookmark
Tab           Toggle filter mode
d d           Delete bookmark (press twice to confirm)
Esc           Close
```

## Your First Workflow

### 1. Navigate to a Project

```bash
bm              # Open from anywhere
# Navigate with j/k, expand with l
# Press Enter on your project directory
# Press q to exit and cd there
```

### 2. Create a Bookmark

```bash
bm ~/projects/webapp
# Press 'm' to create a bookmark
# Type a name: webapp
# Press Enter to save
# Press q to exit
```

### 3. Use the Bookmark

```bash
# From anywhere — no TUI needed:
bm webapp

# Or inside bm:
bm              # Open TUI
# Press ' (apostrophe)
# Navigate to 'webapp', press Enter or q
```

## Configuration File

Config is auto-created on first run:

- **Linux/macOS**: `~/.config/bmrk/config.toml`
- **Windows**: `%APPDATA%\bmrk\config.toml`

Delete it to reset to defaults — it will be recreated on next run.

## Getting Help

Run `bm -h` for the embedded help reference (all keys, config paths, CLI options).
