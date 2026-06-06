use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind, MouseEvent,
        MouseEventKind,
    },
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal, TerminalOptions, Viewport};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};

use crate::app::App;

/// Compact mode height in rows (1 header + content rows)
pub const COMPACT_HEIGHT: u16 = 8;

/// Global flag: true when compact inline mode is active (used by panic hook)
static IS_COMPACT_MODE: AtomicBool = AtomicBool::new(false);

/// Row where compact viewport starts (absolute screen row, 0-indexed)
static COMPACT_START_ROW: AtomicU16 = AtomicU16::new(0);

/// Install panic hook to ensure terminal is always cleaned up
pub fn install_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // Clean up whichever terminal mode is active
        if IS_COMPACT_MODE.load(Ordering::Relaxed) {
            let _ = cleanup_terminal_compact();
        } else {
            let _ = cleanup_terminal();
        }
        original_hook(panic_info);
    }));
}

#[allow(dead_code)]
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<std::io::Stderr>>> {
    // Install panic hook before any terminal modifications
    install_panic_hook();
    IS_COMPACT_MODE.store(false, Ordering::Relaxed);

    enable_raw_mode()?;
    std::io::stderr().execute(EnterAlternateScreen)?;
    std::io::stderr().execute(EnableMouseCapture)?;

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

/// Set up a compact inline terminal that occupies only COMPACT_HEIGHT rows.
/// The inline viewport appears at the current cursor position without taking
/// over the full screen. On cleanup, the area is erased and the terminal
/// returns to its pre-launch state.
pub fn setup_terminal_compact() -> Result<Terminal<CrosstermBackend<std::io::Stderr>>> {
    install_panic_hook();
    IS_COMPACT_MODE.store(true, Ordering::Relaxed);

    // Enable raw mode first (needed for cursor::position() on Unix)
    enable_raw_mode()?;

    // Determine where our viewport will start after potential terminal scrolling.
    // Viewport::Inline(N) scrolls the terminal up if the cursor is too close to
    // the bottom, so we calculate the absolute start row in advance.
    let (_, cursor_row) = crossterm::cursor::position().unwrap_or((0, 0));
    let (_, terminal_height) = crossterm::terminal::size().unwrap_or((80, 24));
    let start_row = if cursor_row + COMPACT_HEIGHT >= terminal_height {
        terminal_height.saturating_sub(COMPACT_HEIGHT)
    } else {
        cursor_row
    };
    COMPACT_START_ROW.store(start_row, Ordering::Relaxed);

    std::io::stderr().execute(EnableMouseCapture)?;

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Inline(COMPACT_HEIGHT),
        },
    )?;

    Ok(terminal)
}

/// Clean up after compact inline mode.
/// Erases the COMPACT_HEIGHT rows that were drawn and restores the cursor to
/// its position before the program launched — leaving the terminal clean.
pub fn cleanup_terminal_compact() -> Result<()> {
    use std::io::Write;

    IS_COMPACT_MODE.store(false, Ordering::Relaxed);

    // 1. Disable all mouse tracking modes
    let _ = write!(std::io::stderr(), "\x1b[?1000l");
    let _ = write!(std::io::stderr(), "\x1b[?1002l");
    let _ = write!(std::io::stderr(), "\x1b[?1003l");
    let _ = write!(std::io::stderr(), "\x1b[?1006l");
    let _ = write!(std::io::stderr(), "\x1b[?1015l");
    let _ = std::io::stderr().execute(DisableMouseCapture);
    let _ = std::io::stderr().flush();

    // 2. Give terminal time to process mouse-disable commands
    std::thread::sleep(std::time::Duration::from_millis(20));

    // 3. Drain any queued input events
    let mut drain_count = 0;
    while event::poll(std::time::Duration::from_millis(0)).unwrap_or(false) && drain_count < 100 {
        let _ = event::read();
        drain_count += 1;
    }

    // 4. Disable raw mode
    let _ = disable_raw_mode();

    // 5. Move cursor to the first row of our inline viewport and erase downward.
    //    This removes every line we drew, leaving no visual artifacts.
    let start_row = COMPACT_START_ROW.load(Ordering::Relaxed);
    let _ = std::io::stderr().execute(MoveTo(0, start_row));
    let _ = write!(std::io::stderr(), "\x1b[0J"); // clear from cursor to end of screen

    // 6. Second event drain after mode changes
    std::thread::sleep(std::time::Duration::from_millis(10));
    let mut drain_count2 = 0;
    while event::poll(std::time::Duration::from_millis(0)).unwrap_or(false) && drain_count2 < 50 {
        let _ = event::read();
        drain_count2 += 1;
    }

    // 7. Reset attributes and show cursor
    let _ = write!(std::io::stderr(), "\x1b[0m\x1b[?25h");
    let _ = std::io::stderr().flush();

    Ok(())
}

pub fn cleanup_terminal() -> Result<()> {
    use crossterm::terminal::{Clear, ClearType};
    use std::io::Write;

    // Restore terminal state in reverse order of setup

    // 1. CRITICAL: Explicitly disable ALL mouse tracking modes
    //    This is more thorough than just DisableMouseCapture
    let _ = write!(std::io::stderr(), "\x1b[?1000l"); // Disable X10 mouse
    let _ = write!(std::io::stderr(), "\x1b[?1002l"); // Disable cell motion
    let _ = write!(std::io::stderr(), "\x1b[?1003l"); // Disable all motion
    let _ = write!(std::io::stderr(), "\x1b[?1006l"); // Disable SGR mode
    let _ = write!(std::io::stderr(), "\x1b[?1015l"); // Disable urxvt mode
    let _ = std::io::stderr().execute(DisableMouseCapture);
    let _ = std::io::stderr().flush();

    // 2. Give terminal MORE time to process mouse disable commands
    //    Increased to 20ms to handle slow terminals
    std::thread::sleep(std::time::Duration::from_millis(20));

    // 3. First aggressive drain of pending events
    let mut drain_count = 0;
    while event::poll(std::time::Duration::from_millis(0)).unwrap_or(false) && drain_count < 100 {
        let _ = event::read();
        drain_count += 1;
    }

    // 4. Clear alternate screen before leaving it
    let _ = std::io::stderr().execute(Clear(ClearType::All));
    let _ = std::io::stderr().flush();

    // 5. Leave alternate screen
    let _ = std::io::stderr().execute(LeaveAlternateScreen);
    let _ = std::io::stderr().flush();

    // 6. IMPORTANT: Another delay + drain AFTER leaving alternate screen
    //    Sometimes events leak during the screen transition
    std::thread::sleep(std::time::Duration::from_millis(10));

    let mut drain_count2 = 0;
    while event::poll(std::time::Duration::from_millis(0)).unwrap_or(false) && drain_count2 < 50 {
        let _ = event::read();
        drain_count2 += 1;
    }

    // 7. Disable raw mode (this should stop all special terminal modes)
    let _ = disable_raw_mode();

    // 8. Send minimal reset sequences (no screen clearing!)
    //    Reset character attributes (SGR 0)
    let _ = write!(std::io::stderr(), "\x1b[0m");
    //    Show cursor
    let _ = write!(std::io::stderr(), "\x1b[?25h");
    let _ = std::io::stderr().flush();

    // 9. Final delay to ensure terminal processes everything
    std::thread::sleep(std::time::Duration::from_millis(10));

    Ok(())
}

pub fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stderr>>,
    app: &mut App,
) -> Result<Option<PathBuf>> {
    loop {
        // Only render when needed (dirty flag optimization)
        if app.needs_redraw() {
            terminal.draw(|f| app.render(f))?;
            app.clear_dirty();
        }

        // Wait up to 8ms for the first event; on timeout poll async updates and continue
        if !event::poll(std::time::Duration::from_millis(8))? {
            let _ = app.poll_search();
            continue;
        }

        // Drain all accumulated events before next render.
        // Scroll events are coalesced: only the last scroll event per direction
        // is applied, preventing jumpy navigation when the OS buffers multiple
        // wheel ticks before the next render frame.
        let mut scroll_up_event: Option<MouseEvent> = None;
        let mut scroll_down_event: Option<MouseEvent> = None;
        loop {
            if event::poll(std::time::Duration::from_millis(0))? {
                match event::read()? {
                    Event::Key(key) => {
                        if matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                            match app.handle_key(key)? {
                                Some(path) if !path.as_os_str().is_empty() => {
                                    return Ok(Some(path));
                                }
                                None => {
                                    return Ok(None);
                                }
                                _ => {}
                            }
                        }
                    }
                    Event::Mouse(mouse) => match mouse.kind {
                        MouseEventKind::ScrollUp => scroll_up_event = Some(mouse),
                        MouseEventKind::ScrollDown => scroll_down_event = Some(mouse),
                        _ => {
                            let _ = app.handle_mouse(mouse);
                        }
                    },
                    Event::Resize(_width, _height) => {
                        app.mark_dirty();
                    }
                    _ => {}
                }
            } else {
                break;
            }
        }
        if let Some(mouse) = scroll_up_event {
            let _ = app.handle_mouse(mouse);
        }
        if let Some(mouse) = scroll_down_event {
            let _ = app.handle_mouse(mouse);
        }
    }
}
