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
    #[arg(long = "version")]
    version: bool,

    /// Bookmark management mode (use: -bm, -bm add <name> [path], -bm remove <name>, -bm list)
    #[arg(long = "bm")]
    bookmark_mode: bool,

    /// All positional arguments (path or bookmark commands)
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
                Use 'bm -bm list' to see all bookmarks",
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
        Use 'bm -bm list' to see all bookmarks",
        input,
        input
    );
}

fn main() -> Result<()> {
    // Preprocess: convert -bm to --bm for clap compatibility
    let args: Vec<String> = std::env::args()
        .map(|arg| {
            if arg == "-bm" {
                "--bm".to_string()
            } else {
                arg
            }
        })
        .collect();

    let args = Args::parse_from(args);

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

    // Bookmark management mode (-bm)
    if args.bookmark_mode {
        let mut bookmarks = Bookmarks::new()?;

        if args.args.is_empty() {
            println!("Bookmarks:");
            if bookmarks.list().is_empty() {
                println!("  No bookmarks saved yet.");
                println!("\nUsage:");
                println!("  bm -bm add <name> [path]    Add a bookmark");
                println!("  bm -bm remove <name>        Remove a bookmark");
                println!("  bm -bm list                 List all bookmarks");
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

        match args.args[0].as_str() {
            "add" => {
                if args.args.len() < 2 {
                    anyhow::bail!("Missing bookmark name\nUsage: bm -bm add <name> [path]");
                }
                let name = &args.args[1];
                let path = if args.args.len() >= 3 {
                    PathBuf::from(&args.args[2])
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
            }
            "remove" => {
                if args.args.len() < 2 {
                    anyhow::bail!("Missing bookmark name\nUsage: bm -bm remove <name>");
                }
                let name = &args.args[1];
                bookmarks.remove(name)?;
                println!("Bookmark '{}' removed", name);
            }
            "list" => {
                println!("Bookmarks:");
                if bookmarks.list().is_empty() {
                    println!("  No bookmarks saved yet.");
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
            }
            unknown => {
                anyhow::bail!(
                    "Unknown bookmark command '{}'\n\n\
                    Available commands:\n\
                      bm -bm              List all bookmarks\n\
                      bm -bm add <name> [path]\n\
                      bm -bm remove <name>\n\
                      bm -bm list",
                    unknown
                );
            }
        }
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
