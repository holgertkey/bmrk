use anyhow::{Context, Result};
use crossterm::event::KeyCode;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::theme::ThemeConfig;

/// Appearance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    /// Theme name (can be expanded later for preset themes)
    #[serde(default = "default_theme")]
    pub theme: String,

    /// Maximum filename length before middle-truncation (0 = disabled)
    #[serde(default = "default_max_name_length")]
    pub max_name_length: usize,

    /// Custom theme colors
    #[serde(default)]
    pub colors: ThemeConfig,
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            max_name_length: default_max_name_length(),
            colors: ThemeConfig::default(),
        }
    }
}

fn default_theme() -> String {
    "default".to_string()
}
fn default_max_name_length() -> usize {
    30
}

/// Behavior configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorConfig {
    /// Show hidden files (dotfiles)
    #[serde(default = "default_show_hidden")]
    pub show_hidden: bool,

    /// Follow symbolic links
    #[serde(default = "default_follow_symlinks")]
    pub follow_symlinks: bool,

    /// Double-click timeout in milliseconds
    #[serde(default = "default_double_click_timeout")]
    pub double_click_timeout_ms: u64,

    /// Number of lines to scroll with mouse wheel
    #[serde(default = "default_mouse_scroll_lines")]
    pub mouse_scroll_lines: usize,
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            show_hidden: default_show_hidden(),
            follow_symlinks: default_follow_symlinks(),
            double_click_timeout_ms: default_double_click_timeout(),
            mouse_scroll_lines: default_mouse_scroll_lines(),
        }
    }
}

fn default_show_hidden() -> bool {
    true
}
fn default_follow_symlinks() -> bool {
    true
}
fn default_double_click_timeout() -> u64 {
    500
}
fn default_mouse_scroll_lines() -> usize {
    1
}

/// Keybindings configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindingsConfig {
    /// Keys to search
    #[serde(default = "default_search_keys")]
    pub search: Vec<String>,

    /// Keys to create bookmark
    #[serde(default = "default_create_bookmark_keys")]
    pub create_bookmark: Vec<String>,

    /// Keys to select bookmark
    #[serde(default = "default_select_bookmark_keys")]
    pub select_bookmark: Vec<String>,

    /// Keys to open disk selection panel
    #[serde(default = "default_select_disk_keys")]
    pub select_disk: Vec<String>,
}

impl Default for KeybindingsConfig {
    fn default() -> Self {
        Self {
            search: default_search_keys(),
            create_bookmark: default_create_bookmark_keys(),
            select_bookmark: default_select_bookmark_keys(),
            select_disk: default_select_disk_keys(),
        }
    }
}

fn default_search_keys() -> Vec<String> {
    vec!["/".to_string()]
}
fn default_create_bookmark_keys() -> Vec<String> {
    vec!["m".to_string()]
}
fn default_select_bookmark_keys() -> Vec<String> {
    vec!["'".to_string()]
}
fn default_select_disk_keys() -> Vec<String> {
    vec!["d".to_string()]
}

impl KeybindingsConfig {
    fn matches_key(&self, key: KeyCode, configured_keys: &[String]) -> bool {
        let key_str = match key {
            KeyCode::Char(c) => c.to_string(),
            KeyCode::Esc => "Esc".to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Left => "Left".to_string(),
            KeyCode::Right => "Right".to_string(),
            KeyCode::Up => "Up".to_string(),
            KeyCode::Down => "Down".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::Delete => "Delete".to_string(),
            KeyCode::Home => "Home".to_string(),
            KeyCode::End => "End".to_string(),
            KeyCode::PageUp => "PageUp".to_string(),
            KeyCode::PageDown => "PageDown".to_string(),
            _ => return false,
        };
        configured_keys
            .iter()
            .any(|k| k.eq_ignore_ascii_case(&key_str))
    }

    pub fn is_search(&self, key: KeyCode) -> bool {
        self.matches_key(key, &self.search)
    }

    pub fn is_create_bookmark(&self, key: KeyCode) -> bool {
        self.matches_key(key, &self.create_bookmark)
    }

    pub fn is_select_bookmark(&self, key: KeyCode) -> bool {
        self.matches_key(key, &self.select_bookmark)
    }

    pub fn is_select_disk(&self, key: KeyCode) -> bool {
        self.matches_key(key, &self.select_disk)
    }
}

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub appearance: AppearanceConfig,

    #[serde(default)]
    pub behavior: BehaviorConfig,

    #[serde(default)]
    pub keybindings: KeybindingsConfig,
}

impl Config {
    /// Parse a color string to ratatui Color
    pub fn parse_color(color_str: &str) -> Color {
        ThemeConfig::parse_color(color_str)
    }

    /// Get a color value (guaranteed to be Some after load())
    pub fn get_color(opt: &Option<String>) -> &str {
        opt.as_ref()
            .expect("Color should be resolved after config load")
    }

    /// Load configuration from a file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        Ok(config)
    }

    /// Get the global config file path
    /// Unix: ~/.config/bmrk/config.toml
    /// Windows: %APPDATA%\bmrk\config.toml
    pub fn global_config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("bmrk").join("config.toml"))
    }

    /// Load configuration with fallback to defaults.
    /// If config file doesn't exist, it will be created automatically.
    /// If config file has parse errors, returns an error with details.
    pub fn load() -> anyhow::Result<Self> {
        let mut config = Config::default();

        if let Some(global_path) = Self::global_config_path() {
            if !global_path.exists() {
                let _ = Self::create_default_file(&global_path);
            }

            if global_path.exists() {
                match Self::from_file(&global_path) {
                    Ok(global_config) => {
                        config = global_config;
                    }
                    Err(e) => {
                        anyhow::bail!(
                            "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
                            Configuration file error!\n\
                            ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
                            \n\
                            Config file: {}\n\
                            \n\
                            Error details:\n\
                            {:#}\n\
                            \n\
                            To fix:\n\
                              1. Edit the config file and fix the syntax error\n\
                              2. Or delete the file - it will be recreated with defaults\n\
                            \n\
                            ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
                            global_path.display(),
                            e
                        );
                    }
                }
            }
        }

        // Resolve color values from preset theme + fallbacks
        let preset = ThemeConfig::get_preset_theme(&config.appearance.theme);
        let fallback = ThemeConfig::fallback_colors();

        macro_rules! resolve_color {
            ($field:ident) => {
                config.appearance.colors.$field = config
                    .appearance
                    .colors
                    .$field
                    .or_else(|| preset.as_ref().and_then(|p| p.$field.clone()))
                    .or_else(|| fallback.$field.clone());
            };
        }

        resolve_color!(selected_color);
        resolve_color!(directory_color);
        resolve_color!(file_color);
        resolve_color!(border_color);
        resolve_color!(error_color);
        resolve_color!(highlight_color);
        resolve_color!(file_search_highlight_color);
        resolve_color!(cursor_color);
        resolve_color!(tree_cursor_color);
        resolve_color!(tree_cursor_bg_color);
        resolve_color!(main_border_color);
        resolve_color!(panel_border_color);
        resolve_color!(background_color);

        Ok(config)
    }

    /// Create a default config file with comments
    pub fn create_default_file(path: &Path) -> Result<()> {
        let default_config = r#"# bmrk configuration file
# This file uses TOML format: https://toml.io

[appearance]
# Theme name - preset color schemes
# Available themes:
#   "default"    - Classic terminal colors (blue dirs, cyan selection)
#   "gruvbox"    - Warm, high contrast theme inspired by Gruvbox
#   "nord"       - Cold, muted colors inspired by Nord theme
#   "tokyonight" - Modern dark theme with vibrant colors
#   "dracula"    - Popular dark theme with high contrast
#   "obsidian"   - Dark theme inspired by Obsidian app with subtle cursor
theme = "default"

# Maximum filename length in the tree before middle-truncation
# Example: "very_long_project_name.rs" -> "very_long_pro...ame.rs"
# Set to 0 to disable truncation
max_name_length = 30

# Custom theme colors (override preset theme)
[appearance.colors]
# Color formats: name (red, blue...), #RRGGBB hex, 0-255 indexed, "reset"
#
# selected_color = "cyan"
# directory_color = "gray"
# file_color = "white"
# border_color = "gray"
# error_color = "red"
# highlight_color = "yellow"
# file_search_highlight_color = "yellow"
# cursor_color = "yellow"
# tree_cursor_color = "dim"
# tree_cursor_bg_color = "dim"
# main_border_color = "gray"
# panel_border_color = "cyan"
# background_color = "reset"

[behavior]
# Show hidden files (dotfiles)
show_hidden = true

# Follow symbolic links
follow_symlinks = true

# Double-click timeout in milliseconds
double_click_timeout_ms = 500

# Number of lines to scroll with mouse wheel
mouse_scroll_lines = 1

[keybindings]
# Key bindings (each can have multiple keys)
search = ["/"]
create_bookmark = ["m"]
select_bookmark = ["'"]
select_disk = ["d"]
"#;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory: {}", parent.display())
            })?;
        }

        fs::write(path, default_config)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.appearance.max_name_length, 30);
        assert!(config.behavior.show_hidden);
    }

    #[test]
    fn test_color_parsing() {
        assert!(matches!(ThemeConfig::parse_color("red"), Color::Red));
        assert!(matches!(ThemeConfig::parse_color("blue"), Color::Blue));
        assert!(matches!(
            ThemeConfig::parse_color("#FF0000"),
            Color::Rgb(255, 0, 0)
        ));
    }
}
