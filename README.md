# bmrk — Bookmark Manager and Directory Navigator

**A fast, compact TUI for directory navigation and bookmark management.**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/bmrk)](https://crates.io/crates/bmrk)

---

## What is bmrk?

**bmrk** (`bm` command) is a compact inline TUI that fits in 8 rows and never takes over your terminal screen. Navigate directories, manage bookmarks, search for files — all from the keyboard.

**Bookmarks are the core.** Save a directory with `m`, jump back with `bm myproject` from anywhere. No more retyping long paths.

---

## Features

- **Compact inline mode** — default launch uses only 8 rows, leaves terminal clean on exit
- **Bookmarks** — save and jump to favorite directories; `bm myproject` navigates instantly
- **Interactive tree view** — visual directory navigation with expand/collapse
- **Fuzzy search** — fast asynchronous search with intelligent matching
- **Disk selection** — browse and switch between all drives/mount points (`d` key)
- **Mouse support** — click, double-click, scroll
- **Customizable** — TOML configuration with full theme support

---

## Installation

### From crates.io

```bash
cargo install bmrk
```

### From source

```bash
git clone https://github.com/holgertkey/bmrk.git
cd bmrk
cargo build --release
cp target/release/bm ~/bin/   # Linux/macOS
# or copy target\release\bm.exe to a directory on your PATH (Windows)
```

---

## Usage

```bash
bm                          # Open interactive TUI from current directory
bm /path/to/dir             # Open TUI from specific directory
bm myproject                # Jump to bookmark 'myproject' (prints path)

# Bookmark management
bm -bm                      # List all bookmarks
bm -bm add work             # Save current directory as 'work'
bm -bm add work /some/path  # Save specific path as 'work'
bm -bm remove work          # Remove bookmark 'work'
bm -bm list                 # List all bookmarks

bm --version                # Print version
bm -h / --help              # Print help
```

### Keyboard shortcuts

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `l` / `→` | Expand directory |
| `h` / `←` | Collapse directory |
| `u` / `Backspace` | Go to parent |
| `Enter` | Go into directory |
| `q` | Exit, print selected path |
| `Esc` | Exit without output |
| `/` | Search |
| `m` | Create bookmark |
| `'` | Select bookmark |
| `d` | Disk selection |

---

## Configuration

Config file is created automatically on first run:

- **Linux/macOS**: `~/.config/bmrk/config.toml`
- **Windows**: `%APPDATA%\bmrk\config.toml`

Bookmarks are stored in the same directory as `bookmarks.json`.

Example config:

```toml
[appearance]
theme = "default"   # default, gruvbox, nord, tokyonight, dracula, obsidian
max_name_length = 30

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

## License

MIT — see [LICENSE](LICENSE).

See [CHANGELOG.md](CHANGELOG.md) for version history.
