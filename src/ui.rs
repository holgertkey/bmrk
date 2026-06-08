// Allow many arguments for UI render functions
#![allow(clippy::too_many_arguments)]

use crate::bookmarks::Bookmarks;
use crate::config::Config;
use crate::disks::Disks;
use crate::navigation::Navigation;
use crate::search::Search;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Paragraph},
    Frame,
};
fn truncate_name_middle(name: &str, max_len: usize) -> String {
    if max_len == 0 {
        return name.to_string();
    }
    let char_count = name.chars().count();
    if char_count <= max_len || max_len < 4 {
        return name.to_string();
    }
    let available = max_len - 3;
    let first_len = available.div_ceil(2);
    let last_len = available - first_len;
    let first: String = name.chars().take(first_len).collect();
    let last: String = name
        .chars()
        .rev()
        .take(last_len)
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    format!("{}...{}", first, last)
}

/// Word-wrap a comma-separated bookmark key string to `width` columns.
/// Returns lines joined with `\n`; the first line has a leading space.
fn wrap_bookmark_keys(keys_str: &str, width: usize) -> String {
    if keys_str.is_empty() || width == 0 {
        return String::new();
    }
    let mut lines: Vec<String> = Vec::new();
    let mut current = String::from(" ");
    for (i, word) in keys_str.split_ascii_whitespace().enumerate() {
        if i == 0 {
            current.push_str(word);
        } else if current.len() + 1 + word.len() <= width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(std::mem::take(&mut current));
            current.push_str(word);
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines.join("\n")
}

/// UI rendering state
pub struct UI {
    pub tree_area_start: u16,
    pub tree_area_end: u16,
    pub tree_area_top: u16,
    pub tree_area_height: u16,
    /// Absolute screen row of the first tree item (no border in compact mode).
    pub tree_item_top: u16,
    pub terminal_width: u16,
    pub terminal_height: u16,
    pub tree_scroll_offset: usize,
    /// Scroll offset of the disk list, updated each render when disk selection is active.
    pub disk_scroll_offset: usize,
    pub bottom_panel_height: u16,
    /// Visible row count of the bookmark list in creation mode, set each render.
    pub bookmark_panel_height: usize,
    /// Scroll offset of the bookmark selection list, updated each render.
    pub bookmark_scroll_offset: usize,
    /// Scroll offset of the search results list, updated each render.
    pub search_scroll_offset: usize,
}

impl Default for UI {
    fn default() -> Self {
        Self::new()
    }
}

impl UI {
    pub fn new() -> Self {
        Self {
            tree_area_start: 0,
            tree_area_end: 0,
            tree_area_top: 0,
            tree_area_height: 0,
            tree_item_top: 0,
            terminal_width: 0,
            terminal_height: 0,
            tree_scroll_offset: 0,
            disk_scroll_offset: 0,
            bottom_panel_height: 0,
            bookmark_panel_height: 0,
            bookmark_scroll_offset: 0,
            search_scroll_offset: 0,
        }
    }

    /// Render the compact inline viewport (COMPACT_HEIGHT rows, no alternate screen).
    /// Layout:
    ///   Row 0   — header: root path + key hints (adapts to mode)
    ///   Rows 1+ — tree items, or bookmark/search/disk list when active
    ///   Last row — search bar / bookmark input (only when active)
    pub fn render_compact(
        &mut self,
        frame: &mut Frame,
        nav: &Navigation,
        search: &Search,
        bookmarks: &Bookmarks,
        disks: &Disks,
        config: &Config,
    ) {
        let area = frame.area();
        self.terminal_width = area.width;
        self.terminal_height = area.height;

        let selected_color =
            Config::parse_color(Config::get_color(&config.appearance.colors.selected_color));

        // Reserve the last row: search bar, bookmark input, or nothing
        let (content_area, search_bar_area, bookmark_input_area) = if search.mode {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(2), Constraint::Length(1)])
                .split(area);
            (chunks[0], Some(chunks[1]), None)
        } else if bookmarks.is_creating {
            let header_rows: u16 = 1;
            let input_rows: u16 = 1;
            let available = area.height.saturating_sub(header_rows + input_rows);
            let body_rows: u16 = if bookmarks.list().is_empty() {
                0
            } else {
                let keys = bookmarks
                    .list()
                    .iter()
                    .map(|b| b.key.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                let wrap_width = if config.appearance.max_name_length > 0 {
                    config.appearance.max_name_length
                } else {
                    area.width as usize
                };
                let wrapped = wrap_bookmark_keys(&keys, wrap_width);
                let line_count = wrapped.lines().count().max(1);
                (line_count as u16).min(available).max(1)
            };
            let content_height = header_rows + body_rows;
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(content_height),
                    Constraint::Length(input_rows),
                    Constraint::Min(0),
                ])
                .split(area);
            (chunks[0], None, Some(chunks[1]))
        } else {
            (area, None, None)
        };

        // Split content into header (1 row) + body (rest)
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(content_area);

        let header_area = chunks[0];
        let body_area = chunks[1];

        // Store body area geometry for mouse handling
        self.tree_area_top = body_area.y;
        self.tree_area_height = body_area.height;
        self.tree_area_start = body_area.x;
        self.tree_area_end = body_area.x + body_area.width;
        self.tree_item_top = body_area.y;

        // --- Header ---
        let root_path = nav.root.borrow().path.display().to_string();
        let header_path_color = Config::parse_color(Config::get_color(
            &config.appearance.colors.header_path_color,
        ));
        let header_hints_color = Config::parse_color(Config::get_color(
            &config.appearance.colors.header_hints_color,
        ));

        if disks.is_selecting {
            let hints = "  jk:select  Enter:go  Esc:cancel";
            let label = if disks.disks.is_empty() {
                "Disks (none found)".to_string()
            } else {
                format!("Disks ({}/{})", disks.selected_index + 1, disks.disks.len())
            };
            let max_label_len = (area.width as usize).saturating_sub(hints.len() + 4).max(8);
            let label_display = if label.len() > max_label_len {
                format!("...{}", &label[label.len().saturating_sub(max_label_len)..])
            } else {
                label
            };
            frame.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::styled(" > ", Style::default().fg(Color::Yellow)),
                    Span::styled(
                        label_display,
                        Style::default()
                            .fg(header_path_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(hints, Style::default().fg(header_hints_color)),
                ])),
                header_area,
            );
        } else if bookmarks.is_selecting {
            let hints = if bookmarks.filter_mode {
                "  jk:select  Enter:jump  Tab:nav  Esc:cancel"
            } else {
                "  jk:select  Enter:jump  q:exit  Tab:filter  dd:del  Esc:cancel"
            };
            let filtered = bookmarks.get_filtered_bookmarks();
            let label = if bookmarks.filter_mode {
                format!("Filter: {}", bookmarks.get_input())
            } else if filtered.is_empty() {
                "Bookmarks (empty)".to_string()
            } else {
                format!(
                    "Bookmarks ({}/{})",
                    bookmarks.selected_index + 1,
                    filtered.len()
                )
            };
            let max_label_len = (area.width as usize).saturating_sub(hints.len() + 4).max(8);
            let label_display = if label.len() > max_label_len {
                format!("...{}", &label[label.len().saturating_sub(max_label_len)..])
            } else {
                label
            };
            frame.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::styled(" > ", Style::default().fg(Color::Yellow)),
                    Span::styled(
                        label_display,
                        Style::default()
                            .fg(header_path_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(hints, Style::default().fg(header_hints_color)),
                ])),
                header_area,
            );
        } else if search.show_results && search.focus_on_results {
            let hints = if search.is_searching {
                "  jk:select  Enter:jump  q:exit  Esc:cancel"
            } else {
                "  jk:select  Enter:jump  q:exit  Esc:close"
            };
            let label = if search.is_searching {
                format!(
                    "Search: {} found | Scanning {} dirs...",
                    search.results.len(),
                    search.scanned_count
                )
            } else if search.results.is_empty() {
                "Search: no results".to_string()
            } else {
                format!("Search: {}/{}", search.selected + 1, search.results.len())
            };
            let max_label_len = (area.width as usize).saturating_sub(hints.len() + 4).max(8);
            let label_display = if label.len() > max_label_len {
                format!("...{}", &label[label.len().saturating_sub(max_label_len)..])
            } else {
                label
            };
            frame.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::styled(" * ", Style::default().fg(Color::Yellow)),
                    Span::styled(
                        label_display,
                        Style::default()
                            .fg(header_path_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(hints, Style::default().fg(header_hints_color)),
                ])),
                header_area,
            );
        } else if let Some(ref error_msg) = nav.nav_error {
            let hints = "  hjkl:nav  u:up  m:bmark  ':jump  d:disk  /:search  q:exit";
            let max_msg_len = (area.width as usize).saturating_sub(hints.len() + 4).max(8);
            let msg_display = if error_msg.len() > max_msg_len {
                format!(
                    "...{}",
                    &error_msg[error_msg.len().saturating_sub(max_msg_len)..]
                )
            } else {
                error_msg.clone()
            };
            let error_icon = if config.appearance.icons != "ascii" {
                " ✗ "
            } else {
                " ! "
            };
            frame.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::styled(error_icon, Style::default().fg(Color::Red)),
                    Span::styled(
                        msg_display,
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(hints, Style::default().fg(header_hints_color)),
                ])),
                header_area,
            );
        } else {
            let hints = if bookmarks.is_creating {
                "  Enter:save  Ctrl+j/k:scroll  Esc:cancel"
            } else {
                "  hjkl:nav  u:up  m:bmark  ':jump  d:disk  /:search  q:exit"
            };
            let total = nav.flat_list.len();
            let count_str = if total > 0 {
                format!(" ({}/{})", nav.selected + 1, total)
            } else {
                String::new()
            };
            let active_path = if bookmarks.is_creating {
                nav.get_selected_node()
                    .map(|n| n.borrow().path.display().to_string())
                    .unwrap_or(root_path)
            } else {
                root_path
            };
            let max_path_len = (area.width as usize)
                .saturating_sub(hints.len() + count_str.len() + 4)
                .max(8);
            let path_display = if active_path.len() > max_path_len {
                format!(
                    "...{}",
                    &active_path[active_path.len().saturating_sub(max_path_len)..]
                )
            } else {
                active_path
            };
            let header_icon = if config.appearance.icons != "ascii" {
                " ▼ "
            } else {
                " v "
            };
            frame.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::styled(
                        header_icon,
                        Style::default().fg(Config::parse_color(Config::get_color(
                            &config.appearance.colors.directory_color,
                        ))),
                    ),
                    Span::styled(
                        path_display,
                        Style::default()
                            .fg(header_path_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        count_str,
                        Style::default()
                            .fg(header_path_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(hints, Style::default().fg(header_hints_color)),
                ])),
                header_area,
            );
        }

        // --- Body ---
        if disks.is_selecting {
            let file_color =
                Config::parse_color(Config::get_color(&config.appearance.colors.file_color));
            let cursor_color_str = Config::get_color(&config.appearance.colors.cursor_color);
            let highlight_style = if cursor_color_str.to_lowercase() == "dim" {
                Style::default().add_modifier(Modifier::DIM)
            } else {
                Style::default()
                    .fg(Config::parse_color(cursor_color_str))
                    .add_modifier(Modifier::BOLD)
            };

            if disks.disks.is_empty() {
                frame.render_widget(
                    Paragraph::new("No disks found.").style(Style::default().fg(Color::DarkGray)),
                    body_area,
                );
            } else {
                let items: Vec<ListItem> = disks
                    .disks
                    .iter()
                    .map(|d| ListItem::new(d.display_line()).style(Style::default().fg(file_color)))
                    .collect();

                let mut state = ListState::default();
                state.select(Some(disks.selected_index));

                let visible = body_area.height as usize;
                let total = disks.disks.len();
                let offset = if disks.center_selection {
                    if disks.selected_index < visible / 2 {
                        0
                    } else if disks.selected_index >= total.saturating_sub(visible / 2) {
                        total.saturating_sub(visible)
                    } else {
                        disks.selected_index.saturating_sub(visible / 2)
                    }
                } else {
                    let current = self.disk_scroll_offset;
                    if disks.selected_index < current {
                        disks.selected_index
                    } else if visible > 0 && disks.selected_index >= current + visible {
                        disks.selected_index.saturating_sub(visible - 1)
                    } else {
                        current
                    }
                };
                *state.offset_mut() = offset;
                self.disk_scroll_offset = offset;

                frame.render_stateful_widget(
                    List::new(items)
                        .highlight_style(highlight_style)
                        .highlight_symbol(">> "),
                    body_area,
                    &mut state,
                );
            }
        } else if bookmarks.is_selecting {
            let filtered = bookmarks.get_filtered_bookmarks();
            let file_color =
                Config::parse_color(Config::get_color(&config.appearance.colors.file_color));
            let error_color =
                Config::parse_color(Config::get_color(&config.appearance.colors.error_color));

            if filtered.is_empty() {
                let msg = if bookmarks.filter_mode {
                    format!("No bookmarks match '{}'", bookmarks.get_input())
                } else {
                    "No bookmarks. Press 'm' to create one.".to_string()
                };
                frame.render_widget(
                    Paragraph::new(msg).style(Style::default().fg(Color::DarkGray)),
                    body_area,
                );
            } else {
                let items: Vec<ListItem> = filtered
                    .iter()
                    .enumerate()
                    .map(|(idx, bookmark)| {
                        let name = bookmark.name.as_deref().unwrap_or("(unnamed)");
                        let path_str = bookmark.path.display().to_string();
                        let is_marked = bookmarks.pending_deletion_index == Some(idx);
                        let text = if is_marked {
                            format!("[DEL] {:<10} {} ({})", bookmark.key, name, path_str)
                        } else {
                            format!("{:<10} {} ({})", bookmark.key, name, path_str)
                        };
                        let style = if is_marked {
                            Style::default().fg(error_color)
                        } else {
                            Style::default().fg(file_color)
                        };
                        ListItem::new(text).style(style)
                    })
                    .collect();

                let mut state = ListState::default();
                state.select(Some(bookmarks.selected_index));

                let visible = body_area.height as usize;
                let total = filtered.len();
                let offset = if bookmarks.center_selection {
                    if bookmarks.selected_index < visible / 2 {
                        0
                    } else if bookmarks.selected_index >= total.saturating_sub(visible / 2) {
                        total.saturating_sub(visible)
                    } else {
                        bookmarks.selected_index.saturating_sub(visible / 2)
                    }
                } else {
                    let current = self.bookmark_scroll_offset;
                    if bookmarks.selected_index < current {
                        bookmarks.selected_index
                    } else if visible > 0 && bookmarks.selected_index >= current + visible {
                        bookmarks.selected_index.saturating_sub(visible - 1)
                    } else {
                        current
                    }
                };
                *state.offset_mut() = offset;
                self.bookmark_scroll_offset = offset;

                let cursor_color_str = Config::get_color(&config.appearance.colors.cursor_color);
                let highlight_style = if cursor_color_str.to_lowercase() == "dim" {
                    Style::default().add_modifier(Modifier::DIM)
                } else {
                    Style::default()
                        .fg(Config::parse_color(cursor_color_str))
                        .add_modifier(Modifier::BOLD)
                };

                frame.render_stateful_widget(
                    List::new(items)
                        .highlight_style(highlight_style)
                        .highlight_symbol(">> "),
                    body_area,
                    &mut state,
                );
            }
        } else if bookmarks.is_creating {
            self.bookmark_panel_height = body_area.height as usize;
            let all_bookmarks = bookmarks.list();
            let file_color =
                Config::parse_color(Config::get_color(&config.appearance.colors.file_color));

            if !all_bookmarks.is_empty() {
                let keys = all_bookmarks
                    .iter()
                    .map(|b| b.key.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                let wrap_width = if config.appearance.max_name_length > 0 {
                    config.appearance.max_name_length
                } else {
                    area.width as usize
                };
                let wrapped = wrap_bookmark_keys(&keys, wrap_width);
                frame.render_widget(
                    Paragraph::new(wrapped).style(Style::default().fg(file_color)),
                    body_area,
                );
            }
        } else if search.show_results && search.focus_on_results {
            // Inline search results
            let dir_color =
                Config::parse_color(Config::get_color(&config.appearance.colors.directory_color));
            let file_color =
                Config::parse_color(Config::get_color(&config.appearance.colors.file_color));
            let highlight_color =
                Config::parse_color(Config::get_color(&config.appearance.colors.highlight_color));

            if search.results.is_empty() {
                let msg = if search.is_searching {
                    "Searching..."
                } else {
                    "No results found"
                };
                frame.render_widget(
                    Paragraph::new(msg).style(Style::default().fg(Color::DarkGray)),
                    body_area,
                );
            } else {
                let root_path_buf = nav.root.borrow().path.clone();
                let root_parent = root_path_buf
                    .parent()
                    .unwrap_or(&root_path_buf)
                    .to_path_buf();

                let items: Vec<ListItem> = search
                    .results
                    .iter()
                    .map(|result| {
                        let display_path = result
                            .path
                            .strip_prefix(&root_parent)
                            .unwrap_or(&result.path)
                            .display()
                            .to_string();

                        let base_color = if result.is_dir { dir_color } else { file_color };

                        if let (true, Some(indices)) =
                            (search.fuzzy_mode, result.match_indices.as_ref())
                        {
                            let mut spans = Vec::new();
                            let chars: Vec<char> = display_path.chars().collect();
                            let mut last_idx = 0;
                            for &match_idx in indices {
                                if match_idx > last_idx {
                                    let text: String = chars[last_idx..match_idx].iter().collect();
                                    spans.push(Span::styled(text, Style::default().fg(base_color)));
                                }
                                if match_idx < chars.len() {
                                    let text: String =
                                        chars[match_idx..match_idx + 1].iter().collect();
                                    spans.push(Span::styled(
                                        text,
                                        Style::default()
                                            .fg(highlight_color)
                                            .add_modifier(Modifier::BOLD),
                                    ));
                                }
                                last_idx = match_idx + 1;
                            }
                            if last_idx < chars.len() {
                                let text: String = chars[last_idx..].iter().collect();
                                spans.push(Span::styled(text, Style::default().fg(base_color)));
                            }
                            ListItem::new(Line::from(spans))
                        } else {
                            ListItem::new(display_path).style(Style::default().fg(base_color))
                        }
                    })
                    .collect();

                let mut state = ListState::default();
                state.select(Some(search.selected));

                let visible = body_area.height as usize;
                let total = search.results.len();
                let offset = if search.center_selection {
                    if search.selected < visible / 2 {
                        0
                    } else if search.selected >= total.saturating_sub(visible / 2) {
                        total.saturating_sub(visible)
                    } else {
                        search.selected.saturating_sub(visible / 2)
                    }
                } else {
                    let current = self.search_scroll_offset;
                    if search.selected < current {
                        search.selected
                    } else if visible > 0 && search.selected >= current + visible {
                        search.selected.saturating_sub(visible - 1)
                    } else {
                        current
                    }
                };
                *state.offset_mut() = offset;
                self.search_scroll_offset = offset;

                let cursor_color_str = Config::get_color(&config.appearance.colors.cursor_color);
                let highlight_style = if cursor_color_str.to_lowercase() == "dim" {
                    Style::default().add_modifier(Modifier::DIM)
                } else {
                    Style::default()
                        .fg(Config::parse_color(cursor_color_str))
                        .add_modifier(Modifier::BOLD)
                };

                frame.render_stateful_widget(
                    List::new(items)
                        .highlight_style(highlight_style)
                        .highlight_symbol(">> "),
                    body_area,
                    &mut state,
                );
            }
        } else {
            // Normal tree items
            let dir_color =
                Config::parse_color(Config::get_color(&config.appearance.colors.directory_color));
            let error_color =
                Config::parse_color(Config::get_color(&config.appearance.colors.error_color));

            let items: Vec<ListItem> = nav
                .flat_list
                .iter()
                .map(|node| {
                    let n = node.borrow();
                    let indent = "  ".repeat(n.depth);
                    let unicode_icons = config.appearance.icons != "ascii";
                    let icon = if n.has_error {
                        if unicode_icons {
                            "⊘ "
                        } else {
                            "! "
                        }
                    } else if n.is_dir {
                        if n.is_expanded {
                            if unicode_icons {
                                "▼ "
                            } else {
                                "v "
                            }
                        } else if n.has_children == Some(false) {
                            "  "
                        } else if unicode_icons {
                            "▶ "
                        } else {
                            "> "
                        }
                    } else {
                        "  "
                    };
                    let display_name =
                        truncate_name_middle(&n.name, config.appearance.max_name_length);
                    let text = format!("{}{}{}", indent, icon, display_name);
                    let style = if n.has_error {
                        Style::default().fg(error_color)
                    } else if n.is_dir {
                        Style::default().fg(dir_color)
                    } else {
                        Style::default()
                    };
                    ListItem::new(text).style(style)
                })
                .collect();

            let mut state = ListState::default();
            state.select(Some(nav.selected));

            let visible = body_area.height as usize;
            let total = nav.flat_list.len();
            let offset = if nav.center_selection {
                // keyboard nav: keep selection centered in viewport
                if nav.selected < visible / 2 {
                    0
                } else if nav.selected >= total.saturating_sub(visible / 2) {
                    total.saturating_sub(visible)
                } else {
                    nav.selected.saturating_sub(visible / 2)
                }
            } else {
                // mouse action: minimal scroll — only move if selection left the visible area
                let current = self.tree_scroll_offset;
                if nav.selected < current {
                    nav.selected
                } else if visible > 0 && nav.selected >= current + visible {
                    nav.selected.saturating_sub(visible - 1)
                } else {
                    current
                }
            };
            *state.offset_mut() = offset;
            self.tree_scroll_offset = offset;

            let cursor_color_str = Config::get_color(&config.appearance.colors.tree_cursor_color);
            let cursor_bg_str = Config::get_color(&config.appearance.colors.tree_cursor_bg_color);

            let mut highlight_style = if cursor_color_str.to_lowercase() == "dim" {
                Style::default().add_modifier(Modifier::DIM)
            } else {
                Style::default().fg(Config::parse_color(cursor_color_str))
            };
            if cursor_bg_str.to_lowercase() != "dim" {
                highlight_style = highlight_style.bg(Config::parse_color(cursor_bg_str));
            }

            frame.render_stateful_widget(
                List::new(items)
                    .highlight_style(highlight_style)
                    .highlight_symbol(">> "),
                body_area,
                &mut state,
            );
        }

        // Search bar (single line, no block)
        if let Some(bar_area) = search_bar_area {
            let mode_tag = if search.fuzzy_mode { "(fuzzy) " } else { "" };
            let bar_text = format!("/{}{}", mode_tag, search.query);
            frame.render_widget(
                Paragraph::new(bar_text).style(Style::default().fg(selected_color)),
                bar_area,
            );
        }

        // Bookmark input bar (single line, no block)
        if let Some(input_area) = bookmark_input_area {
            let input_text = format!(" Add bookmark: {}|", bookmarks.get_input());
            frame.render_widget(
                Paragraph::new(input_text).style(
                    Style::default()
                        .fg(selected_color)
                        .add_modifier(Modifier::BOLD),
                ),
                input_area,
            );
        }
    }
}

/// Load help content from HELP.txt (embedded at compile time)
pub fn get_help_content() -> Vec<String> {
    const HELP_TEXT: &str = include_str!("../HELP.txt");
    HELP_TEXT.lines().map(|line| line.to_string()).collect()
}
