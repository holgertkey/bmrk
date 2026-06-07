# Key Bindings

Complete reference of all keyboard shortcuts in bmrk.

## Tree Navigation Mode

Default mode when bmrk launches.

### Movement

| Key        | Action             |
|------------|--------------------|
| `j` or `↓` | Move down one item |
| `k` or `↑` | Move up one item   |

### Directory Operations

| Key         | Action                                                                   |
|-------------|--------------------------------------------------------------------------|
| `l` or `→`  | Expand directory (show subdirectories)                                   |
| `h` or `←`  | Collapse expanded directory; if already collapsed, move to parent        |
| `Enter`     | Change root to selected directory                                        |
| `u`         | Go to parent directory (change root) — configurable: `go_to_parent`     |
| `Backspace` | Go back (undo last navigation) — configurable: `go_back`                |

### Other Actions

| Key | Action                              |
|-----|-------------------------------------|
| `/` | Enter search mode — configurable: `search`  |
| `m` | Create bookmark — configurable: `create_bookmark` |
| `'` | Open bookmark selection — configurable: `select_bookmark` |
| `d` | Open disk/drive selection — configurable: `select_disk` |

### Exit

| Key   | Action                                        |
|-------|-----------------------------------------------|
| `q`   | Exit and cd to selected directory — configurable: `quit` |
| `Esc` | Exit without directory change — configurable: `exit` |

## Search Mode

After pressing `/`:

### Input

| Key             | Action                           |
|-----------------|----------------------------------|
| Type characters | Add to search query              |
| `Backspace`     | Remove last character            |
| `Enter`         | Execute search and show results  |
| `Esc`           | Cancel search (exit search mode) |

### Fuzzy Search

Start the query with `/` to enable fuzzy matching:

```
src        Normal search — finds folders whose name contains "src"
/srch      Fuzzy search — finds "search", "src", "scratch", etc.
```

Results are ranked by relevance score.

### Search Results Navigation

After executing a search:

| Key        | Action                                        |
|------------|-----------------------------------------------|
| `Tab`      | Switch focus between tree and results         |
| `j` or `↓` | Navigate down in results                      |
| `k` or `↑` | Navigate up in results                        |
| `Enter`    | Jump to selected result in tree               |
| `q`        | Exit and cd to selected result's directory    |
| `Esc`      | Close results and exit search mode            |

## Bookmark Creation Mode

After pressing `m`:

| Key             | Action                                         |
|-----------------|------------------------------------------------|
| Type characters | Add to bookmark name                           |
| `Backspace`     | Remove last character                          |
| `Enter`         | Save bookmark                                  |
| `Esc`           | Cancel bookmark creation                       |

Naming rules: alphanumeric, hyphens, and underscores; no path separators; cannot use `-`, `.`, `..`.

## Bookmark Selection Mode

After pressing `'`:

### Navigation Mode (Default)

| Key        | Action                                              |
|------------|-----------------------------------------------------|
| `j` or `↓` | Move selection down                                 |
| `k` or `↑` | Move selection up                                   |
| `Enter`    | Jump to selected bookmark                           |
| `q`        | Exit and cd to selected bookmark                    |
| `d`        | Mark bookmark for deletion (press twice to confirm) |
| `Tab`      | Switch to filter mode                               |
| `Esc`      | Close bookmark selection                            |

### Filter Mode

| Key             | Action                                     |
|-----------------|--------------------------------------------|
| Type characters | Filter bookmarks by name or path           |
| `Backspace`     | Remove last character from filter          |
| `Tab`           | Switch back to navigation mode             |
| `Enter`         | Jump to selected bookmark                  |
| `Esc`           | Close bookmark selection                   |

### Deletion Workflow

Two-phase deletion prevents accidental removal:

1. Press `d` once → bookmark marked with `[DEL]` prefix
2. Press `d` again → bookmark deleted
3. Navigate with `j`/`k` → mark cleared (cancels deletion)

## Disk Selection Mode

After pressing `d` in tree mode:

| Key        | Action                        |
|------------|-------------------------------|
| `j` or `↓` | Move selection down           |
| `k` or `↑` | Move selection up             |
| `Enter`    | Navigate to selected disk root |
| `Esc`      | Close without navigating       |

## Mouse Bindings

| Action       | Effect                         |
|--------------|--------------------------------|
| Click        | Select item under cursor       |
| Double-click | Expand/collapse directory      |
| Scroll wheel | Navigate tree up/down          |

## Context-Specific Behavior

### `exit` key (`Esc` by default)

| Context              | Action                         |
|----------------------|--------------------------------|
| Tree mode            | Exit bmrk without output       |
| Search input mode    | Cancel search                  |
| Search results       | Close results panel            |
| Bookmark creation    | Cancel creation                |
| Bookmark selection   | Close selection panel          |
| Disk selection       | Close disk panel               |

### `Enter` Key

| Context              | Action                                  |
|----------------------|-----------------------------------------|
| Tree mode, directory | Change root to directory                |
| Search input mode    | Execute search                          |
| Search results       | Jump to selected result in tree         |
| Bookmark creation    | Save bookmark                           |
| Bookmark selection   | Jump to selected bookmark               |
| Disk selection       | Navigate to selected disk root          |

### `quit` key (`q` by default)

| Context              | Action                                     |
|----------------------|--------------------------------------------|
| Tree mode            | Exit and cd to selected directory          |
| Search results       | Exit and cd to selected result's directory |
| Bookmark selection (navigation mode) | Exit and cd to selected bookmark |
| Disk selection       | Exit and cd to selected disk root          |

`quit` does not exit when bookmark filter mode is active — typed text goes to the filter instead.

## Keybinding Customization

All major keys can be remapped in the `[keybindings]` section of `config.toml`:

```toml
[keybindings]
# Navigation
go_to_parent = ["u"]          # Go to parent directory
go_back      = ["Backspace"]  # Go back (undo last navigation)

# Exit
quit = ["q"]    # Exit and output selected path (triggers cd in shell)
exit = ["Esc"]  # Exit without output / cancel current mode

# Panels
search          = ["/"]
create_bookmark = ["m"]
select_bookmark = ["'"]
select_disk     = ["d"]
```

Each field accepts a list — multiple keys can trigger the same action:

```toml
go_back = ["Backspace", "b"]   # Backspace or b to go back
quit    = ["q", "Q"]           # Both q and Q quit with output
```

Keys not remappable: `j`/`k` (move up/down), `h`/`l` (collapse/expand), `Enter`, `Tab`, arrow keys, and `d` (deletion inside bookmark selection).

## Quick Reference Card

### Tree Navigation

```
Movement:    j/k (down/up)   l (expand)   h (collapse/parent)   u (back)
Other:       / (search)   m (bookmark)   ' (bookmarks)   d (disks)
Exit:        q (exit+cd)   Esc (exit)   Enter (enter dir)
```

### Search

```
Input:       type query   Backspace (delete)   Enter (search)   Esc (cancel)
Results:     j/k (nav)   Enter (jump)   q (exit+cd)   Tab (focus)   Esc (close)
```

### Bookmarks

```
Create:      m   type name   Enter (save)   Esc (cancel)
Select:      '   j/k (nav)   Enter (jump)   q (exit+cd)   Tab (filter)   d d (delete)
```
