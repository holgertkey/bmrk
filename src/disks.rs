use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: PathBuf,
    pub fs_type: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
}

impl DiskInfo {
    fn format_size(bytes: u64) -> String {
        if bytes >= 1_000_000_000_000 {
            format!("{:.1}T", bytes as f64 / 1_000_000_000_000.0)
        } else if bytes >= 1_000_000_000 {
            format!("{:.1}G", bytes as f64 / 1_000_000_000.0)
        } else if bytes >= 1_000_000 {
            format!("{:.1}M", bytes as f64 / 1_000_000.0)
        } else {
            format!("{:.1}K", bytes as f64 / 1_000.0)
        }
    }

    pub fn display_line(&self) -> String {
        let path = self.mount_point.display().to_string();
        let fs = &self.fs_type;
        // Show name only when it differs from the mount point label
        let mount_trimmed = path.trim_end_matches(['/', '\\']);
        let name_suffix = if !self.name.is_empty() && self.name != mount_trimmed {
            format!("  {}", self.name)
        } else {
            String::new()
        };
        if self.total_bytes == 0 {
            format!("{:<12}[{:<6}]{}", path, fs, name_suffix)
        } else {
            let free = Self::format_size(self.available_bytes);
            let total = Self::format_size(self.total_bytes);
            format!(
                "{:<12}[{:<6}]  {:>8} free / {:>8} total{}",
                path, fs, free, total, name_suffix
            )
        }
    }
}

pub struct Disks {
    pub disks: Vec<DiskInfo>,
    pub is_selecting: bool,
    pub selected_index: usize,
}

impl Default for Disks {
    fn default() -> Self {
        Self::new()
    }
}

impl Disks {
    pub fn new() -> Self {
        Self {
            disks: Vec::new(),
            is_selecting: false,
            selected_index: 0,
        }
    }

    pub fn enter_selection_mode(&mut self, current_path: Option<&std::path::Path>) {
        self.disks = enumerate_disks();
        self.is_selecting = true;
        self.selected_index = 0;
        // Pre-select the disk whose mount point is the longest prefix of current_path
        if let Some(path) = current_path {
            let mut best_len = 0usize;
            let mut best_idx = 0usize;
            for (i, disk) in self.disks.iter().enumerate() {
                if path.starts_with(&disk.mount_point) {
                    let len = disk.mount_point.as_os_str().len();
                    if len > best_len {
                        best_len = len;
                        best_idx = i;
                    }
                }
            }
            if best_len > 0 {
                self.selected_index = best_idx;
            }
        }
    }

    pub fn exit_selection_mode(&mut self) {
        self.is_selecting = false;
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if !self.disks.is_empty() && self.selected_index + 1 < self.disks.len() {
            self.selected_index += 1;
        }
    }

    pub fn get_selected(&self) -> Option<&DiskInfo> {
        self.disks.get(self.selected_index)
    }
}

fn enumerate_disks() -> Vec<DiskInfo> {
    use sysinfo::Disks as SysDisks;
    let sys_disks = SysDisks::new_with_refreshed_list();
    let mut result: Vec<DiskInfo> = sys_disks
        .list()
        .iter()
        .map(|d| DiskInfo {
            name: d.name().to_string_lossy().to_string(),
            mount_point: d.mount_point().to_path_buf(),
            fs_type: d.file_system().to_string_lossy().to_string(),
            total_bytes: d.total_space(),
            available_bytes: d.available_space(),
        })
        .collect();
    result.sort_by(|a, b| a.mount_point.cmp(&b.mount_point));
    result
}
