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

The wrapper is required for `cd` to work. Choose your shell below.

---

#### Windows — CMD

1. Copy `bmrk.exe` and `bm.bat` to the same directory that is on your `PATH`
   (e.g. `C:\Users\<YourName>\bin\`).

2. If that directory is not on your PATH yet, add it:
   ```
   setx PATH "%PATH%;C:\Users\<YourName>\bin"
   ```
   Then open a new CMD window.

3. Test:
   ```
   bm --version
   bm
   ```

---

#### Windows — PowerShell

1. Copy `bmrk.exe` to a directory on your PATH (e.g. `C:\Users\<YourName>\bin\`).

2. Open your PowerShell profile for editing:
   ```powershell
   notepad $PROFILE
   ```
   If the file does not exist yet, create it:
   ```powershell
   New-Item -ItemType File -Force $PROFILE
   ```

3. Add the wrapper to your profile. Choose one option:

   **Option A — source the provided file (recommended):**
   ```powershell
   . "C:\path\to\bmrk\bm.ps1"
   ```

   **Option B — inline function (no extra file needed):**
   ```powershell
   function bm {
       if ($args.Count -eq 1 -and $args[0] -eq '-') {
           if ($env:BMRK_PREV_DIR -and (Test-Path $env:BMRK_PREV_DIR -PathType Container)) {
               $prev = $env:BMRK_PREV_DIR
               $env:BMRK_PREV_DIR = $PWD.Path
               Set-Location $prev
           } else { Write-Error 'bm: no previous directory' }
           return
       }
       if ($args.Count -ge 1 -and $args[0] -in '-h','--help','-v','--version','-l','--list','-a','--add','-d','--del') {
           & bmrk.exe @args; return
       }
       $t = [IO.Path]::GetTempFileName()
       try {
           & bmrk.exe @args > $t
           if ($LASTEXITCODE -eq 0) {
               $r = (Get-Content $t -Raw)?.Trim()
               if ($r -and (Test-Path $r -PathType Container)) {
                   $env:BMRK_PREV_DIR = $PWD.Path; Set-Location $r
               } elseif ($r) { Write-Output $r }
           }
       } finally { Remove-Item $t -ErrorAction SilentlyContinue }
   }
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

#### Linux / macOS — bash

1. Copy the `bmrk` binary to a directory on your PATH:
   ```bash
   cp target/release/bmrk ~/.local/bin/
   # or: sudo cp target/release/bmrk /usr/local/bin/
   ```

2. Add the wrapper to `~/.bashrc` (choose one option):

   **Option A — source the provided file:**
   ```bash
   echo 'source /path/to/bmrk/bm.sh' >> ~/.bashrc
   ```

   **Option B — inline one-liner (no extra file):**
   ```bash
   echo 'bm() { local r; r=$(bmrk "$@"); [ -d "$r" ] && cd "$r" || { [ -n "$r" ] && echo "$r"; }; }' >> ~/.bashrc
   ```

3. Reload:
   ```bash
   source ~/.bashrc
   ```

4. Test:
   ```bash
   bm --version
   bm
   ```

---

#### Linux / macOS — zsh

Same as bash, but edit `~/.zshrc` instead of `~/.bashrc`.

---

#### Linux / macOS — fish

1. Copy the binary to your PATH:
   ```bash
   cp target/release/bmrk ~/.local/bin/
   ```

2. Create the function file:
   ```bash
   mkdir -p ~/.config/fish/functions
   ```
   Create `~/.config/fish/functions/bm.fish` with this content:
   ```fish
   function bm
       set r (bmrk $argv)
       if test -d "$r"
           cd $r
       else if test -n "$r"
           echo $r
       end
   end
   ```

3. Fish reloads functions automatically. Test:
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
