# Introduction

**bmrk** is a compact inline TUI for directory navigation and bookmark management.
It fits in 8 rows and never takes over your terminal screen.

## What is bmrk?

bmrk is written in Rust and designed for fast, keyboard-driven directory navigation.
Unlike traditional `cd` + `ls` workflows, bmrk gives you a visual tree view of your
filesystem directly in the terminal stream — without interrupting your scroll history
or entering a fullscreen mode.

The binary is `bmrk`. The shell wrapper `bm` wraps it and handles `cd` automatically:
because a process cannot change its parent shell's directory, a thin wrapper is required
(the same approach used by `fzf`, `zoxide`, and `autojump`).

## Key Features

- **Compact inline mode** — 8 rows, no fullscreen takeover, terminal fully restored on exit
- **Interactive tree view** — directory navigation with expand/collapse and vim-style keys
- **Bookmark system** — save and jump to favorite directories; `bm myproject` navigates instantly
- **Fuzzy search** — fast asynchronous folder-name search with intelligent matching
- **Disk selection** — browse and switch between all drives and mount points
- **Mouse support** — click, double-click, scroll
- **Customizable** — TOML configuration with theme presets and custom colors

## Design Philosophy

1. **Keyboard-first** — all features accessible via keyboard, mouse is optional
2. **Compact** — stays out of the way; opens and closes without disrupting your workflow
3. **Terminal-native** — no GUI dependencies, works over SSH
4. **Non-destructive** — navigate and view, never modify files
5. **Configurable** — sensible defaults, everything customizable

## Next Steps

- [Getting Started](./getting-started.md) — quick start guide
- [Installation](./installation.md) — installation instructions
- [Basic Usage](./usage.md) — learn the basics
- [Configuration](./configuration.md) — customize bmrk
