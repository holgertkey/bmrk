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
| `u`         | Go back (undo last navigation)                                           |
| `Backspace` | Go to parent directory (change root)                                     |

### Other Actions

| Key | Action                              |
|-----|-------------------------------------|
| `/` | Enter search mode (folder name search) |
| `m` | Create bookmark for current directory  |
| `'` | Open bookmark selection menu (apostrophe) |
| `d` | Open disk/drive selection panel     |

### Exit

| Key   | Action                                        |
|-------|-----------------------------------------------|
| `q`   | Exit and cd to selected directory             |
| `Esc` | Exit without directory change                 |

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

### `Esc` Key

| Context              | Action                         |
|----------------------|--------------------------------|
| Tree mode            | Exit bmrk                      |
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

### `q` Key

| Context              | Action                                     |
|----------------------|--------------------------------------------|
| Tree mode            | Exit and cd to selected directory          |
| Search results       | Exit and cd to selected result's directory |
| Bookmark selection (navigation mode) | Exit and cd to selected bookmark |

`q` does not exit when bookmark filter mode is active (typed text goes to the filter instead).

## Keybinding Customization

The following keys can be customized in the `[keybindings]` section of `config.toml`:

```toml
[keybindings]
search          = ["/"]
create_bookmark = ["m"]
select_bookmark = ["'"]
select_disk     = ["d"]
```

Navigation keys (`j`, `k`, `h`, `l`, arrow keys, `Enter`, `Esc`, `Backspace`, `u`) and
mode-specific keys (`Tab`, `d` for deletion) are hardcoded.

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
