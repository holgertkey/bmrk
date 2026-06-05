# bmrk Cheat Sheet

Quick reference for `bm` — Bookmark Manager and Directory Navigator.  
Binary: `bmrk` · Wrapper: `bm` (handles `cd` automatically).

---

## Installation

```bash
cargo install bmrk            # installs bmrk binary

# Linux/macOS — add wrapper to ~/.bashrc or ~/.zshrc:
bm() { local r; r=$(bmrk "$@"); [ -d "$r" ] && cd "$r" || { [ -n "$r" ] && echo "$r"; }; }

# Windows CMD — place bm.bat + bmrk.exe in the same PATH directory.
# Windows PowerShell — add to $PROFILE:
# function bm { $r = & bmrk @args; if ($r -and (Test-Path $r -PathType Container)) { Set-Location $r } elseif ($r) { Write-Output $r } }
```

See README.md for detailed installation instructions.

---

## Command Line

```bash
bm                          # Open tree navigator (compact, 8 rows)
bm /path/to/directory       # Open navigator at specific path
bm myproject                # Jump to bookmark (prints path, no TUI)
bm -l                       # List all bookmarks
bm -a name                  # Save current directory as 'name'
bm -a name /path            # Save specific path as 'name'
bm -d name                  # Remove bookmark
bm -h / --help              # Show help
bm -v / --version           # Show version
```

---

## Tree Navigation

| Key              | Action                              |
|------------------|-------------------------------------|
| `j` `↓`          | Move down                           |
| `k` `↑`          | Move up                             |
| `l` `→`          | Expand directory                    |
| `h` `←`          | Collapse directory; if already collapsed, move to parent (collapsing it) |
| `Enter`          | Enter directory (change root)       |
| `u`              | Go back (undo last navigation)      |
| `Backspace`      | Go to parent directory              |
| `q`              | Exit and print selected path        |
| `Esc`            | Exit without output                 |

---

## Search

| Key         | Action                  |
|-------------|-------------------------|
| `/`         | Enter search mode       |
| Type        | Add to query            |
| `Backspace` | Delete character        |
| `Enter`     | Execute search          |
| `Tab`       | Switch tree ↔ results   |
| `j` `k`     | Navigate results        |
| `Enter`     | Jump to result          |
| `Esc`       | Cancel / close results  |

**Fuzzy mode**: prefix query with `/` — e.g., `/srch` finds `search`.

---

## Bookmarks

### Interactive (inside `bm`)

| Key     | Action                |
|---------|-----------------------|
| `m`     | Create bookmark       |
| `'`     | Open bookmark list    |
| `j` `k` | Navigate list         |
| `Tab`   | Toggle filter mode    |
| `d`     | Delete (press twice)  |
| `Enter` | Jump to bookmark      |
| `Esc`   | Close                 |

### CLI

```bash
bm -l                       # List all
bm -a work                  # Save current dir as 'work'
bm -a work /some/path       # Save specific path
bm -d work                  # Delete bookmark
bm work                     # Jump to bookmark
```

---

## Disk Selection

| Key     | Action                         |
|---------|--------------------------------|
| `d`     | Open disk/drive panel          |
| `j` `k` | Navigate drives                |
| `Enter` | Go to selected drive root      |
| `Esc`   | Close without navigating       |

---

## Mouse

| Action           | Effect                  |
|------------------|-------------------------|
| **Click**        | Select item             |
| **Double-click** | Expand/collapse         |
| **Scroll**       | Navigate tree           |

---

## Configuration

**File location:**
- Linux/macOS: `~/.config/bmrk/config.toml`
- Windows: `%APPDATA%\bmrk\config.toml`

```toml
[appearance]
theme = "default"       # default, gruvbox, nord, tokyonight, dracula, obsidian
max_name_length = 30    # Truncate long names (0 = disabled)

[appearance.colors]
selected_color = "cyan"
directory_color = "blue"
file_color = "white"

[behavior]
show_hidden = true
follow_symlinks = true
mouse_scroll_lines = 3

[keybindings]
search = ["/"]
create_bookmark = ["m"]
select_bookmark = ["'"]
select_disk = ["d"]
```

---

## Common Workflows

### Quick Navigation
```
bm                      → Open navigator
j/k                     → Move in tree
l/h                     → Expand / collapse or go to parent
u                       → Go back (undo last navigation)
q                       → Exit and print path
```

### Search and Jump
```
bm                      → Open navigator
/                       → Enter search mode
Type 'src' + Enter      → Search
j/k + Enter             → Jump to result
```

### Create and Use Bookmarks
```
bm ~/projects/myapp     → Navigate to project
m → type 'myapp' → Enter → Save bookmark
q                       → Exit

bm myapp                → Instant jump next time
```

### Switch Drive (Windows)
```
bm                      → Open navigator
d                       → Open disk panel
j/k                     → Select drive
Enter                   → Navigate to drive
```

---

## Tips

- Bookmark names are matched before directory paths — `bm work` jumps to a bookmark first
- Filter mode in bookmark selection (`Tab`) supports partial matching
- Fuzzy search: `/query` — finds non-adjacent matches, e.g., `/prjct` → `my_project`
- `q` outputs the path to stdout — pipe or capture as needed: `cd $(bm)`

---

**Repository**: https://github.com/holgertkey/bmrk  
**License**: MIT  
**Built with**: Rust + Ratatui
