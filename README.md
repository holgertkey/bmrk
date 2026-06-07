# bmrk — Bookmark Manager and Directory Navigator

**A fast, compact TUI for directory navigation and bookmark management.**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/bmrk)](https://crates.io/crates/bmrk)

---

## What is bmrk?

**bmrk** is a compact inline TUI that fits in 8 rows and never takes over your terminal screen.
Navigate directories, manage bookmarks, search for folders — all from the keyboard.

The binary is `bmrk`. The shell wrapper `bm` wraps it and handles `cd` automatically — because a
process cannot change its parent shell's directory, a thin wrapper is required (the same approach
used by `fzf`, `zoxide`, and `autojump`).

---

## Features

- **Compact inline mode** — 8 rows, no fullscreen takeover, terminal fully restored on exit
- **Bookmarks** — save and jump to favorite directories; `bm myproject` navigates instantly
- **Interactive tree view** — directory navigation with expand/collapse
- **Fuzzy search** — fast asynchronous search with intelligent matching
- **Disk selection** — browse and switch between all drives/mount points (`d` key)
- **Mouse support** — click, double-click, scroll
- **Customizable** — TOML configuration with theme presets and custom colors

---

## Installation

### Step 1 — Install the `bmrk` binary

**From crates.io:**
```bash
cargo install bmrk
```

**From source:**
```bash
git clone https://github.com/holgertkey/bmrk.git
cd bmrk
cargo build --release
```

The binary is at `target/release/bmrk` (Linux/macOS) or `target\release\bmrk.exe` (Windows).

---

### Step 2 — Set up the `bm` wrapper

The repository includes three ready-made wrapper files. Each one integrates `bmrk` with a specific
shell so that `cd` works correctly. Pick the file that matches your environment.

---

#### `bm.bat` — Windows CMD

**What it does.** `bm.bat` is a batch script that runs `bmrk.exe` and redirects its stdout to a
temporary file. If the captured output is a valid directory path, it calls `cd /d` to change the
current session's directory. Non-directory output (help text, version, bookmark lists) is printed
as-is. The previous directory is saved in `%BMRK_PREV_DIR%` so that `bm -` can return to it.
Flags that do not trigger navigation (`-h`, `--help`, `-v`, `--version`, `-l`, `--list`, `-a`,
`--add`, `-d`, `--del`) are passed straight through to `bmrk.exe`.

**Installation:**

1. Copy `bmrk.exe` and `bm.bat` to the same directory that is on your `PATH`
   (e.g. `C:\Users\<YourName>\bin\`).

2. If that directory is not on your PATH yet, add it and open a new CMD window:
   ```
   setx PATH "%PATH%;C:\Users\<YourName>\bin"
   ```

3. Test:
   ```
   bm --version
   bm
   ```

---

#### `bm.ps1` — Windows PowerShell

**What it does.** `bm.ps1` defines a `bm` function for PowerShell. It runs `bmrk.exe`, captures
stdout to a temporary file, and calls `Set-Location` if the result is a valid directory path.
The previous directory is stored in `$env:BMRK_PREV_DIR` for `bm -` support. Pass-through flags
(`-h`, `--help`, `-v`, `--version`, `-l`, `--list`, `-a`, `--add`, `-d`, `--del`) are forwarded
directly to `bmrk.exe` without any cd logic. Compatible with Windows PowerShell 5.1 and
PowerShell 7+.

**Installation:**

1. Copy `bmrk.exe` to a directory on your PATH (e.g. `C:\Users\<YourName>\bin\`).

2. Open your PowerShell profile for editing:
   ```powershell
   notepad $PROFILE
   ```
   If the file does not exist yet, create it first:
   ```powershell
   New-Item -ItemType File -Force $PROFILE
   ```

3. Add the following line to your profile:
   ```powershell
   . "C:\path\to\bmrk\bm.ps1"
   ```

4. Reload the profile:
   ```powershell
   . $PROFILE
   ```

5. Test:
   ```powershell
   bm --version
   bm
   ```

---

#### `bm.sh` — bash, zsh (and fish via comment)

**What it does.** `bm.sh` defines a `bm` function for POSIX-compatible shells (bash and zsh). It
runs `bmrk` via command substitution, captures stdout, and calls `cd` when the result is a valid
directory. The previous directory is stored in `$BMRK_PREV_DIR` for `bm -` support. Any output
that is not a directory path (help text, version, bookmark list) is echoed to the terminal
unchanged. The file's header comment also contains a ready-to-use fish function for
`~/.config/fish/functions/bm.fish`.

**Installation — bash:**

1. Copy the `bmrk` binary to a directory on your PATH:
   ```bash
   cp target/release/bmrk ~/.local/bin/
   # or: sudo cp target/release/bmrk /usr/local/bin/
   ```

2. Source the wrapper from your shell config:
   ```bash
   echo 'source /path/to/bmrk/bm.sh' >> ~/.bashrc
   ```

3. Reload and test:
   ```bash
   source ~/.bashrc
   bm --version
   bm
   ```

**Installation — zsh:**

Same steps as bash, but add to `~/.zshrc` instead of `~/.bashrc`.

**Installation — fish:**

Fish uses its own function-file format and cannot source `bm.sh` directly. Instead, copy the fish
function from the comment block at the top of `bm.sh` into a new file:

```
~/.config/fish/functions/bm.fish
```

Fish picks up functions in that directory automatically — no reload step needed. Test with:

```bash
bm --version
bm
```

---

## Usage

```bash
bm                          # Open interactive TUI (compact, 8 rows)
bm /path/to/dir             # Open TUI at specific directory
bm myproject                # Jump to bookmark (cd directly, no TUI)
bm -                        # Return to previous directory

# Bookmark management
bm -l                       # List all bookmarks
bm -a work                  # Save current directory as 'work'
bm -a work /some/path       # Save specific path as 'work'
bm -d work                  # Remove bookmark 'work'

bm -v / --version           # Print version
bm -h / --help              # Print help
```

### Keyboard shortcuts (inside TUI)

| Key              | Action                         |
|------------------|--------------------------------|
| `j` / `↓`        | Move down                      |
| `k` / `↑`        | Move up                        |
| `l` / `→`        | Expand directory               |
| `h` / `←`        | Collapse directory; go to parent if already collapsed |
| `u`              | Go to parent directory         |
| `Backspace`      | Go back (undo last navigation) |
| `Enter`          | Go into directory (change root)|
| `q`              | Exit and cd to selected dir    |
| `Esc`            | Exit without cd                |
| `/`              | Search                         |
| `m`              | Create bookmark                |
| `'`              | Select bookmark                |
| `d`              | Disk selection                 |

---

## Configuration

Config file is created automatically on first run:

- **Linux/macOS**: `~/.config/bmrk/config.toml`
- **Windows**: `%APPDATA%\bmrk\config.toml`

Bookmarks are stored as `bookmarks.json` in the same directory.

```toml
[appearance]
theme = "default"       # default, gruvbox, nord, tokyonight, dracula, obsidian
max_name_length = 80    # Truncate long names in the middle (0 = disabled)
icons = "unicode"       # "unicode" (▼▶) or "ascii" (v>)

[appearance.colors]
# header_path_color = "cyan"       # Path/label in the header row
# header_hints_color = "darkgray"  # Key hints in the header row
# directory_color = "gray"         # Directory names; also the header icon (▼)
# See docs/configuration.md for all available color fields

[behavior]
show_hidden = true
follow_symlinks = true
mouse_scroll_lines = 1

[keybindings]
go_to_parent = ["u"]
go_back = ["Backspace"]
quit = ["q"]
exit = ["Esc"]
search = ["/"]
create_bookmark = ["m"]
select_bookmark = ["'"]
select_disk = ["d"]
```

---

## How the wrapper works

`bmrk` writes its TUI to **stderr** (visible in the terminal) and the selected directory path to
**stdout**. The wrapper captures stdout via a temp file (CMD: `set /p`; PowerShell/bash: `$t`):

- If captured output is a **valid directory path** → `cd` to it (saves previous dir for `bm -`)
- If output is **empty** (Esc pressed) → do nothing
- Otherwise → print the output as-is (help, version, bookmark list)

| Command               | bmrk stdout                | Wrapper action        |
|-----------------------|----------------------------|-----------------------|
| `bm` (TUI → `q`)      | `/selected/path`           | `cd` there            |
| `bm myproject`        | `/bookmarked/path`         | `cd` there            |
| `bm -`                | _(no bmrk call)_           | `cd` to previous dir  |
| `bm -l`               | `Bookmarks: …` (text)      | Print it (passthrough)|
| `bm -a work`          | `Bookmark 'work' added: …` | Print it (passthrough)|
| `bm -d work`          | `Bookmark 'work' removed`  | Print it (passthrough)|
| `bm --help`           | Help text                  | Print it (passthrough)|
| `bm -v`               | `bmrk 0.1.0`               | Print it (passthrough)|
| `bm` (TUI → Esc)      | _(empty)_                  | Do nothing            |

---

## License

MIT — see [LICENSE](LICENSE).

See [CHANGELOG.md](CHANGELOG.md) for version history.
