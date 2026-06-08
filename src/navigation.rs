use crate::tree_node::{TreeNode, TreeNodeRef};
use anyhow::Result;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// Result of a recursive toggle search: distinguishes "not found" from "found".
enum ToggleResult {
    NotFound,
    Found(Option<String>),
}

/// Navigation logic for tree traversal and manipulation
pub struct Navigation {
    pub root: TreeNodeRef,
    pub flat_list: Vec<TreeNodeRef>,
    pub selected: usize,
    pub show_hidden: bool,
    pub follow_symlinks: bool,
    /// History stack of previously visited root paths for back-navigation (`u`).
    pub history: VecDeque<PathBuf>,
    /// Last navigation error message, shown in the UI header until the next successful navigation.
    pub nav_error: Option<String>,
    /// When `true` the renderer centers the selected item; `false` uses minimal (ensure-visible) scroll.
    /// Set to `true` by keyboard navigation, `false` by mouse actions.
    pub center_selection: bool,
    // Performance optimization: HashMap for O(1) path lookup
    path_to_index: HashMap<PathBuf, usize>,
}

impl Navigation {
    pub fn new(
        start_path: PathBuf,
        show_files: bool,
        show_hidden: bool,
        follow_symlinks: bool,
    ) -> Result<Self> {
        let mut root = TreeNode::new(start_path, 0)?;
        root.load_children(show_files, show_hidden, follow_symlinks)?;
        root.is_expanded = true;
        let root = Rc::new(RefCell::new(root));

        let mut nav = Self {
            root,
            flat_list: Vec::new(),
            selected: 0,
            show_hidden,
            follow_symlinks,
            history: VecDeque::new(),
            nav_error: None,
            center_selection: true,
            path_to_index: HashMap::new(),
        };

        nav.rebuild_flat_list();
        Ok(nav)
    }

    /// Rebuild flat list of visible nodes and update path index
    pub fn rebuild_flat_list(&mut self) {
        self.flat_list.clear();
        self.path_to_index.clear();
        Self::collect_visible_nodes(&self.root, &mut self.flat_list);

        // Build path → index mapping for O(1) lookups
        for (idx, node) in self.flat_list.iter().enumerate() {
            let path = node.borrow().path.clone();
            self.path_to_index.insert(path, idx);
        }
    }

    fn collect_visible_nodes(node: &TreeNodeRef, result: &mut Vec<TreeNodeRef>) {
        result.push(Rc::clone(node));

        // Check if node is expanded and get children count
        let (is_expanded, children_count) = {
            let node_borrowed = node.borrow();
            (node_borrowed.is_expanded, node_borrowed.children.len())
        };

        if is_expanded {
            // Recursively collect children without cloning the entire vector
            for i in 0..children_count {
                let child = Rc::clone(&node.borrow().children[i]);
                Self::collect_visible_nodes(&child, result);
            }
        }
    }

    /// Get currently selected node
    pub fn get_selected_node(&self) -> Option<TreeNodeRef> {
        self.flat_list.get(self.selected).map(Rc::clone)
    }

    /// Move selection down
    pub fn move_down(&mut self) {
        if self.selected < self.flat_list.len().saturating_sub(1) {
            self.selected += 1;
        }
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    /// Toggle node expansion at path.
    /// Returns `Some(error_message)` if the node has an error after toggle, `None` otherwise.
    pub fn toggle_node(&mut self, path: &Path, show_files: bool) -> Result<Option<String>> {
        // Fast path: use path index to toggle without walking the tree.
        if let Some(index) = self.path_to_index.get(path).copied() {
            if index < self.flat_list.len() {
                let error_msg = {
                    let mut node_borrowed = self.flat_list[index].borrow_mut();
                    node_borrowed.toggle_expand(
                        show_files,
                        self.show_hidden,
                        self.follow_symlinks,
                    )?;
                    if node_borrowed.has_error {
                        node_borrowed.error_message.clone()
                    } else {
                        None
                    }
                };
                self.rebuild_flat_list();
                return Ok(error_msg);
            }
        }

        // Fallback: walk the tree to find and toggle the node.
        let error_msg = match Self::toggle_node_recursive(
            &self.root,
            path,
            show_files,
            self.show_hidden,
            self.follow_symlinks,
        )? {
            ToggleResult::Found(msg) => msg,
            ToggleResult::NotFound => None,
        };
        self.rebuild_flat_list();
        Ok(error_msg)
    }

    fn toggle_node_recursive(
        node: &TreeNodeRef,
        target_path: &Path,
        show_files: bool,
        show_hidden: bool,
        follow_symlinks: bool,
    ) -> Result<ToggleResult> {
        // Check if this is the target node
        {
            let mut node_borrowed = node.borrow_mut();
            if node_borrowed.path == target_path {
                node_borrowed.toggle_expand(show_files, show_hidden, follow_symlinks)?;
                let error_msg = if node_borrowed.has_error {
                    node_borrowed.error_message.clone()
                } else {
                    None
                };
                return Ok(ToggleResult::Found(error_msg));
            }
        }

        // Recursively search children; stop as soon as the target is found.
        let children_count = node.borrow().children.len();
        for i in 0..children_count {
            let child = Rc::clone(&node.borrow().children[i]);
            if let ToggleResult::Found(msg) = Self::toggle_node_recursive(
                &child,
                target_path,
                show_files,
                show_hidden,
                follow_symlinks,
            )? {
                return Ok(ToggleResult::Found(msg));
            }
        }

        Ok(ToggleResult::NotFound)
    }

    /// Reload tree with new show_files setting
    #[allow(dead_code)]
    pub fn reload_tree(&mut self, show_files: bool) -> Result<()> {
        Self::reload_node_recursive(
            &self.root,
            show_files,
            self.show_hidden,
            self.follow_symlinks,
        )?;
        self.rebuild_flat_list();
        Ok(())
    }

    fn reload_node_recursive(
        node: &TreeNodeRef,
        show_files: bool,
        show_hidden: bool,
        follow_symlinks: bool,
    ) -> Result<()> {
        // Check if we need to reload this node
        let should_reload = {
            let node_borrowed = node.borrow();
            node_borrowed.is_expanded && node_borrowed.is_dir
        };

        if should_reload {
            // Clear children and reload with new mode
            {
                let mut node_borrowed = node.borrow_mut();
                node_borrowed.children.clear();
                node_borrowed.load_children(show_files, show_hidden, follow_symlinks)?;
            }

            // Recursively reload child nodes without cloning
            let children_count = node.borrow().children.len();
            for i in 0..children_count {
                let child = Rc::clone(&node.borrow().children[i]);
                Self::reload_node_recursive(&child, show_files, show_hidden, follow_symlinks)?;
            }
        }
        Ok(())
    }

    /// Move selection to the parent node within the current flat list.
    /// Returns true if a parent was found, false if already at depth 0.
    pub fn select_parent_node(&mut self) -> bool {
        if let Some(node) = self.flat_list.get(self.selected) {
            let depth = node.borrow().depth;
            if depth == 0 {
                return false;
            }
            let target_depth = depth - 1;
            for i in (0..self.selected).rev() {
                if self.flat_list[i].borrow().depth == target_depth {
                    self.selected = i;
                    return true;
                }
            }
        }
        false
    }

    /// Navigate to parent directory
    pub fn go_to_parent(&mut self, show_files: bool) -> Result<()> {
        let parent_path = {
            let root_borrowed = self.root.borrow();
            root_borrowed.path.parent().map(|p| p.to_path_buf())
        };

        if let Some(parent_path) = parent_path {
            let current_path = self.root.borrow().path.clone();

            let mut new_root = TreeNode::new(parent_path, 0)?;
            new_root.load_children(show_files, self.show_hidden, self.follow_symlinks)?;
            new_root.is_expanded = true;

            self.push_history(current_path.clone());
            self.root = Rc::new(RefCell::new(new_root));
            self.rebuild_flat_list();

            // Find and select previous directory using HashMap (O(1) instead of O(n))
            if let Some(&idx) = self.path_to_index.get(&current_path) {
                self.selected = idx;
            }
        }

        Ok(())
    }

    /// Navigate back to the previous root in history.
    /// Returns `true` if navigation occurred, `false` if history is empty.
    pub fn go_back(&mut self, show_files: bool) -> Result<bool> {
        let Some(prev_path) = self.history.pop_back() else {
            return Ok(false);
        };

        let mut new_root = TreeNode::new(prev_path, 0)?;
        new_root.load_children(show_files, self.show_hidden, self.follow_symlinks)?;
        new_root.is_expanded = true;

        if new_root.has_error {
            return Ok(false);
        }

        self.root = Rc::new(RefCell::new(new_root));
        self.rebuild_flat_list();
        self.selected = 0;
        Ok(true)
    }

    /// Push path to history, capping at 50 entries.
    fn push_history(&mut self, path: PathBuf) {
        if self.history.len() >= 50 {
            self.history.pop_front();
        }
        self.history.push_back(path);
    }

    /// Navigate to arbitrary directory (for bookmarks).
    ///
    /// Returns `Some(error_message)` when the path does not exist, is not a directory,
    /// or cannot be read. Returns `None` on success. The error is also stored in
    /// [`Navigation::nav_error`] and displayed in the UI header until the next
    /// successful navigation.
    pub fn go_to_directory(
        &mut self,
        target_path: PathBuf,
        show_files: bool,
    ) -> Result<Option<String>> {
        if !target_path.exists() {
            let msg = format!("Directory not found: {}", target_path.display());
            self.nav_error = Some(msg.clone());
            return Ok(Some(msg));
        }
        if !target_path.is_dir() {
            let msg = format!("Not a directory: {}", target_path.display());
            self.nav_error = Some(msg.clone());
            return Ok(Some(msg));
        }

        // Save current state in case we need to restore it
        let old_root = Rc::clone(&self.root);
        let old_selected = self.selected;

        let mut new_root = TreeNode::new(target_path, 0)?;
        new_root.load_children(show_files, self.show_hidden, self.follow_symlinks)?;
        new_root.is_expanded = true;

        // Check if the new root has an error
        if new_root.has_error {
            // Restore previous state - don't change directory
            self.root = old_root;
            self.selected = old_selected;
            let msg = new_root.error_message.clone();
            if let Some(ref m) = msg {
                self.nav_error = Some(m.clone());
            }
            return Ok(msg);
        }

        // Success - record current root in history before switching
        let current_path = old_root.borrow().path.clone();
        self.push_history(current_path);

        self.nav_error = None;
        self.root = Rc::new(RefCell::new(new_root));
        self.rebuild_flat_list();
        self.selected = 0;

        Ok(None)
    }

    /// Expand path to node (for search results)
    pub fn expand_path_to_node(&mut self, target_path: &PathBuf, show_files: bool) -> Result<()> {
        Self::expand_path_recursive(
            &self.root,
            target_path,
            show_files,
            self.show_hidden,
            self.follow_symlinks,
        )?;
        self.rebuild_flat_list();

        // Find and select element in tree using HashMap (O(1) instead of O(n))
        if let Some(&idx) = self.path_to_index.get(target_path) {
            self.selected = idx;
        }

        Ok(())
    }

    fn expand_path_recursive(
        node: &TreeNodeRef,
        target_path: &PathBuf,
        show_files: bool,
        show_hidden: bool,
        follow_symlinks: bool,
    ) -> Result<bool> {
        // Check if this is the target node or if target is a descendant
        {
            let mut node_borrowed = node.borrow_mut();

            // If this is the target node, do nothing
            if &node_borrowed.path == target_path {
                return Ok(true);
            }

            // Check if target_path is a descendant of current node
            if !target_path.starts_with(&node_borrowed.path) {
                return Ok(false);
            }

            // Load children if needed
            if node_borrowed.children.is_empty() && node_borrowed.is_dir {
                node_borrowed.load_children(show_files, show_hidden, follow_symlinks)?;
            }

            // Expand current node
            node_borrowed.is_expanded = true;
        }

        // Recursively search in children without cloning
        let children_count = node.borrow().children.len();
        for i in 0..children_count {
            let child = Rc::clone(&node.borrow().children[i]);
            if Self::expand_path_recursive(
                &child,
                target_path,
                show_files,
                show_hidden,
                follow_symlinks,
            )? {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_nav(path: PathBuf) -> Navigation {
        Navigation::new(path, false, false, false).expect("Navigation::new failed")
    }

    #[test]
    fn go_back_round_trips_after_go_to_directory() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();
        let child = root.join("child");
        std::fs::create_dir(&child).unwrap();

        let mut nav = make_nav(root.clone());
        assert_eq!(nav.history.len(), 0);

        nav.go_to_directory(child.clone(), false).unwrap();
        assert_eq!(nav.root.borrow().path, child);
        assert_eq!(nav.history.len(), 1);
        assert_eq!(nav.history[0], root);

        let went_back = nav.go_back(false).unwrap();
        assert!(went_back);
        assert_eq!(nav.root.borrow().path, root);
        assert_eq!(nav.history.len(), 0);
    }

    #[test]
    fn failed_navigation_does_not_push_history() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();
        let nonexistent = root.join("does_not_exist");

        let mut nav = make_nav(root.clone());
        let result = nav.go_to_directory(nonexistent.clone(), false).unwrap();

        assert!(
            result.is_some(),
            "must return an error message for nonexistent path"
        );
        let msg = result.unwrap();
        assert!(
            msg.contains(nonexistent.to_str().unwrap()),
            "error message must mention the path"
        );
        assert_eq!(nav.nav_error.as_deref(), Some(msg.as_str()));
        assert_eq!(nav.root.borrow().path, root, "root must not change");
        assert_eq!(nav.history.len(), 0, "history must not grow on failed nav");
    }

    #[test]
    fn successful_navigation_clears_nav_error() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();
        let child = root.join("child");
        std::fs::create_dir(&child).unwrap();
        let nonexistent = root.join("does_not_exist");

        let mut nav = make_nav(root.clone());
        nav.go_to_directory(nonexistent, false).unwrap();
        assert!(
            nav.nav_error.is_some(),
            "error should be set after failed nav"
        );

        nav.go_to_directory(child.clone(), false).unwrap();
        assert!(
            nav.nav_error.is_none(),
            "error must be cleared after successful nav"
        );
    }

    #[test]
    fn go_back_does_not_push_history() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();
        let child = root.join("child");
        std::fs::create_dir(&child).unwrap();

        let mut nav = make_nav(root.clone());
        nav.go_to_directory(child.clone(), false).unwrap();
        assert_eq!(nav.history.len(), 1);

        nav.go_back(false).unwrap();
        assert_eq!(nav.history.len(), 0, "go_back must not push to history");

        let went_back = nav.go_back(false).unwrap();
        assert!(!went_back, "go_back on empty history returns false");
    }

    #[test]
    fn go_to_parent_pushes_history() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();
        let child = root.join("child");
        std::fs::create_dir(&child).unwrap();

        let mut nav = make_nav(child.clone());
        assert_eq!(nav.history.len(), 0);

        nav.go_to_parent(false).unwrap();
        assert_eq!(nav.history.len(), 1);
        assert_eq!(nav.history[0], child);
    }
}
