use anyhow::Result;
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

pub type TreeNodeRef = Rc<RefCell<TreeNode>>;

pub struct TreeNode {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub is_expanded: bool,
    pub depth: usize,
    pub children: Vec<TreeNodeRef>,
    pub has_error: bool,               // Indicates read/access errors
    pub error_message: Option<String>, // Optional error description
    /// Whether this directory has any visible children under the current settings.
    /// `None` means unknown (not yet probed); `Some(false)` means leaf — do not show `>`.
    pub has_children: Option<bool>,
    is_sorted: bool, // Cache flag: true if children are already sorted
}

impl TreeNode {
    pub fn new(path: PathBuf, depth: usize) -> Result<Self> {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let is_dir = path.is_dir();

        Ok(TreeNode {
            path,
            name,
            is_dir,
            is_expanded: false,
            depth,
            children: Vec::new(),
            has_error: false,
            error_message: None,
            has_children: None,
            is_sorted: false,
        })
    }

    /// Probe whether this directory has any visible children under the given settings,
    /// without fully loading children. Result is cached in `has_children`.
    /// IMPORTANT: the filtering logic here must stay in sync with `load_children`.
    pub fn probe_has_children(
        &mut self,
        show_files: bool,
        show_hidden: bool,
        follow_symlinks: bool,
    ) {
        if !self.is_dir {
            self.has_children = Some(false);
            return;
        }
        let Ok(entries) = fs::read_dir(&self.path) else {
            return; // Unreadable — leave as None (unknown)
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if !follow_symlinks {
                if let Ok(meta) = fs::symlink_metadata(&path) {
                    if meta.is_symlink() {
                        continue;
                    }
                }
            }
            if !show_hidden {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with('.') {
                        continue;
                    }
                }
            }
            if path.is_dir() || show_files {
                self.has_children = Some(true);
                return;
            }
        }
        self.has_children = Some(false);
    }

    pub fn load_children(
        &mut self,
        show_files: bool,
        show_hidden: bool,
        follow_symlinks: bool,
    ) -> Result<()> {
        // If children are already loaded and sorted, skip
        if !self.is_dir || (!self.children.is_empty() && self.is_sorted) {
            return Ok(());
        }

        // If we're reloading (children exist but not sorted), clear them first
        if !self.children.is_empty() {
            self.children.clear();
            self.is_sorted = false;
            self.has_children = None;
        }

        // Try to read directory
        let entries = match fs::read_dir(&self.path) {
            Ok(entries) => entries,
            Err(e) => {
                // Mark this node as having an error
                self.has_error = true;
                self.error_message = Some(format!("Cannot read: {}", e));
                return Ok(()); // Don't propagate error, just mark the node
            }
        };

        let mut error_count = 0;
        let mut skipped_entries = Vec::new();

        // Process entries, tracking errors
        for entry in entries {
            match entry {
                Ok(entry) => {
                    let path = entry.path();

                    // Check if entry is a symlink and whether to follow it
                    if !follow_symlinks {
                        if let Ok(metadata) = fs::symlink_metadata(&path) {
                            if metadata.is_symlink() {
                                continue; // Skip symlinks if follow_symlinks is false
                            }
                        }
                    }

                    let is_dir = path.is_dir();

                    // Check if file/directory is hidden (starts with .)
                    if !show_hidden {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            if name.starts_with('.') {
                                continue; // Skip hidden files/directories
                            }
                        }
                    }

                    // Show directories always, files only if show_files == true
                    if is_dir || show_files {
                        match TreeNode::new(path.clone(), self.depth + 1) {
                            Ok(node) => {
                                self.children.push(Rc::new(RefCell::new(node)));
                            }
                            Err(e) => {
                                error_count += 1;
                                skipped_entries.push(format!(
                                    "{}: {}",
                                    path.file_name().unwrap_or_default().to_string_lossy(),
                                    e
                                ));
                            }
                        }
                    }
                }
                Err(e) => {
                    error_count += 1;
                    skipped_entries.push(format!("unknown entry: {}", e));
                }
            }
        }

        // If we had errors, mark the node and store summary
        if error_count > 0 {
            self.has_error = true;
            if error_count <= 3 {
                self.error_message = Some(skipped_entries.join(", "));
            } else {
                self.error_message = Some(format!("{} entries inaccessible", error_count));
            }
        }

        // Sort: directories first, then files, sorted by name within each group
        self.children.sort_by(|a, b| {
            let a_borrowed = a.borrow();
            let b_borrowed = b.borrow();
            match (a_borrowed.is_dir, b_borrowed.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a_borrowed.name.cmp(&b_borrowed.name),
            }
        });

        // Mark as sorted so we don't re-sort on next load
        self.is_sorted = true;

        // Update has_children for self based on loaded children
        self.has_children = Some(!self.children.is_empty());

        // Probe each child so the UI knows whether to show ">" for it.
        // IMPORTANT: filtering in probe_has_children must stay in sync with load_children.
        for child_ref in &self.children {
            child_ref
                .borrow_mut()
                .probe_has_children(show_files, show_hidden, follow_symlinks);
        }

        Ok(())
    }

    pub fn toggle_expand(
        &mut self,
        show_files: bool,
        show_hidden: bool,
        follow_symlinks: bool,
    ) -> Result<()> {
        if !self.is_dir {
            return Ok(());
        }

        // Leaf directories (confirmed empty) cannot be expanded
        if self.has_children == Some(false) {
            return Ok(());
        }

        if self.is_expanded {
            self.is_expanded = false;
        } else {
            self.load_children(show_files, show_hidden, follow_symlinks)?;
            // Only expand if no access error occurred
            if !self.has_error {
                self.is_expanded = true;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_dir_node(path: PathBuf) -> TreeNode {
        TreeNode::new(path, 0).expect("TreeNode::new failed")
    }

    #[test]
    fn leaf_dir_has_children_false_after_load() {
        let tmp = TempDir::new().unwrap();
        let mut node = make_dir_node(tmp.path().to_path_buf());
        node.load_children(false, false, false).unwrap();
        assert_eq!(
            node.has_children,
            Some(false),
            "empty dir should be Some(false)"
        );
    }

    #[test]
    fn dir_with_subdir_has_children_true_after_load() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir(tmp.path().join("sub")).unwrap();
        let mut node = make_dir_node(tmp.path().to_path_buf());
        node.load_children(false, false, false).unwrap();
        assert_eq!(node.has_children, Some(true));
    }

    #[test]
    fn child_leaf_gets_has_children_false_after_parent_load() {
        let tmp = TempDir::new().unwrap();
        let child = tmp.path().join("leaf");
        std::fs::create_dir(&child).unwrap();

        let mut root = make_dir_node(tmp.path().to_path_buf());
        root.load_children(false, false, false).unwrap();

        let child_node = root.children[0].borrow();
        assert_eq!(
            child_node.has_children,
            Some(false),
            "leaf child should be probed as Some(false)"
        );
    }

    #[test]
    fn child_with_subdir_gets_has_children_true_after_parent_load() {
        let tmp = TempDir::new().unwrap();
        let child = tmp.path().join("inner");
        std::fs::create_dir(&child).unwrap();
        std::fs::create_dir(child.join("nested")).unwrap();

        let mut root = make_dir_node(tmp.path().to_path_buf());
        root.load_children(false, false, false).unwrap();

        let child_node = root.children[0].borrow();
        assert_eq!(child_node.has_children, Some(true));
    }

    #[test]
    fn toggle_expand_does_not_expand_leaf_dir() {
        let tmp = TempDir::new().unwrap();
        let mut node = make_dir_node(tmp.path().to_path_buf());
        // Probe first (parent would normally do this)
        node.probe_has_children(false, false, false);
        assert_eq!(node.has_children, Some(false));

        node.toggle_expand(false, false, false).unwrap();
        assert!(!node.is_expanded, "leaf dir must not expand");
    }

    #[test]
    fn toggle_expand_expands_dir_with_subdir() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir(tmp.path().join("child")).unwrap();
        let mut node = make_dir_node(tmp.path().to_path_buf());
        node.probe_has_children(false, false, false);
        assert_eq!(node.has_children, Some(true));

        node.toggle_expand(false, false, false).unwrap();
        assert!(node.is_expanded);
    }
}
