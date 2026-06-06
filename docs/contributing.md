# Contributing to bmrk

Thank you for your interest in contributing to bmrk! This guide will help you get started.

## Code of Conduct

Be respectful, inclusive, and constructive.

## Ways to Contribute

### 1. Report Bugs

Found a bug? Please [open an issue](https://github.com/holgertkey/bmrk/issues/new) with:

- **Clear title**: Describe the issue concisely
- **Steps to reproduce**: Exact steps to trigger the bug
- **Expected behavior**: What should happen
- **Actual behavior**: What actually happens
- **Environment**: OS, terminal emulator, Rust version, bmrk version

### 2. Suggest Features

Have an idea? [Open an issue](https://github.com/holgertkey/bmrk/issues/new) with:

- **Use case**: Why is this feature needed?
- **Proposed solution**: How should it work?
- **Alternatives**: Other approaches considered

### 3. Improve Documentation

Documentation improvements are always welcome:

- Fix typos or unclear wording
- Add examples
- Improve formatting

### 4. Write Code

Contribute code by:

- Fixing bugs
- Implementing features
- Improving performance
- Adding tests
- Refactoring

## Development Setup

### Prerequisites

- **Rust 1.70+**: Install from [rustup.rs](https://rustup.rs/)
- **Git**
- **A modern terminal**

### Clone and Build

```bash
# Fork the repository on GitHub first

# Clone your fork
git clone https://github.com/YOUR_USERNAME/bmrk.git
cd bmrk

# Add upstream remote
git remote add upstream https://github.com/holgertkey/bmrk.git

# Build
cargo build

# Run
cargo run

# Run with arguments
cargo run -- -l
```

### Development Workflow

```bash
# Create a feature branch
git checkout -b feature/my-feature

# Make changes and test
cargo run

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy

# Build release
cargo build --release
```

## Project Structure

```
bmrk/
├── src/
│   ├── main.rs           # Entry point, CLI, terminal setup
│   ├── app.rs            # Application state manager
│   ├── terminal.rs       # Terminal lifecycle and event loop
│   ├── event_handler.rs  # Input processing
│   ├── ui.rs             # Rendering logic
│   ├── navigation.rs     # Tree navigation logic
│   ├── tree_node.rs      # Tree data structure
│   ├── search.rs         # Search functionality
│   ├── bookmarks.rs      # Bookmark management
│   ├── config.rs         # Configuration management
│   ├── disks.rs          # Disk/volume information
│   ├── platform.rs       # Platform-specific utilities
│   └── lib.rs            # Library entry point
├── theme/                # Color theme structs and presets
├── docs/                 # Documentation
├── tests/                # Integration tests
├── HELP.txt              # Embedded help content (bm -h)
├── Cargo.toml            # Dependencies and metadata
├── CLAUDE.md             # Development guide
└── README.md             # Project README
```

## Architecture

bmrk follows a modular MVC-style architecture. See [Architecture](./architecture.md) for details.

**Key principles**:

1. **Separation of concerns**: Each module has a single responsibility
2. **Composition over inheritance**: `app.rs` orchestrates submodules
3. **Zero-copy when possible**: Use `Rc<RefCell<>>` for shared tree ownership
4. **Async for slow operations**: Background threads for search
5. **Graceful error handling**: Never crash, always inform the user

## Coding Guidelines

### Rust Style

- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Code Organization

- **Keep modules focused**: Single responsibility per module
- **Avoid `app.rs` bloat**: Create new modules instead of growing `app.rs`
- **Use descriptive names**: `handle_search_input` not `hsi`

### Error Handling

- **Use `anyhow::Result`** for ergonomic error propagation
- **Never use `unwrap()`** or `std::process::exit()` in the main code path
- **No panics**: Use `Result` or `Option`
- **Graceful degradation**: Show errors to user, don't crash

### Documentation

- All public items in the `engine` crate must have doc comments
- Comments in English only
- Explain *why*, not *what*

### Testing

- Write tests for new features and bug fixes
- Test edge cases (empty directories, permission errors, etc.)
- Use meaningful test names: `test_expand_collapses_all_children`

## Pull Request Process

### Before Submitting

1. Run tests: `cargo test`
2. Run clippy: `cargo clippy`
3. Run fmt: `cargo fmt`
4. Test manually: run `bm` and verify your changes
5. Update HELP.txt and docs if keybindings or behavior changed

### PR Template

```markdown
## Summary
Brief description of changes

## Motivation
Why is this change needed?

## Changes
- Specific change 1
- Specific change 2

## Testing
How was this tested?

Closes #123
```

### Review Process

1. CI runs tests, clippy, fmt
2. Maintainer reviews code
3. Address feedback
4. Once approved, PR is merged

## Getting Help

- Search [existing issues](https://github.com/holgertkey/bmrk/issues)
- Open a new issue for questions

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
