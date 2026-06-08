# Configuration

bmrk uses a TOML configuration file that is automatically created on first launch.

## Configuration File Location

- **Linux/macOS**: `~/.config/bmrk/config.toml`
- **Windows**: `%APPDATA%\bmrk\config.toml`

## Configuration Structure

The file has three sections:

1. **`[appearance]`** — visual settings
2. **`[behavior]`** — functional settings
3. **`[keybindings]`** — keyboard shortcuts

## Default Configuration

```toml
[appearance]
# Color theme preset
# Options: default, gruvbox, nord, tokyonight, dracula, obsidian
theme = "default"

# Icon style for directories and tree indicators
# Options: "unicode" (default), "ascii"
icons = "unicode"

# Maximum display length for directory names (middle-truncation)
# Names longer than this are shortened with "..." in the middle
# Set to 0 to disable truncation
max_name_length = 80

[appearance.colors]
# Override individual theme colors
# Formats: color name ("cyan"), RGB hex ("#00FFFF"), or indexed (0-255)
# selected_color = "cyan"         # Search bar and bookmark input text
# directory_color = "gray"        # Directory names in the tree; also the header icon (▼)
# file_color = "white"            # File names and list items
# error_color = "red"             # Inaccessible directories (⊘), nav errors, deletion markers
# highlight_color = "yellow"      # Matched characters in search results
# cursor_color = "yellow"         # Selected item in search/bookmark/disk lists
# tree_cursor_color = "dim"       # Selected item color in the tree
# tree_cursor_bg_color = "dim"    # Selected item background in the tree
# header_path_color = "cyan"      # Path and mode labels in the header row
# header_hints_color = "darkgray" # Key hint text in the header row

[behavior]
# Show hidden files (dotfiles) — default: true
show_hidden = true

# Follow symbolic links when traversing — default: true
follow_symlinks = true

# Mouse double-click timeout in milliseconds — default: 800
double_click_timeout_ms = 800

# Number of lines to scroll with the mouse wheel — default: 1
mouse_scroll_lines = 1

[keybindings]
# Customizable key bindings — each accepts a list of key strings
search          = ["/"]
create_bookmark = ["m"]
select_bookmark = ["'"]
select_disk     = ["d"]
```

## Appearance Settings

### Theme Presets

Choose a built-in color theme:

```toml
[appearance]
theme = "default"     # Clean blue/cyan
theme = "gruvbox"     # Warm retro palette
theme = "nord"        # Arctic cool tones
theme = "tokyonight"  # Dark purple/blue
theme = "dracula"     # Classic dark theme
theme = "obsidian"    # Dark gray tones
```

### Color Overrides

Override individual colors within any theme. Three formats are supported:

```toml
[appearance.colors]
selected_color = "cyan"        # Color name
directory_color = "#569CD6"    # RGB hex
cursor_color = "240"           # Indexed (0-255)
```

Available color fields:

| Field | Default | What it colors |
|---|---|---|
| `selected_color` | `"cyan"` | Search bar and bookmark input text |
| `directory_color` | `"gray"` | Directory names in the tree; also the header icon (▼) |
| `file_color` | `"white"` | File names and list items |
| `error_color` | `"red"` | Inaccessible directories (`⊘`), navigation errors, deletion markers |
| `highlight_color` | `"yellow"` | Matched characters in search results |
| `cursor_color` | `"yellow"` | Selected item in search/bookmark/disk lists |
| `tree_cursor_color` | `"dim"` | Selected item color in the tree |
| `tree_cursor_bg_color` | `"dim"` | Selected item background in the tree |
| `header_path_color` | `"cyan"` | Path and mode labels in the header row |
| `header_hints_color` | `"darkgray"` | Key hint text in the header row |

Available color names: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`,
`gray`, `darkgray`, `lightred`, `lightgreen`, `lightyellow`, `lightblue`, `lightmagenta`, `lightcyan`,
or `"reset"` (terminal default).

### Icon Style

Controls the characters used for tree indicators and folder icons:

```toml
icons = "unicode"   # ▼ ► (default)
icons = "ascii"     # v > (plain ASCII, maximum compatibility)
```

### Name Truncation

Long directory names are middle-truncated for readability:

```toml
max_name_length = 80     # Default
max_name_length = 40     # Shorter names
max_name_length = 0      # Disable truncation
```

Example: `very_long_project_name_here` → `very_long_proj...ame_here`

## Behavior Settings

### Show Hidden Files

Include dotfiles (`.git`, `.config`, etc.) in the tree:

```toml
show_hidden = true    # Default — show hidden files
show_hidden = false   # Hide dotfiles
```

### Follow Symlinks

Follow symbolic links when traversing directories:

```toml
follow_symlinks = true    # Default — follow symlinks
follow_symlinks = false   # Skip symlinks (safer, avoids infinite loops with circular links)
```

### Double-Click Timeout

How quickly two clicks must occur to register as a double-click:

```toml
double_click_timeout_ms = 800   # Default (0.8 seconds)
double_click_timeout_ms = 500   # Faster
double_click_timeout_ms = 1000  # Slower
```

### Mouse Scroll Speed

Lines scrolled per mouse wheel tick:

```toml
mouse_scroll_lines = 1    # Default — one line at a time
mouse_scroll_lines = 3    # Faster scrolling
```

## Keybinding Configuration

All major keys are configurable. Each binding accepts a list so multiple keys can trigger the same action:

```toml
[keybindings]
# Navigation
go_to_parent = ["u"]          # Go to parent directory (change root up one level)
go_back      = ["Backspace"]  # Go back (undo last navigation)

# Exit
quit = ["q"]    # Exit and output selected path to shell (triggers cd)
exit = ["Esc"]  # Exit without output / cancel current mode

# Panels
search          = ["/"]   # Enter search mode
create_bookmark = ["m"]   # Create bookmark
select_bookmark = ["'"]   # Open bookmark selection
select_disk     = ["d"]   # Open disk selection
```

Multiple keys per action:

```toml
quit = ["q", "Q"]           # Both q and Q quit with output
go_back = ["Backspace", "b"] # Backspace or b to go back
```

Supported key names: single letters/symbols, `Esc`, `Enter`, `Backspace`, `Tab`,
`Up`, `Down`, `Left`, `Right`, `Home`, `End`, `PageUp`, `PageDown`, `Delete`.

Keys not configurable: `j`/`k` (up/down), `h`/`l` (collapse/expand), arrow keys, `Enter`, `Tab` — these are hardcoded navigation keys within each mode.

## Resetting Configuration

Delete the config file to reset to defaults:

```bash
# Linux/macOS
rm ~/.config/bmrk/config.toml

# Windows PowerShell
Remove-Item "$env:APPDATA\bmrk\config.toml"
```

The file will be recreated with defaults on the next run.

## Configuration Examples

### Dark Theme with Custom Colors

```toml
[appearance]
theme = "tokyonight"
max_name_length = 60

[appearance.colors]
selected_color = "#00FFFF"
directory_color = "#569CD6"
header_hints_color = "#404040"

[behavior]
show_hidden = true
mouse_scroll_lines = 2
```

### Minimal Setup (ASCII only)

```toml
[appearance]
theme = "default"
icons = "ascii"
max_name_length = 0

[behavior]
show_hidden = false
follow_symlinks = false
```

## Next Steps

- [Key Bindings](./keybindings.md) — complete keybinding reference
- [Features](./features.md) — detailed feature documentation
- [Troubleshooting](./troubleshooting.md) — common issues
