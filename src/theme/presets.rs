use super::ThemeConfig;

/// Get preset theme by name
pub fn get_preset(theme_name: &str) -> Option<ThemeConfig> {
    match theme_name.to_lowercase().as_str() {
        "default" => Some(default_theme()),
        "gruvbox" => Some(gruvbox_theme()),
        "nord" => Some(nord_theme()),
        "tokyonight" => Some(tokyonight_theme()),
        "dracula" => Some(dracula_theme()),
        "obsidian" => Some(obsidian_theme()),
        _ => None,
    }
}

/// Default theme - Classic terminal colors
fn default_theme() -> ThemeConfig {
    ThemeConfig {
        selected_color: Some("cyan".to_string()),
        directory_color: Some("gray".to_string()),
        file_color: Some("white".to_string()),
        error_color: Some("red".to_string()),
        highlight_color: Some("yellow".to_string()),
        cursor_color: Some("yellow".to_string()),
        tree_cursor_color: Some("dim".to_string()),
        tree_cursor_bg_color: Some("dim".to_string()),
        header_path_color: Some("cyan".to_string()),
        header_hints_color: Some("darkgray".to_string()),
    }
}

/// Gruvbox theme - Warm, high contrast theme inspired by Gruvbox
fn gruvbox_theme() -> ThemeConfig {
    ThemeConfig {
        selected_color: Some("#fe8019".to_string()), // bright orange
        directory_color: Some("#83a598".to_string()), // bright blue
        file_color: Some("#ebdbb2".to_string()),     // light foreground
        error_color: Some("#fb4934".to_string()),    // bright red
        highlight_color: Some("#fabd2f".to_string()), // bright yellow
        cursor_color: Some("#fabd2f".to_string()),   // yellow for search & bookmarks
        tree_cursor_color: Some("#ebdbb2".to_string()), // light foreground text
        tree_cursor_bg_color: Some("#303030".to_string()), // barely visible darker background
        header_path_color: Some("#fe8019".to_string()), // bright orange
        header_hints_color: Some("#928374".to_string()), // warm gray
    }
}

/// Nord theme - Cold, muted colors inspired by Nord theme
fn nord_theme() -> ThemeConfig {
    ThemeConfig {
        selected_color: Some("#88c0d0".to_string()), // frost cyan
        directory_color: Some("#81a1c1".to_string()), // frost blue
        file_color: Some("#eceff4".to_string()),     // snow white
        error_color: Some("#bf616a".to_string()),    // aurora red
        highlight_color: Some("#ebcb8b".to_string()), // aurora yellow
        cursor_color: Some("#ebcb8b".to_string()),   // yellow for search & bookmarks
        tree_cursor_color: Some("#eceff4".to_string()), // snow white text
        tree_cursor_bg_color: Some("#343a48".to_string()), // barely visible lighter background
        header_path_color: Some("#88c0d0".to_string()), // frost cyan
        header_hints_color: Some("#616e88".to_string()), // muted polar night
    }
}

/// Tokyo Night theme - Modern dark theme with vibrant colors
fn tokyonight_theme() -> ThemeConfig {
    ThemeConfig {
        selected_color: Some("#7aa2f7".to_string()),       // blue
        directory_color: Some("#7dcfff".to_string()),      // cyan
        file_color: Some("#a9b1d6".to_string()),           // light gray-blue
        error_color: Some("#f7768e".to_string()),          // red
        highlight_color: Some("#e0af68".to_string()),      // yellow
        cursor_color: Some("#bb9af7".to_string()),         // purple for search & bookmarks
        tree_cursor_color: Some("#a9b1d6".to_string()),    // light gray-blue text
        tree_cursor_bg_color: Some("#1f202e".to_string()), // barely visible lighter background
        header_path_color: Some("#7aa2f7".to_string()),    // blue
        header_hints_color: Some("#565f89".to_string()),   // comment gray
    }
}

/// Dracula theme - Popular dark theme with high contrast
fn dracula_theme() -> ThemeConfig {
    ThemeConfig {
        selected_color: Some("#ff79c6".to_string()),       // pink
        directory_color: Some("#8be9fd".to_string()),      // cyan
        file_color: Some("#f8f8f2".to_string()),           // white
        error_color: Some("#ff5555".to_string()),          // red
        highlight_color: Some("#f1fa8c".to_string()),      // yellow
        cursor_color: Some("#bd93f9".to_string()),         // purple for search & bookmarks
        tree_cursor_color: Some("#f8f8f2".to_string()),    // white text
        tree_cursor_bg_color: Some("#2d2f3d".to_string()), // barely visible lighter background
        header_path_color: Some("#ff79c6".to_string()),    // pink
        header_hints_color: Some("#6272a4".to_string()),   // comment gray
    }
}

/// Obsidian theme - Dark theme inspired by Obsidian app with subtle cursor
fn obsidian_theme() -> ThemeConfig {
    ThemeConfig {
        selected_color: Some("#a88bfa".to_string()), // soft purple
        directory_color: Some("#8b9dff".to_string()), // light blue
        file_color: Some("#dcddde".to_string()),     // light gray
        error_color: Some("#f14c4c".to_string()),    // soft red
        highlight_color: Some("#c792ea".to_string()), // violet
        cursor_color: Some("#a88bfa".to_string()),   // purple for search & bookmarks
        tree_cursor_color: Some("#dcddde".to_string()), // light gray text
        tree_cursor_bg_color: Some("#1e1e21".to_string()), // barely visible dark background
        header_path_color: Some("#a88bfa".to_string()), // soft purple
        header_hints_color: Some("#5a5a5e".to_string()), // muted gray
    }
}
