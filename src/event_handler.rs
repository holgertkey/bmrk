// Allow many arguments for event handler functions
#![allow(clippy::too_many_arguments)]

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use crate::bookmarks::Bookmarks;
use crate::config::Config;
use crate::disks::Disks;
use crate::navigation::Navigation;
use crate::search::Search;
use crate::ui::UI;

/// Event handler for keyboard and mouse input
pub struct EventHandler {
    pub last_click_time: Option<(Instant, usize)>,
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            last_click_time: None,
        }
    }

    /// Handle keyboard events
    pub fn handle_key(
        &mut self,
        key: KeyEvent,
        nav: &mut Navigation,
        search: &mut Search,
        bookmarks: &mut Bookmarks,
        disks: &mut Disks,
        ui: &UI,
        config: &Config,
    ) -> Result<Option<PathBuf>> {
        // Search input mode
        if search.mode {
            return self.handle_search_input(key, search, nav);
        }

        // Disk selection mode
        if disks.is_selecting {
            match key.code {
                KeyCode::Esc => {
                    disks.exit_selection_mode();
                    return Ok(Some(PathBuf::new()));
                }
                KeyCode::Enter => {
                    if let Some(disk) = disks.get_selected() {
                        let path = disk.mount_point.clone();
                        disks.exit_selection_mode();
                        let _ = nav.go_to_directory(path, false);
                    } else {
                        disks.exit_selection_mode();
                    }
                    return Ok(Some(PathBuf::new()));
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    disks.move_down();
                    return Ok(Some(PathBuf::new()));
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    disks.move_up();
                    return Ok(Some(PathBuf::new()));
                }
                _ => return Ok(Some(PathBuf::new())),
            }
        }

        // Bookmark selection mode
        if bookmarks.is_selecting {
            match key.code {
                KeyCode::Esc => {
                    bookmarks.exit_selection_mode();
                    return Ok(Some(PathBuf::new()));
                }
                KeyCode::Tab => {
                    bookmarks.toggle_filter_mode();
                    return Ok(Some(PathBuf::new()));
                }
                KeyCode::Enter => {
                    if let Some(bookmark) = bookmarks.get_selected_bookmark() {
                        let path = bookmark.path.clone();
                        bookmarks.exit_selection_mode();
                        let _ = nav.go_to_directory(path, false);
                    } else {
                        bookmarks.exit_selection_mode();
                    }
                    return Ok(Some(PathBuf::new()));
                }
                KeyCode::Char('j') | KeyCode::Down if !bookmarks.filter_mode => {
                    bookmarks.move_down();
                    return Ok(Some(PathBuf::new()));
                }
                KeyCode::Char('k') | KeyCode::Up if !bookmarks.filter_mode => {
                    bookmarks.move_up();
                    return Ok(Some(PathBuf::new()));
                }
                KeyCode::Char('d') if !bookmarks.filter_mode => {
                    let _ = bookmarks.handle_deletion_key();
                    return Ok(Some(PathBuf::new()));
                }
                KeyCode::Char(c) if bookmarks.filter_mode => {
                    bookmarks.add_char(c);
                    return Ok(Some(PathBuf::new()));
                }
                KeyCode::Backspace if bookmarks.filter_mode => {
                    bookmarks.backspace();
                    return Ok(Some(PathBuf::new()));
                }
                _ => return Ok(Some(PathBuf::new())),
            }
        }

        // Bookmark creation mode
        if bookmarks.is_creating {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match key.code {
                    KeyCode::Char('j') | KeyCode::Char('J') | KeyCode::Down => {
                        let max_visible = if ui.bottom_panel_height >= 5 {
                            (ui.bottom_panel_height as usize).saturating_sub(5).max(1)
                        } else {
                            (ui.terminal_height as usize).saturating_sub(2).max(1)
                        };
                        bookmarks.scroll_down(max_visible);
                        return Ok(Some(PathBuf::new()));
                    }
                    KeyCode::Char('k') | KeyCode::Char('K') | KeyCode::Up => {
                        bookmarks.scroll_up();
                        return Ok(Some(PathBuf::new()));
                    }
                    _ => {}
                }
            }

            match key.code {
                KeyCode::Esc => {
                    bookmarks.exit_creation_mode();
                    return Ok(Some(PathBuf::new()));
                }
                KeyCode::Enter => {
                    let bookmark_name = bookmarks.get_input().to_string();
                    if !bookmark_name.is_empty() {
                        if let Some(node) = nav.get_selected_node() {
                            let node_borrowed = node.borrow();
                            let path = if node_borrowed.is_dir {
                                node_borrowed.path.clone()
                            } else {
                                node_borrowed
                                    .path
                                    .parent()
                                    .map(|p| p.to_path_buf())
                                    .unwrap_or_else(|| node_borrowed.path.clone())
                            };
                            let dir_name = path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .map(|s| s.to_string());
                            drop(node_borrowed);
                            let _ = bookmarks.add(bookmark_name, path, dir_name);
                        }
                    }
                    bookmarks.exit_creation_mode();
                    return Ok(Some(PathBuf::new()));
                }
                KeyCode::Char(c) => {
                    bookmarks.add_char(c);
                    return Ok(Some(PathBuf::new()));
                }
                KeyCode::Backspace => {
                    bookmarks.backspace();
                    return Ok(Some(PathBuf::new()));
                }
                _ => return Ok(Some(PathBuf::new())),
            }
        }

        // Esc — cancel search/exit
        if matches!(key.code, KeyCode::Esc) {
            if search.is_active() {
                search.cancel_search();
                return Ok(Some(PathBuf::new()));
            } else if search.show_results {
                search.close_results();
                return Ok(Some(PathBuf::new()));
            } else {
                return Ok(None);
            }
        }

        // q — exit, output path of selected directory for shell
        if matches!(key.code, KeyCode::Char('q') | KeyCode::Char('Q')) {
            if let Some(node) = nav.get_selected_node() {
                let node_borrowed = node.borrow();
                if node_borrowed.is_dir {
                    return Ok(Some(node_borrowed.path.clone()));
                } else if let Some(parent) = node_borrowed.path.parent() {
                    return Ok(Some(parent.to_path_buf()));
                }
            }
            return Ok(None);
        }

        match key.code {
            _ if config.keybindings.is_search(key.code) => {
                search.enter_mode();
                return Ok(Some(PathBuf::new()));
            }
            KeyCode::Tab => {
                search.toggle_focus();
                return Ok(Some(PathBuf::new()));
            }
            KeyCode::Char('j') | KeyCode::Down => {
                if search.focus_on_results {
                    search.move_down();
                } else {
                    nav.move_down();
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if search.focus_on_results {
                    search.move_up();
                } else {
                    nav.move_up();
                }
            }
            KeyCode::Enter => {
                if search.focus_on_results && search.show_results {
                    if let Some(path) = search.get_selected_result() {
                        let _ = nav.expand_path_to_node(&path, false);
                        search.focus_on_results = false;
                    }
                    return Ok(Some(PathBuf::new()));
                } else if let Some(node) = nav.get_selected_node() {
                    let node_borrowed = node.borrow();
                    if node_borrowed.is_dir {
                        let path = node_borrowed.path.clone();
                        drop(node_borrowed);
                        let _ = nav.go_to_directory(path, false);
                    }
                }
            }
            KeyCode::Char('l') | KeyCode::Right if !search.focus_on_results => {
                if let Some(node) = nav.get_selected_node() {
                    let node_borrowed = node.borrow();
                    if node_borrowed.is_dir {
                        let path = node_borrowed.path.clone();
                        drop(node_borrowed);
                        let _ = nav.toggle_node(&path, false);
                    }
                }
            }
            KeyCode::Char('h') | KeyCode::Left => {
                if let Some(node) = nav.get_selected_node() {
                    let node_borrowed = node.borrow();
                    if node_borrowed.is_dir {
                        let path = node_borrowed.path.clone();
                        drop(node_borrowed);
                        let _ = nav.toggle_node(&path, false)?;
                    }
                }
            }
            KeyCode::Char('u') | KeyCode::Backspace => {
                nav.go_to_parent(false)?;
            }
            _ if config.keybindings.is_create_bookmark(key.code) => {
                bookmarks.enter_creation_mode();
            }
            _ if config.keybindings.is_select_bookmark(key.code) => {
                bookmarks.enter_selection_mode();
            }
            _ if config.keybindings.is_select_disk(key.code) => {
                let current_path = nav.root.borrow().path.clone();
                disks.enter_selection_mode(Some(&current_path));
            }
            _ => {}
        }

        Ok(Some(PathBuf::new()))
    }

    fn handle_search_input(
        &mut self,
        key: KeyEvent,
        search: &mut Search,
        nav: &Navigation,
    ) -> Result<Option<PathBuf>> {
        match key.code {
            KeyCode::Esc => {
                search.exit_mode();
                Ok(Some(PathBuf::new()))
            }
            KeyCode::Enter => {
                search.perform_search(&nav.root, false, nav.show_hidden, nav.follow_symlinks);
                Ok(Some(PathBuf::new()))
            }
            KeyCode::Char(c) => {
                search.add_char(c);
                Ok(Some(PathBuf::new()))
            }
            KeyCode::Backspace => {
                search.backspace();
                Ok(Some(PathBuf::new()))
            }
            _ => Ok(Some(PathBuf::new())),
        }
    }

    /// Handle mouse events
    pub fn handle_mouse(
        &mut self,
        mouse: MouseEvent,
        nav: &mut Navigation,
        search: &mut Search,
        bookmarks: &mut Bookmarks,
        disks: &mut Disks,
        ui: &mut UI,
        config: &Config,
    ) -> Result<()> {
        match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                self.handle_mouse_click(mouse, nav, disks, ui, config)?;
            }
            MouseEventKind::ScrollUp => {
                self.handle_scroll_up(mouse, nav, search, bookmarks, disks, ui, config)?;
            }
            MouseEventKind::ScrollDown => {
                self.handle_scroll_down(mouse, nav, search, bookmarks, disks, ui, config)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_mouse_click(
        &mut self,
        mouse: MouseEvent,
        nav: &mut Navigation,
        disks: &mut Disks,
        ui: &UI,
        config: &Config,
    ) -> Result<()> {
        if disks.is_selecting {
            if mouse.row >= ui.tree_area_top
                && mouse.row < ui.tree_area_top + ui.tree_area_height
            {
                let clicked_row_visible = mouse.row.saturating_sub(ui.tree_area_top) as usize;
                let clicked_disk = clicked_row_visible + ui.disk_scroll_offset;

                if clicked_disk < disks.disks.len() {
                    let now = Instant::now();
                    let is_double_click = if let Some((last_time, last_idx)) = self.last_click_time
                    {
                        clicked_disk == last_idx
                            && now.duration_since(last_time)
                                < Duration::from_millis(config.behavior.double_click_timeout_ms)
                    } else {
                        false
                    };

                    if is_double_click {
                        let path = disks.disks[clicked_disk].mount_point.clone();
                        disks.exit_selection_mode();
                        let _ = nav.go_to_directory(path, false);
                        self.last_click_time = None;
                    } else {
                        disks.selected_index = clicked_disk;
                        self.last_click_time = Some((now, clicked_disk));
                    }
                }
            }
            return Ok(());
        }

        if mouse.column >= ui.tree_area_start
            && mouse.column < ui.tree_area_end
            && mouse.row >= ui.tree_area_top
            && mouse.row < ui.tree_area_top + ui.tree_area_height
        {
            let clicked_row_visible = mouse.row.saturating_sub(ui.tree_item_top) as usize;
            let clicked_row = clicked_row_visible + ui.tree_scroll_offset;

            if clicked_row < nav.flat_list.len() {
                let now = Instant::now();
                let is_double_click =
                    if let Some((last_time, last_idx)) = self.last_click_time {
                        clicked_row == last_idx
                            && now.duration_since(last_time)
                                < Duration::from_millis(
                                    config.behavior.double_click_timeout_ms,
                                )
                    } else {
                        false
                    };

                if is_double_click {
                    let node = &nav.flat_list[clicked_row];
                    let node_borrowed = node.borrow();
                    if node_borrowed.is_dir {
                        let path = node_borrowed.path.clone();
                        drop(node_borrowed);
                        let _ = nav.toggle_node(&path, false);
                    }
                    self.last_click_time = None;
                } else {
                    nav.selected = clicked_row;
                    self.last_click_time = Some((now, clicked_row));
                }
            }
        }
        Ok(())
    }

    fn handle_scroll_up(
        &mut self,
        _mouse: MouseEvent,
        nav: &mut Navigation,
        search: &mut Search,
        bookmarks: &mut Bookmarks,
        disks: &mut Disks,
        ui: &UI,
        config: &Config,
    ) -> Result<()> {
        if disks.is_selecting {
            disks.move_up();
            return Ok(());
        }
        // Bottom panel scrolling (bookmarks/search in non-compact layout)
        if ui.bottom_panel_height > 0 {
            if search.show_results {
                search.move_up();
                return Ok(());
            }
            if bookmarks.is_selecting {
                bookmarks.move_up();
                return Ok(());
            }
            if bookmarks.is_creating {
                bookmarks.scroll_up();
                return Ok(());
            }
        }
        for _ in 0..config.behavior.mouse_scroll_lines {
            nav.move_up();
        }
        Ok(())
    }

    fn handle_scroll_down(
        &mut self,
        _mouse: MouseEvent,
        nav: &mut Navigation,
        search: &mut Search,
        bookmarks: &mut Bookmarks,
        disks: &mut Disks,
        ui: &UI,
        config: &Config,
    ) -> Result<()> {
        if disks.is_selecting {
            disks.move_down();
            return Ok(());
        }
        if ui.bottom_panel_height > 0 {
            if search.show_results {
                search.move_down();
                return Ok(());
            }
            if bookmarks.is_selecting {
                bookmarks.move_down();
                return Ok(());
            }
            if bookmarks.is_creating {
                let max_visible = 10;
                bookmarks.scroll_down(max_visible);
                return Ok(());
            }
        }
        for _ in 0..config.behavior.mouse_scroll_lines {
            if nav.selected < nav.flat_list.len().saturating_sub(1) {
                nav.move_down();
            }
        }
        Ok(())
    }
}
