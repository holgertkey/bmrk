# Architecture

Internal architecture of bmrk.

## Overview

MVC-style modular architecture. `app.rs` is the thin orchestrator; all logic lives
in specialized modules.

```
main.rs          CLI parsing (clap), terminal setup, path/bookmark resolution, entry-point routing
app.rs           Central state — holds all module instances, delegates everything
terminal.rs      Terminal lifecycle: setup, cleanup, panic hook, main event loop (100ms poll)
event_handler.rs All keyboard and mouse input — routes to correct module based on app mode
ui.rs            All rendering (ratatui layouts, widget composition, area tracking for mouse hits)
navigation.rs    Tree state: root node, flat visible list, selection, expand/collapse
tree_node.rs     TreeNode data structure — Rc<RefCell<>> for zero-copy sharing
search.rs        Two-phase search: immediate visible-node scan + background thread for full tree
bookmarks.rs     Bookmark CRUD, persistence (JSON), interactive selection/creation/filter UI
config.rs        TOML config loading, color parsing, theme presets, auto-creates default config
disks.rs         Disk/volume information (via sysinfo)
platform.rs      Platform-specific path utilities (canonicalize, is_absolute, etc.)
theme/           Color theme structs and built-in presets
```

## Key Design Decisions

### stdout vs stderr

The TUI renders to **stderr**; path output goes to **stdout**. This is what makes the `bm`
wrapper work — it captures `$(bmrk "$@")` from stdout while the UI appears on screen.

### TreeNodeRef

```rust
pub type TreeNodeRef = Rc<RefCell<TreeNode>>;
```

The entire tree is shared references, not clones. The flat list in `Navigation` stores `Rc`
references into the same nodes, so expand/collapse and selection are O(1).

### Event Loop

100ms poll timeout in `terminal.rs`. On timeout (no input), `poll_search()` checks the
background thread channel for incremental search results.

### Terminal Cleanup

`cleanup_terminal_compact()` performs a multi-stage process to disable all mouse tracking
modes, drain pending events, and restore terminal state. This is critical — do not simplify
it. Terminal artifacts (escape sequences leaking into shell) occur specifically with
resize + mouse interaction.

### Error Handling

All errors propagate via `anyhow::Result`. Never use `std::process::exit()` — it bypasses
terminal cleanup. Config and bookmark errors happen before terminal init so cleanup is not
needed; runtime errors after terminal init must still go through the explicit
`cleanup_terminal_compact()` call before the result is checked.

## Module Details

### `main.rs`

Entry point. Handles:
- CLI argument parsing with `clap`
- Early exits (`-h`, `-v`, `-l`, `-a`, `-d`)
- Bookmark/path resolution for the positional argument
- Terminal setup → `run_app()` → terminal cleanup → path output

### `app.rs`

Thin state container. Holds instances of `Navigation`, `Search`, `Bookmarks`, `Config`,
and `EventHandler`. Delegates `handle_key()`, `handle_mouse()`, `render()`, `poll_search()`
to the appropriate modules.

### `terminal.rs`

Terminal lifecycle:
- `setup_terminal()` — enable raw mode, mouse capture, install panic hook
- `run_app()` — 100ms event loop; dispatches to `app.handle_key/mouse`, calls `app.render()`
- `cleanup_terminal_compact()` — multi-stage cleanup (mouse disable, event drain, raw mode off)

### `event_handler.rs`

Processes all keyboard and mouse events. Routes based on current app mode:
1. Search input mode → search handling
2. Disk selection → disk navigation
3. Bookmark selection → bookmark navigation
4. Bookmark creation → name input
5. Escape / q check
6. Tree mode → navigation

Mouse click and scroll events are dispatched to the active panel (disk, bookmark, or tree).
All panels use **minimal scrolling** — the view shifts only when the selection leaves the
visible area. Keyboard navigation sets `center_selection = true`; mouse actions set it to
`false`. This flag is present on `Navigation`, `Bookmarks`, and `Disks`.

### `ui.rs`

All rendering using `ratatui`. Calculates areas on each render and stores dimensions for
mouse hit testing. Renders: tree view, search results panel, bookmark panel, disk panel,
hint strings in the header row.

### `navigation.rs`

Manages the directory tree state:
- `root: TreeNodeRef` — current root node
- `flat_list: Vec<TreeNodeRef>` — visible nodes in display order
- `selected: usize` — cursor position
- `history: VecDeque<PathBuf>` — undo stack (up to 50 entries)
- `center_selection: bool` — scroll hint for the renderer: `true` centers the selection
  (keyboard navigation), `false` uses minimal scrolling (mouse actions)
- `nav_error: Option<String>` — last navigation error, displayed in the header until cleared

Key methods: `go_back()` (undo), `go_to_parent()`, `go_to_directory()`, `rebuild_flat_list()`.

### `tree_node.rs`

```rust
pub struct TreeNode {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub is_expanded: bool,
    pub depth: usize,
    pub children: Vec<TreeNodeRef>,
    pub has_children: Option<bool>, // None = unknown, Some(false) = leaf (no "▶")
    pub has_error: bool,            // set by probe_has_children OR load_children on read failure
    pub error_message: Option<String>,
}

pub type TreeNodeRef = Rc<RefCell<TreeNode>>;
```

Directories are loaded lazily when first expanded. `probe_has_children` is called on each
child after the parent loads — it peeks inside the directory to determine whether a `▶`
expand arrow should be shown, and also sets `has_error` if `read_dir` fails. This means
inaccessible directories are marked with `⊘` (error color) as soon as the parent expands,
without requiring the user to attempt entry.

### `search.rs`

Two-phase search:
- **Phase 1** (quick): searches already-loaded visible nodes — instant
- **Phase 2** (deep): background thread walks the full tree and sends results via
  `crossbeam-channel`; UI polls with `poll_search()` in the timeout branch of the event loop

Fuzzy mode activates when the query starts with `/` (uses `SkimMatcherV2`).
Results are capped at 500 to prevent UI stall on broad queries.

### `bookmarks.rs`

Bookmark CRUD with JSON persistence. Manages two interactive modes:
- `is_selecting` — bookmark selection panel with navigation and filter sub-modes
- `is_creating` — name input with existing bookmarks shown for reference

`center_selection: bool` mirrors the same flag on `Navigation` — keyboard moves center the
view, mouse clicks and scroll use minimal scrolling. `filter_mode: bool` switches between
navigation (j/k) and text filter (type to narrow) within the selection panel.

### `config.rs`

Loads `config.toml`, auto-creates with defaults if missing. Parses:
- `AppearanceConfig`: `theme`, `icons`, `max_name_length`, `colors: ThemeConfig`
- `BehaviorConfig`: `show_hidden`, `follow_symlinks`, `double_click_timeout_ms`, `mouse_scroll_lines`
- `KeybindingsConfig`: `search`, `create_bookmark`, `select_bookmark`, `select_disk`

### `disks.rs`

Uses the `sysinfo` crate to enumerate all disk volumes with mount point, filesystem type,
free space, and total capacity. `center_selection: bool` controls scroll behavior for the
disk list panel — same semantics as `Navigation::center_selection`.

### `platform.rs`

Platform-specific path helpers: canonicalization, absolute path checks.

### `theme/`

Color theme structs and built-in presets (`default`, `gruvbox`, `nord`, `tokyonight`,
`dracula`, `obsidian`).

## Data Flow

### Startup

```
main()
  → Config::load()?          — parse config.toml (or create defaults)
  → [Handle -h/-v/-l/-a/-d]  — early exits
  → [Resolve bookmark/path]  — positional arg
  → setup_terminal()?        — raw mode, mouse, panic hook
  → App::new()?              — init Navigation, Search, Bookmarks
  → run_app()?               — event loop
  → cleanup_terminal()?      — always runs
  → [print selected path]
```

### Event Loop

```
loop {
  terminal.draw(|f| app.render(f))

  if event::poll(100ms) {
    Key(k)    → app.handle_key(k) → Some(path) | None = exit
    Mouse(m)  → app.handle_mouse(m)
    Resize    → consume (next draw recalculates layout)
  } else {
    app.poll_search()   // drain background search channel
  }
}
```

### Search Flow

```
User presses '/'
  → search.enter_mode()
User presses Enter
  → search.perform_search()
      → Phase 1: scan loaded nodes (instant)
      → Phase 2: spawn thread, walk tree, send via channel
Main loop (100ms timeout)
  → app.poll_search() → drain channel, update results, re-render
```

## Dependencies

| Crate               | Purpose                          |
|---------------------|----------------------------------|
| `ratatui 0.28`      | TUI framework                    |
| `crossterm 0.28`    | Terminal manipulation            |
| `anyhow 1.0`        | Error handling                   |
| `clap 4.5`          | CLI argument parsing             |
| `serde + serde_json`| Bookmark JSON persistence        |
| `toml 0.8`          | Config file parsing              |
| `dirs 5.0`          | Platform config/data directories |
| `crossbeam-channel` | Background thread communication  |
| `fuzzy-matcher 0.3` | Fuzzy search (SkimMatcherV2)     |
| `sysinfo 0.32`      | Disk enumeration                 |
| `unicode-width 0.1` | Unicode display width            |
| `time >= 0.3.47`    | Pinned to fix RUSTSEC-2026-0009  |
