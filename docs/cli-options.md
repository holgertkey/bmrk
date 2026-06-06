# CLI Reference

Complete command-line interface reference for bmrk.

## Synopsis

```
bmrk [OPTIONS] [PATH|BOOKMARK]
bm   [OPTIONS] [PATH|BOOKMARK]
```

`bm` is the recommended shell wrapper (see [Installation](./installation.md)).
Use `bmrk` directly in scripts where `cd` behavior is not needed.

---

## Interactive Navigation

```bash
# Launch TUI from current directory
bm

# Launch TUI with a specific directory as root
bm /path/to/directory

# Launch TUI at a relative path
bm ../other-project

# Jump to a bookmark (no TUI — navigates instantly)
bm myproject

# Return to previous directory (wrapper only)
bm -
```

**Resolution order for positional argument**:
1. Saved bookmark with that name
2. Valid directory path (absolute or relative)
3. Error

---

## Bookmark Management

```bash
# List all saved bookmarks
bm -l
bm --list

# Add bookmark for the current directory
bm -a NAME
bm --add NAME

# Add bookmark for a specific path
bm -a NAME /path/to/directory
bm --add NAME /path/to/directory

# Remove a bookmark
bm -d NAME
bm --del NAME
```

**Bookmark naming rules**:
- Alphanumeric characters, hyphens, underscores
- No path separators (`/`, `\`)
- Cannot use reserved names: `-`, `.`, `..`
- Case-sensitive

---

## Help and Version

```bash
# Show help (all keys, config paths, CLI reference)
bm -h
bm --help

# Show version
bm -v
bm --version
```

---

## Options Reference

### `-h, --help`

Print the embedded help reference and exit. Covers all keybindings, CLI options,
configuration file location, and bookmark storage paths.

### `-v, --version`

Print the version string and exit.

Output format: `bmrk x.y.z`

### `-l, --list`

List all saved bookmarks to stdout and exit.

### `-a, --add NAME [PATH]`

Add a bookmark named `NAME`. If `PATH` is omitted, the current working directory is used.

### `-d, --del NAME`

Remove the bookmark named `NAME`.

### `[PATH|BOOKMARK]`

Optional positional argument.

- If a bookmark with this name exists — navigate to it (no TUI)
- If a valid directory path — open TUI rooted there
- Otherwise — error

---

## Storage

### Configuration

- **Linux/macOS**: `~/.config/bmrk/config.toml`
- **Windows**: `%APPDATA%\bmrk\config.toml`

Auto-created with defaults on first run.

### Bookmarks

- **Linux/macOS**: `~/.config/bmrk/bookmarks.json`
- **Windows**: `%APPDATA%\bmrk\bookmarks.json`

Auto-created when the first bookmark is added.

**Format** (JSON array):
```json
[
  { "key": "work", "path": "/home/user/work" },
  { "key": "webapp", "path": "/home/user/projects/webapp" }
]
```

---

## Exit Codes

| Code | Meaning                                       |
|------|-----------------------------------------------|
| `0`  | Normal exit                                   |
| `1`  | Error (invalid argument, path not found, etc.)|

---

## Environment Variables

### `BMRK_PREV_DIR`

Used by the `bm` wrapper to track the previous directory for `bm -`.

- **Set by**: the `bm` wrapper after each successful navigation
- **Format**: absolute directory path
- **Usage**: internal — do not set manually

---

## Script Integration

Use `bmrk` (not `bm`) in scripts to avoid the `cd` side-effect:

```bash
#!/bin/bash
# Capture selected path from bmrk
selected=$(bmrk /path/to/start)
if [ -d "$selected" ]; then
    # Process selected directory
    echo "Selected: $selected"
fi
```

---

## Examples

```bash
# Open TUI from current directory
bm

# Open TUI at /var/log
bm /var/log

# Jump to bookmark 'work' (no TUI)
bm work

# Return to previous directory
bm -

# List all bookmarks
bm -l

# Create a bookmark for ~/projects/webapp
bm -a webapp ~/projects/webapp

# Remove a bookmark
bm -d webapp

# Show version
bm -v

# Show help
bm -h
```

---

## See Also

- [Getting Started](./getting-started.md)
- [Basic Usage](./usage.md)
- [Key Bindings](./keybindings.md)
- [Configuration](./configuration.md)
- [Installation](./installation.md)
