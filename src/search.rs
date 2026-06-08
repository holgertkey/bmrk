// Allow many arguments for recursive search function - it needs context for deep traversal
#![allow(clippy::too_many_arguments)]

use crate::tree_node::TreeNodeRef;
use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use std::collections::HashSet;
use std::path::PathBuf;
use std::rc::Rc;
use std::thread::{self, JoinHandle};

/// Maximum number of search results to collect (prevents O(n²) dedup stall on broad queries)
const MAX_SEARCH_RESULTS: usize = 500;
/// Maximum messages processed per `poll_results` call to keep the UI responsive
const MAX_POLL_BATCH: usize = 200;

/// Messages from search thread to main thread
#[derive(Debug, Clone)]
pub enum SearchMessage {
    /// Found a matching path (path, is_dir, score, match_indices)
    Result(PathBuf, bool, Option<i64>, Option<Vec<usize>>),
    /// Progress update: number of directories scanned
    Progress(usize),
    /// Search completed
    Done,
}

/// Search result with metadata
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: PathBuf,
    pub is_dir: bool,
    pub score: Option<i64>, // Fuzzy match score (None for exact match)
    pub match_indices: Option<Vec<usize>>, // Character positions that matched (for highlighting)
}

/// Search functionality for finding files and directories
pub struct Search {
    pub mode: bool,
    pub query: String,
    pub fuzzy_mode: bool, // True if query starts with '/'
    pub results: Vec<SearchResult>,
    pub selected: usize,
    pub show_results: bool,
    pub focus_on_results: bool,
    /// Whether to center the selected item on the next render (true = keyboard nav, false = mouse).
    pub center_selection: bool,

    // Async search state
    pub is_searching: bool,
    pub scanned_count: usize,
    search_thread: Option<JoinHandle<()>>,
    cancel_sender: Option<Sender<()>>,
    result_receiver: Option<Receiver<SearchMessage>>,

    seen_paths: HashSet<PathBuf>, // O(1) deduplication across Phase 1 + Phase 2
}

impl Default for Search {
    fn default() -> Self {
        Self::new()
    }
}

impl Search {
    pub fn new() -> Self {
        Self {
            mode: false,
            query: String::new(),
            fuzzy_mode: false,
            results: Vec::new(),
            selected: 0,
            show_results: false,
            focus_on_results: false,
            center_selection: true,
            is_searching: false,
            scanned_count: 0,
            search_thread: None,
            cancel_sender: None,
            result_receiver: None,
            seen_paths: HashSet::new(),
        }
    }

    /// Enter search mode
    pub fn enter_mode(&mut self) {
        self.mode = true;
        self.query.clear();
        self.fuzzy_mode = false;
    }

    /// Exit search mode
    pub fn exit_mode(&mut self) {
        self.mode = false;
        self.query.clear();
        self.fuzzy_mode = false;
    }

    /// Add character to query
    pub fn add_char(&mut self, c: char) {
        self.query.push(c);
        self.update_fuzzy_mode();
    }

    /// Remove last character from query
    pub fn backspace(&mut self) {
        self.query.pop();
        self.update_fuzzy_mode();
    }

    /// Update fuzzy mode based on query
    fn update_fuzzy_mode(&mut self) {
        self.fuzzy_mode = self.query.starts_with('/');
    }

    /// Get actual search query (without leading '/' if in fuzzy mode)
    fn get_search_query(&self) -> &str {
        if self.fuzzy_mode && self.query.len() > 1 {
            &self.query[1..]
        } else if self.fuzzy_mode {
            "" // Only '/' entered, empty query
        } else {
            &self.query
        }
    }

    /// Execute two-phase search: quick + deep background scan
    pub fn perform_search(
        &mut self,
        root: &TreeNodeRef,
        show_files: bool,
        show_hidden: bool,
        follow_symlinks: bool,
    ) {
        use fuzzy_matcher::skim::SkimMatcherV2;

        // Cancel any existing search
        self.cancel_search();

        self.results.clear();
        self.seen_paths.clear();
        self.selected = 0;
        self.scanned_count = 0;
        self.center_selection = true;

        let search_query = self.get_search_query();

        // Don't search if query is empty (e.g., user entered just '/')
        if search_query.is_empty() {
            self.show_results = false;
            self.is_searching = false;
            return;
        }

        let query_lower = search_query.to_lowercase();
        let is_fuzzy = self.fuzzy_mode;

        // Create matcher once for Phase 1 (not per-entry)
        let matcher = is_fuzzy.then(SkimMatcherV2::default);

        // Phase 1: Quick search through already loaded nodes
        self.search_loaded_nodes(
            root,
            &query_lower,
            show_files,
            show_hidden,
            is_fuzzy,
            &matcher,
        );

        // Phase 2: Deep search in background thread
        self.spawn_deep_search(
            root,
            query_lower,
            show_files,
            show_hidden,
            follow_symlinks,
            is_fuzzy,
        );

        self.show_results = true;
        self.focus_on_results = true; // Always focus on results after search
        self.mode = false;
        self.is_searching = true;
    }

    /// Phase 1: Quick search through already loaded (visible) nodes
    fn search_loaded_nodes(
        &mut self,
        node: &TreeNodeRef,
        query: &str,
        show_files: bool,
        show_hidden: bool,
        fuzzy: bool,
        matcher: &Option<fuzzy_matcher::skim::SkimMatcherV2>,
    ) {
        use fuzzy_matcher::FuzzyMatcher;

        if self.results.len() >= MAX_SEARCH_RESULTS {
            return;
        }

        let node_borrowed = node.borrow();
        let name_lower = node_borrowed.name.to_lowercase();

        // Skip hidden files/directories
        if !show_hidden && node_borrowed.name.starts_with('.') {
            return;
        }

        // Check current node
        if show_files || node_borrowed.is_dir {
            let path = node_borrowed.path.clone();
            if !self.seen_paths.contains(&path) {
                let result = if fuzzy {
                    matcher
                        .as_ref()
                        .and_then(|m| m.fuzzy_indices(&name_lower, query))
                        .map(|(score, indices)| SearchResult {
                            path: path.clone(),
                            is_dir: node_borrowed.is_dir,
                            score: Some(score),
                            match_indices: Some(indices),
                        })
                } else if name_lower.contains(query) {
                    Some(SearchResult {
                        path: path.clone(),
                        is_dir: node_borrowed.is_dir,
                        score: None,
                        match_indices: None,
                    })
                } else {
                    None
                };

                if let Some(r) = result {
                    self.seen_paths.insert(path);
                    self.results.push(r);
                }
            }
        }

        // Recursively search already loaded children
        if node_borrowed.is_expanded {
            let children_count = node_borrowed.children.len();
            drop(node_borrowed);

            for i in 0..children_count {
                if self.results.len() >= MAX_SEARCH_RESULTS {
                    return;
                }
                let child = Rc::clone(&node.borrow().children[i]);
                self.search_loaded_nodes(&child, query, show_files, show_hidden, fuzzy, matcher);
            }
        }
    }

    /// Phase 2: Spawn background thread for deep search
    fn spawn_deep_search(
        &mut self,
        root: &TreeNodeRef,
        query: String,
        show_files: bool,
        show_hidden: bool,
        follow_symlinks: bool,
        fuzzy: bool,
    ) {
        let (result_tx, result_rx) = unbounded();
        let (cancel_tx, cancel_rx) = bounded(1);

        // Clone root node for thread (Rc can't be sent across threads, so we need path)
        let root_path = root.borrow().path.clone();

        // Spawn search thread — matcher created once here, not per-entry
        let handle = thread::spawn(move || {
            use fuzzy_matcher::skim::SkimMatcherV2;
            let matcher = fuzzy.then(SkimMatcherV2::default);
            Self::deep_search_recursive(
                &root_path,
                &mut HashSet::new(),
                &query,
                &result_tx,
                &cancel_rx,
                show_files,
                show_hidden,
                follow_symlinks,
                fuzzy,
                &mut 0,
                &matcher,
            );
            let _ = result_tx.send(SearchMessage::Done);
        });

        self.search_thread = Some(handle);
        self.cancel_sender = Some(cancel_tx);
        self.result_receiver = Some(result_rx);
    }

    /// Recursive deep search in background thread
    fn deep_search_recursive(
        path: &PathBuf,
        visited: &mut HashSet<PathBuf>,
        query: &str,
        result_tx: &Sender<SearchMessage>,
        cancel_rx: &Receiver<()>,
        show_files: bool,
        show_hidden: bool,
        follow_symlinks: bool,
        fuzzy: bool,
        scanned: &mut usize,
        matcher: &Option<fuzzy_matcher::skim::SkimMatcherV2>,
    ) {
        use fuzzy_matcher::FuzzyMatcher;

        // Check for cancellation
        if cancel_rx.try_recv().is_ok() {
            return;
        }

        // Check if entry is a symlink and whether to follow it
        if !follow_symlinks {
            if let Ok(metadata) = std::fs::symlink_metadata(path) {
                if metadata.is_symlink() {
                    return;
                }
            }
        }

        let is_dir = path.is_dir();

        if !is_dir && !show_files {
            return;
        }

        // Skip hidden files/directories
        if !show_hidden {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with('.') {
                    return;
                }
            }
        }

        // Check if name matches query
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            let name_lower = name.to_lowercase();

            if fuzzy {
                if let Some(m) = matcher {
                    if let Some((score, indices)) = m.fuzzy_indices(&name_lower, query) {
                        let _ = result_tx.send(SearchMessage::Result(
                            path.clone(),
                            is_dir,
                            Some(score),
                            Some(indices),
                        ));
                    }
                }
            } else if name_lower.contains(query) {
                let _ = result_tx.send(SearchMessage::Result(path.clone(), is_dir, None, None));
            }
        }

        // If directory, scan children
        if is_dir {
            // Cycle detection: skip canonical paths already visited (handles symlink cycles A→B→A)
            if follow_symlinks {
                if let Ok(canonical) = path.canonicalize() {
                    if !visited.insert(canonical) {
                        return;
                    }
                }
                // canonicalize() failure is non-fatal: proceed without recording this path
            }

            *scanned += 1;

            if (*scanned).is_multiple_of(100) {
                let _ = result_tx.send(SearchMessage::Progress(*scanned));
            }

            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    if cancel_rx.try_recv().is_ok() {
                        return;
                    }

                    let child_path = entry.path();
                    Self::deep_search_recursive(
                        &child_path,
                        visited,
                        query,
                        result_tx,
                        cancel_rx,
                        show_files,
                        show_hidden,
                        follow_symlinks,
                        fuzzy,
                        scanned,
                        matcher,
                    );
                }
            }
        }
    }

    /// Poll for new search results from background thread
    pub fn poll_results(&mut self) -> bool {
        let mut has_updates = false;
        let mut search_done = false;
        let mut batch = 0;

        if let Some(ref rx) = self.result_receiver {
            while let Ok(msg) = rx.try_recv() {
                batch += 1;
                match msg {
                    SearchMessage::Result(path, is_dir, score, match_indices) => {
                        // O(1) dedup via HashSet (was O(n) Vec scan)
                        if !self.seen_paths.contains(&path) {
                            self.seen_paths.insert(path.clone());
                            self.results.push(SearchResult {
                                path,
                                is_dir,
                                score,
                                match_indices,
                            });
                            has_updates = true;

                            if self.results.len() >= MAX_SEARCH_RESULTS {
                                // Enough results — stop the background thread
                                if let Some(cancel_tx) = &self.cancel_sender {
                                    let _ = cancel_tx.send(());
                                }
                                search_done = true;
                                break;
                            }
                        }
                    }
                    SearchMessage::Progress(count) => {
                        self.scanned_count = count;
                        has_updates = true;
                    }
                    SearchMessage::Done => {
                        search_done = true;
                        has_updates = true;
                    }
                }

                if batch >= MAX_POLL_BATCH {
                    // Yield to the event loop; remaining messages picked up next tick
                    break;
                }
            }
        }

        // Clean up if search is done
        if search_done {
            self.is_searching = false;
            self.search_thread = None;
            self.cancel_sender = None;
            self.result_receiver = None;

            // Sort results by score in fuzzy mode (highest score first)
            if self.fuzzy_mode {
                self.results.sort_by(|a, b| {
                    match (a.score, b.score) {
                        (Some(score_a), Some(score_b)) => score_b.cmp(&score_a), // Descending order
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => std::cmp::Ordering::Equal,
                    }
                });
            }
        }

        has_updates
    }

    /// Cancel current search
    pub fn cancel_search(&mut self) {
        if let Some(cancel_tx) = self.cancel_sender.take() {
            let _ = cancel_tx.send(());
        }
        // Thread checks cancel_rx frequently — detach, no join needed
        self.search_thread = None;
        self.result_receiver = None;
        self.is_searching = false;
        self.seen_paths.clear();
    }

    /// Move selection down in results
    pub fn move_down(&mut self) {
        if self.selected < self.results.len().saturating_sub(1) {
            self.selected += 1;
        }
    }

    /// Move selection up in results
    pub fn move_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    /// Get number of search results
    #[allow(dead_code)]
    pub fn get_results_count(&self) -> usize {
        self.results.len()
    }

    /// Set selected index (with bounds checking)
    #[allow(dead_code)]
    pub fn set_selected(&mut self, index: usize) {
        if index < self.results.len() {
            self.selected = index;
        }
    }

    /// Get selected result path
    pub fn get_selected_result(&self) -> Option<PathBuf> {
        self.results.get(self.selected).map(|r| r.path.clone())
    }

    /// Close search results panel
    pub fn close_results(&mut self) {
        self.cancel_search();
        self.show_results = false;
        self.results.clear();
        self.selected = 0;
        self.focus_on_results = false;
        self.center_selection = true;
        self.scanned_count = 0;
    }

    /// Toggle focus between tree and search results
    pub fn toggle_focus(&mut self) {
        if self.show_results {
            self.focus_on_results = !self.focus_on_results;
        }
    }

    /// Check if search is active
    pub fn is_active(&self) -> bool {
        self.is_searching
    }
}

impl Drop for Search {
    fn drop(&mut self) {
        self.cancel_search();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn test_cancel_search_does_not_block() {
        // This test ensures that cancel_search() returns quickly
        // even if the background thread is still running
        let mut search = Search::new();

        // Create a temporary directory with some files for testing
        let test_dir = std::env::temp_dir().join("dtree_test_cancel");
        std::fs::create_dir_all(&test_dir).unwrap();

        let root = std::rc::Rc::new(std::cell::RefCell::new(
            crate::tree_node::TreeNode::new(test_dir.clone(), 0).unwrap(),
        ));

        // Start a search
        search.enter_mode();
        search.add_char('t');
        search.add_char('e');
        search.add_char('s');
        search.add_char('t');
        search.perform_search(&root, false, false, false);

        // Give the background thread time to start
        std::thread::sleep(Duration::from_millis(10));

        // Now cancel the search and measure how long it takes
        let start = Instant::now();
        search.cancel_search();
        let elapsed = start.elapsed();

        // cancel_search() should return almost immediately (< 50ms)
        // If it blocks on join(), it would take much longer
        assert!(
            elapsed < Duration::from_millis(50),
            "cancel_search() took too long: {:?}",
            elapsed
        );

        // Clean up
        let _ = std::fs::remove_dir_all(&test_dir);
    }

    #[test]
    fn test_repeated_search_does_not_hang() {
        // This test simulates the bug scenario: starting a new search
        // while another search is already running
        let mut search = Search::new();

        let test_dir = std::env::temp_dir().join("dtree_test_repeated");
        std::fs::create_dir_all(&test_dir).unwrap();

        let root = std::rc::Rc::new(std::cell::RefCell::new(
            crate::tree_node::TreeNode::new(test_dir.clone(), 0).unwrap(),
        ));

        // Start first search
        search.enter_mode();
        search.add_char('a');
        search.perform_search(&root, false, false, false);

        // Give it a moment to start
        std::thread::sleep(Duration::from_millis(10));

        // Start second search immediately (this should not hang)
        let start = Instant::now();
        search.enter_mode();
        search.add_char('b');
        search.perform_search(&root, false, false, false);
        let elapsed = start.elapsed();

        // The second search should start quickly without blocking
        assert!(
            elapsed < Duration::from_millis(100),
            "Second search took too long: {:?}",
            elapsed
        );

        // Start third search (stress test)
        search.enter_mode();
        search.add_char('c');
        search.perform_search(&root, false, false, false);

        // Clean up
        search.cancel_search();
        let _ = std::fs::remove_dir_all(&test_dir);
    }

    #[test]
    fn test_rapid_search_start_stop() {
        // Stress test: rapidly start and stop searches
        let mut search = Search::new();

        let test_dir = std::env::temp_dir().join("dtree_test_rapid");
        std::fs::create_dir_all(&test_dir).unwrap();

        let root = std::rc::Rc::new(std::cell::RefCell::new(
            crate::tree_node::TreeNode::new(test_dir.clone(), 0).unwrap(),
        ));

        let start = Instant::now();

        // Rapidly start and cancel 10 searches
        for i in 0..10 {
            search.enter_mode();
            search.add_char('a');
            search.add_char((b'0' + (i % 10) as u8) as char);
            search.perform_search(&root, false, false, false);
            std::thread::sleep(Duration::from_millis(5));
        }

        let elapsed = start.elapsed();

        // All 10 search starts should complete in < 1 second
        // (each should take ~5ms + some overhead)
        assert!(
            elapsed < Duration::from_secs(1),
            "Rapid searches took too long: {:?}",
            elapsed
        );

        // Clean up
        search.cancel_search();
        let _ = std::fs::remove_dir_all(&test_dir);
    }

    /// Verify that cyclic symlinks (A → B → A) do not cause infinite recursion.
    ///
    /// Prints whether symlinks were actually created so a skip is visible in `--nocapture`
    /// output and cannot masquerade as a passing run.
    #[test]
    fn test_symlink_cycle_does_not_hang() {
        use std::time::{Duration, Instant};

        let test_dir = std::env::temp_dir().join("bmrk_test_symlink_cycle");
        let _ = std::fs::remove_dir_all(&test_dir);
        std::fs::create_dir_all(&test_dir).unwrap();

        let dir_a = test_dir.join("dir_a");
        std::fs::create_dir_all(&dir_a).unwrap();

        // dir_b (symlink) → dir_a
        let dir_b = test_dir.join("dir_b");
        #[cfg(unix)]
        let r1 = std::os::unix::fs::symlink(&dir_a, &dir_b);
        #[cfg(windows)]
        let r1 = std::os::windows::fs::symlink_dir(&dir_a, &dir_b);
        #[cfg(not(any(unix, windows)))]
        let r1: std::io::Result<()> = Err(std::io::Error::other("unsupported platform"));

        if r1.is_err() {
            eprintln!(
                "test_symlink_cycle_does_not_hang: SKIPPED (symlink creation failed: {:?})",
                r1
            );
            let _ = std::fs::remove_dir_all(&test_dir);
            return;
        }

        // dir_a/link_to_b (symlink) → dir_b  →  completes the cycle dir_a ↔ dir_b
        let link_in_a = dir_a.join("link_to_b");
        #[cfg(unix)]
        let r2 = std::os::unix::fs::symlink(&dir_b, &link_in_a);
        #[cfg(windows)]
        let r2 = std::os::windows::fs::symlink_dir(&dir_b, &link_in_a);
        #[cfg(not(any(unix, windows)))]
        let r2: std::io::Result<()> = Err(std::io::Error::other("unsupported platform"));

        if r2.is_err() {
            eprintln!(
                "test_symlink_cycle_does_not_hang: SKIPPED (second symlink failed: {:?})",
                r2
            );
            let _ = std::fs::remove_dir_all(&test_dir);
            return;
        }

        eprintln!("test_symlink_cycle_does_not_hang: cyclic symlinks created, running search");

        let root = std::rc::Rc::new(std::cell::RefCell::new(
            crate::tree_node::TreeNode::new(test_dir.clone(), 0).unwrap(),
        ));

        let mut search = Search::new();
        search.enter_mode();
        search.add_char('d'); // matches "dir_a", "dir_b"

        let start = Instant::now();
        search.perform_search(&root, false, true, true); // follow_symlinks = true

        let timeout = Duration::from_secs(5);
        while start.elapsed() < timeout {
            search.poll_results();
            if !search.is_active() {
                break;
            }
            std::thread::sleep(Duration::from_millis(20));
        }

        let elapsed = start.elapsed();
        eprintln!(
            "test_symlink_cycle_does_not_hang: completed in {:?}",
            elapsed
        );

        assert!(
            !search.is_active(),
            "Search did not complete — likely infinite loop in cyclic symlinks"
        );
        assert!(
            elapsed < Duration::from_secs(5),
            "Search took too long ({:?}): possible infinite loop",
            elapsed
        );

        let _ = std::fs::remove_dir_all(&test_dir);
    }
}
