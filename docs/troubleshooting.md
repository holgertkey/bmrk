# Troubleshooting

Common issues and their solutions.

## Installation Issues

### Rust Not Found

**Problem**: `cargo: command not found`

**Solution**:
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
cargo --version
```

### Build Errors

**Problem**: Compilation fails

**Solution**:
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

**Problem**: Missing build tools on Linux

**Solution**:
```bash
# Ubuntu/Debian
sudo apt install build-essential

# Arch
sudo pacman -S base-devel

# Fedora
sudo dnf groupinstall "Development Tools"
```

### Binary Not Found

**Problem**: `bmrk: command not found`

**Solution**:
```bash
# Linux/macOS — check PATH
which bmrk
echo $PATH

# Copy binary to a PATH directory
cp target/release/bmrk ~/.local/bin/

# PowerShell — check PATH
(Get-Command bmrk -ErrorAction SilentlyContinue).Source
$env:PATH
```

### `bm` Opens but Directory Doesn't Change

**Problem**: `bm` runs but the shell directory does not change after selecting.

**Solutions**:
- Ensure you're using the shell wrapper (`bm.ps1` on PowerShell, `bm.sh` on bash/zsh/fish), not just the `bmrk` binary directly.
- On Windows: if using PowerShell, do not use `bm.bat` — `.bat` files run as subprocesses and cannot change the parent session's directory. Use the PowerShell wrapper instead.
- Check the wrapper is sourced (not executed): `source ~/.bashrc` after adding it.

---

## Runtime Issues

### Broken UI / Strange Characters

**Problem**: Characters look wrong or the layout is garbled.

**Solutions**:

1. Check UTF-8 support:
   ```bash
   echo $LANG   # Should contain UTF-8
   export LANG=en_US.UTF-8
   ```

2. Check terminal color support:
   ```bash
   echo $TERM   # Should be xterm-256color or similar
   ```

3. Try a different terminal (Alacritty, Kitty, WezTerm, or iTerm2 on macOS work well).

### Mouse Doesn't Work

**Solutions**:
1. Most modern terminals support mouse reporting — check terminal settings.
2. If mouse isn't working, all bmrk features are fully accessible via keyboard.

### Terminal Artifacts After Exit

**Problem**: Escape sequences or stray characters appear in the shell after exiting bmrk
(e.g., `35;64;18M`).

**Solution**:
- This was fully resolved in v0.1.0+ with improved multi-stage terminal cleanup.
- Update to the latest version.
- If still occurring, report as a bug with your terminal emulator name and version.

---

## Search Issues

### Search Finds No Results

**Solutions**:

1. Remember that bmrk searches **folder names only** — not file names, not paths, not content.
   `main.rs` will never appear in results; `src` and `tests` will.

2. Check query syntax:
   - `config` → substring match (case-insensitive) against folder names
   - `/cnfg` → fuzzy match (starts with `/`)

3. Wait for the background search to complete — Phase 1 is instant, Phase 2 scans the full
   tree in the background. Watch for "Scanned: N directories" progress in the results panel.

4. If the tree root is a shallow directory, expand some subdirectories and search again —
   loaded nodes are searched in Phase 1 (instant).

### Search Is Slow

**Solution**: Normal for very large trees (100K+ directories). Phase 1 results are always
instant; Phase 2 runs in the background without blocking the UI. Press `Esc` to stop the
background search if needed.

---

## Bookmark Issues

### Bookmarks File Location

- **Linux/macOS**: `~/.config/bmrk/bookmarks.json`
- **Windows**: `%APPDATA%\bmrk\bookmarks.json`

### Bookmarks Disappear After Restart

**Solutions**:

1. Check the file exists and has correct permissions:
   ```bash
   ls ~/.config/bmrk/bookmarks.json
   chmod 644 ~/.config/bmrk/bookmarks.json
   ```

2. Check disk space:
   ```bash
   df -h ~
   ```

3. Create the file manually if needed:
   ```bash
   mkdir -p ~/.config/bmrk
   echo '[]' > ~/.config/bmrk/bookmarks.json
   ```

### Bookmark Path No Longer Exists

bmrk stores paths as-is. If a bookmarked directory was moved or deleted, the bookmark
becomes stale. Remove it with `bm -d <name>` and add a new one.

---

## Configuration Issues

### Config File Not Created

Run bmrk once to trigger auto-creation:

```bash
bm
# Press Esc to exit
```

### Config Changes Not Applied

1. bmrk reads the config at startup — restart after editing.
2. Check for TOML syntax errors (mismatched quotes, invalid values).
3. Verify the file is at the correct location (see above).

### Colors Not Working

1. Check terminal supports true color:
   ```bash
   echo $COLORTERM   # Should show "truecolor"
   ```

2. Try indexed colors (0–255) for maximum compatibility:
   ```toml
   [appearance.colors]
   selected_color = "51"
   directory_color = "39"
   ```

---

## Platform-Specific Issues

### Windows: PowerShell Wrapper

If `bm` is not found after adding the wrapper to `$PROFILE`:
```powershell
# Reload profile
. $PROFILE

# Verify wrapper is defined
Get-Command bm
```

### Linux/macOS: Permission Errors

```bash
# Ensure binary is executable
chmod +x ~/.local/bin/bmrk
```

---

## Getting More Help

When reporting issues, include:

```bash
# bmrk version
bmrk --version

# Rust and system info
rustc --version
uname -a      # Linux/macOS
$PSVersionTable  # Windows PowerShell

# Terminal
echo $TERM
echo $COLORTERM
```

Report bugs at: https://github.com/holgertkey/bmrk/issues

Include:
- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- The debug information above
- Screenshots if applicable
