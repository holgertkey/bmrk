use anyhow::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::Frame;
use std::path::PathBuf;

use crate::bookmarks::Bookmarks;
use crate::config::Config;
use crate::disks::Disks;
use crate::event_handler::EventHandler;
use crate::navigation::Navigation;
use crate::search::Search;
use crate::ui::UI;

/// Main application state
pub struct App {
    nav: Navigation,
    search: Search,
    ui: UI,
    event_handler: EventHandler,
    config: Config,
    pub bookmarks: Bookmarks,
    pub disks: Disks,
    needs_redraw: bool,
}

impl App {
    pub fn new(start_path: PathBuf) -> Result<Self> {
        let config = Config::load()?;

        let nav = Navigation::new(
            start_path,
            false,
            config.behavior.show_hidden,
            config.behavior.follow_symlinks,
        )?;
        let search = Search::new();
        let ui = UI::new();
        let event_handler = EventHandler::new();
        let bookmarks = Bookmarks::new()?;
        let disks = Disks::new();

        Ok(App {
            nav,
            search,
            ui,
            event_handler,
            config,
            bookmarks,
            disks,
            needs_redraw: true,
        })
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Result<Option<PathBuf>> {
        let result = self.event_handler.handle_key(
            key,
            &mut self.nav,
            &mut self.search,
            &mut self.bookmarks,
            &mut self.disks,
            &self.ui,
            &self.config,
        );
        self.mark_dirty();
        result
    }

    pub fn handle_mouse(&mut self, mouse: MouseEvent) -> Result<()> {
        let result = self.event_handler.handle_mouse(
            mouse,
            &mut self.nav,
            &mut self.search,
            &mut self.bookmarks,
            &mut self.disks,
            &mut self.ui,
            &self.config,
        );
        self.mark_dirty();
        result
    }

    pub fn render(&mut self, frame: &mut Frame) {
        self.ui.render_compact(
            frame,
            &self.nav,
            &self.search,
            &self.bookmarks,
            &self.disks,
            &self.config,
        );
    }

    /// Poll search results from background thread.
    /// Returns true if there were updates requiring a redraw.
    pub fn poll_search(&mut self) -> bool {
        let updated = self.search.poll_results();
        if updated {
            self.mark_dirty();
        }
        updated
    }

    /// Mark app as needing redraw
    pub fn mark_dirty(&mut self) {
        self.needs_redraw = true;
    }

    /// Clear dirty flag after rendering
    pub fn clear_dirty(&mut self) {
        self.needs_redraw = false;
    }

    /// Check if app needs to be redrawn
    pub fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn test_bookmark_create_enters_creation_mode() {
        let temp_dir = std::env::temp_dir().join("bmrk_test_bm_create");
        std::fs::create_dir_all(&temp_dir).unwrap();

        let mut app = App::new(temp_dir.clone()).unwrap();

        // Press 'm' to enter bookmark creation mode
        let key_m = KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE);
        let result = app.handle_key(key_m).unwrap();

        // Must NOT return None (exit signal) - returns Some(empty) to stay in app
        assert!(result.is_some(), "pressing 'm' should not exit the app");

        // Bookmark creation mode must be active
        assert!(app.bookmarks.is_creating);

        // Press Esc to cancel
        let key_esc = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        let _ = app.handle_key(key_esc);
        assert!(!app.bookmarks.is_creating);

        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_search_mode_activation() {
        let temp_dir = std::env::temp_dir().join("bmrk_test_search");
        std::fs::create_dir_all(&temp_dir).unwrap();

        let mut app = App::new(temp_dir.clone()).unwrap();

        // Press '/' to enter search mode
        let key_slash = KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE);
        let result = app.handle_key(key_slash).unwrap();
        assert!(result.is_some(), "entering search should not exit the app");
        assert!(app.search.mode, "search mode should be active");

        // Type a character
        let key_r = KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE);
        let _ = app.handle_key(key_r);

        // Press Enter to execute search
        let key_enter = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        let result = app.handle_key(key_enter).unwrap();
        assert!(result.is_some(), "search execution should not exit the app");
        assert!(app.search.show_results, "search results should be shown");

        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_bookmark_select_enters_selection_mode() {
        let temp_dir = std::env::temp_dir().join("bmrk_test_bm_select");
        std::fs::create_dir_all(&temp_dir).unwrap();

        let mut app = App::new(temp_dir.clone()).unwrap();

        // Press '\'' to enter bookmark selection mode
        let key_tick = KeyEvent::new(KeyCode::Char('\''), KeyModifiers::NONE);
        let result = app.handle_key(key_tick).unwrap();
        assert!(
            result.is_some(),
            "bookmark selection should not exit the app"
        );
        assert!(app.bookmarks.is_selecting);

        // Press Esc to cancel
        let key_esc = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        let _ = app.handle_key(key_esc);
        assert!(!app.bookmarks.is_selecting);

        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_esc_exits_app() {
        let temp_dir = std::env::temp_dir().join("bmrk_test_esc");
        std::fs::create_dir_all(&temp_dir).unwrap();

        let mut app = App::new(temp_dir.clone()).unwrap();

        // Press Esc in normal mode should return None (exit signal)
        let key_esc = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        let result = app.handle_key(key_esc).unwrap();
        assert!(result.is_none(), "Esc in normal mode should signal exit");

        std::fs::remove_dir_all(&temp_dir).ok();
    }
}
