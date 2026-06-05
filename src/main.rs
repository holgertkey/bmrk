mod app;
mod bookmarks;
mod config;
mod disks;
mod event_handler;
mod navigation;
mod platform;
mod search;
mod terminal;
mod theme;
mod tree_node;
mod ui;

use anyhow::Result;
use app::App;
use bookmarks::Bookmarks;
use clap::Parser;
use platform::canonicalize_and_normalize;
use std::path::PathBuf;
use terminal::{cleanup_terminal_compact, run_app, setup_terminal_compact};

#[derive(Parser)]
#[command(name = "bmrk")]
#[command(about = "Interactive bookmark manager and directory navigator")]
#[command(disable_help_flag = true)]
#[command(disable_version_flag = true)]
struct Args {
    /// Print help information
    #[arg(short = 'h', long = "help")]
    help: bool,

    /// Print version information
    #[arg(short = 'v', long = "version")]
    version: bool,

    /// List all bookmarks
    #[arg(short = 'l', long = "list")]
    list: bool,

    /// Add a bookmark with the given name (uses current dir or trailing path arg)
    #[arg(short = 'a', long = "add", value_name = "NAME")]
    add: Option<String>,

    /// Delete a bookmark by name
    #[arg(short = 'd', long = "del", value_name = "NAME")]
    del: Option<String>,

    /// All positional arguments (path or bookmark name)
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

/// Resolve a path or bookmark name to a PathBuf
fn resolve_path_or_bookmark(input: &str, bookmarks: &Bookmarks) -> Result<PathBuf> {
    // Windows: Handle bare drive letters (e.g., "C:", "E:")
    #[cfg(windows)]
    {
        if input.len() == 2 && input.chars().nth(1) == Some(':') {
            let drive_letter = input.chars().next().unwrap();
            if drive_letter.is_ascii_alphabetic() {
                let root_path = format!("{}\\", input);
                let path = PathBuf::from(&root_path);
                if path.exists() {
                    return Ok(canonicalize_and_normalize(&path)?);
                } else {
                    anyhow::bail!("Drive not found: {}", input);
                }
            }
        }
    }

    if platform::is_absolute_path(input) || input.contains(std::path::MAIN_SEPARATOR) {
        let path = PathBuf::from(input);
        if !path.exists() {
            anyhow::bail!("Directory not found: {}", input);
        }
        return Ok(canonicalize_and_normalize(&path)?);
    }

    if let Some(bookmark) = bookmarks.get(input) {
        if bookmark.path.exists() {
            return Ok(bookmark.path.clone());
        } else {
            anyhow::bail!(
                "Bookmark '{}' points to non-existent directory: {}\n\
                Use 'bm -l' to see all bookmarks",
                input,
                bookmark.path.display()
            );
        }
    }

    let path = PathBuf::from(input);
    if path.exists() {
        return Ok(canonicalize_and_normalize(&path)?);
    }

    anyhow::bail!(
        "Neither bookmark '{}' nor directory '{}' found.\n\
        Use 'bm -l' to see all bookmarks",
        input,
        input
    );
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.version {
        println!("bmrk {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if args.help {
        for line in ui::get_help_content() {
            println!("{}", line);
        }
        return Ok(());
    }

    if args.list {
        let bookmarks = Bookmarks::new()?;
        println!("Bookmarks:");
        if bookmarks.list().is_empty() {
            println!("  No bookmarks saved yet.");
            println!("\nUsage:");
            println!("  bm -a <name> [path]    Add a bookmark");
            println!("  bm -d <name>           Remove a bookmark");
            println!("  bm -l                  List all bookmarks");
        } else {
            for bookmark in bookmarks.list() {
                let name = bookmark.name.as_deref().unwrap_or("(unnamed)");
                println!(
                    "  {} -> {} ({})",
                    bookmark.key,
                    name,
                    bookmark.path.display()
                );
            }
        }
        return Ok(());
    }

    if let Some(name) = args.add {
        let mut bookmarks = Bookmarks::new()?;
        let path = if !args.args.is_empty() {
            PathBuf::from(&args.args[0])
        } else {
            std::env::current_dir()?
        };

        if !path.exists() {
            anyhow::bail!("Path does not exist: {}", path.display());
        }

        let mut path = canonicalize_and_normalize(&path)?;

        if path.is_file() {
            if let Some(parent) = path.parent() {
                path = parent.to_path_buf();
                eprintln!("Note: File provided, using parent directory instead");
            } else {
                anyhow::bail!("Cannot determine parent directory");
            }
        }

        let dir_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string());

        bookmarks.add(name.clone(), path.clone(), dir_name)?;
        println!("Bookmark '{}' added: {}", name, path.display());
        return Ok(());
    }

    if let Some(name) = args.del {
        let mut bookmarks = Bookmarks::new()?;
        bookmarks.remove(&name)?;
        println!("Bookmark '{}' removed", name);
        return Ok(());
    }

    // If path/bookmark argument provided, resolve and output it
    if !args.args.is_empty() {
        let bookmarks = Bookmarks::new()?;
        let resolved = resolve_path_or_bookmark(&args.args[0], &bookmarks)?;
        println!("{}", resolved.display());
        return Ok(());
    }

    // No arguments: launch interactive compact TUI
    let start_path = std::env::current_dir()?;
    let mut app = App::new(start_path)?;

    let result = {
        let mut terminal = setup_terminal_compact()?;
        let r = run_app(&mut terminal, &mut app);
        r
    };
    cleanup_terminal_compact()?;

    if let Some(path) = result? {
        println!("{}", path.display());
    }

    Ok(())
}
