use chrono::{DateTime, Local};
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::HashMap;
use std::fs::Metadata;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

/// Maximum calculation time per directory (5 seconds)
const CALCULATION_TIMEOUT: Duration = Duration::from_secs(5);

/// Maximum number of files to process per directory (to prevent hanging)
const MAX_FILES_TO_PROCESS: usize = 10000;

/// Detailed information about a directory collected by the background worker
#[derive(Debug, Clone)]
pub struct DirInfo {
    pub size: u64,
    pub is_partial: bool,
    pub file_count: u64,
    pub dir_count: u64,
    pub permissions: String,
    pub mtime: String,
}

/// Message types for communication between main thread and size calculation thread
#[derive(Debug)]
pub enum SizeMessage {
    /// Result found (path, info)
    Result(PathBuf, DirInfo),
    /// Calculation done for a path
    Done(PathBuf),
}

/// Task message for worker thread
#[derive(Debug)]
enum TaskMessage {
    Calculate(PathBuf),
    Shutdown,
}

/// Accumulated result of a recursive size traversal
struct CalculationResult {
    size: u64,
    is_partial: bool,
    file_count: u64,
    dir_count: u64,
}

/// Cache for directory sizes with async calculation support
pub struct DirSizeCache {
    /// Cache mapping path to DirInfo (populated only on full or partial completion)
    cache: HashMap<PathBuf, DirInfo>,
    /// Immediately-populated metadata (permissions, mtime) keyed by path;
    /// survives `cancel()` so the UI can always show non-size info.
    metadata_cache: HashMap<PathBuf, (String, String)>,
    /// Paths currently being calculated
    calculating: Arc<Mutex<Vec<PathBuf>>>,
    /// Channel for receiving calculation results
    result_receiver: Option<Receiver<SizeMessage>>,
    /// Channel for sending calculation tasks to worker
    task_sender: Option<Sender<TaskMessage>>,
    /// Handle to background worker thread
    worker_handle: Option<thread::JoinHandle<()>>,
    /// Shared flag: set to `true` to make the worker exit its recursive traversal quickly
    cancel_flag: Arc<AtomicBool>,
}

impl Default for DirSizeCache {
    fn default() -> Self {
        Self::new()
    }
}

impl DirSizeCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            metadata_cache: HashMap::new(),
            calculating: Arc::new(Mutex::new(Vec::new())),
            result_receiver: None,
            task_sender: None,
            worker_handle: None,
            cancel_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Initialize worker thread if not already running.
    fn ensure_worker_running(&mut self) {
        if self.worker_handle.is_some() {
            return;
        }

        // Reset cancel flag so the new worker doesn't exit immediately.
        self.cancel_flag.store(false, Ordering::Relaxed);

        let (task_tx, task_rx) = unbounded();
        let (result_tx, result_rx) = unbounded();

        let calculating = Arc::clone(&self.calculating);
        let cancel_flag = Arc::clone(&self.cancel_flag);

        let handle = thread::spawn(move || {
            worker_loop(task_rx, result_tx, calculating, cancel_flag);
        });

        self.task_sender = Some(task_tx);
        self.result_receiver = Some(result_rx);
        self.worker_handle = Some(handle);
    }

    /// Get cached info for a path
    pub fn get(&self, path: &Path) -> Option<&DirInfo> {
        self.cache.get(path)
    }

    /// Check if a path is currently being calculated
    pub fn is_calculating(&self, path: &Path) -> bool {
        if let Ok(calculating) = self.calculating.lock() {
            calculating.contains(&path.to_path_buf())
        } else {
            false
        }
    }

    /// Check if any calculation is in progress
    pub fn is_any_calculating(&self) -> bool {
        if let Ok(calculating) = self.calculating.lock() {
            !calculating.is_empty()
        } else {
            false
        }
    }

    /// Start async calculation for a directory.
    ///
    /// Immediately caches permissions and mtime (fast stat) so the UI can display
    /// them before the slow recursive size traversal completes.
    pub fn calculate_async(&mut self, path: PathBuf) {
        if self.cache.contains_key(&path) || self.is_calculating(&path) {
            return;
        }

        // Fast metadata read — always available right away regardless of how long size takes.
        if let Ok(meta) = std::fs::metadata(&path) {
            self.metadata_cache.insert(
                path.clone(),
                (format_permissions(&meta), format_mtime(&meta)),
            );
        }

        self.ensure_worker_running();

        if let Ok(mut calculating) = self.calculating.lock() {
            calculating.push(path.clone());
        }

        if let Some(sender) = &self.task_sender {
            let _ = sender.send(TaskMessage::Calculate(path));
        }
    }

    /// Return cached `(permissions, mtime)` for a path if available.
    ///
    /// This is populated immediately when `calculate_async` is called and
    /// survives `cancel()`, so callers can always show non-size metadata.
    pub fn get_metadata(&self, path: &Path) -> Option<(&str, &str)> {
        self.metadata_cache
            .get(path)
            .map(|(p, m)| (p.as_str(), m.as_str()))
    }

    /// Poll for calculation results; returns true if there were updates
    pub fn poll_results(&mut self) -> bool {
        let mut updated = false;

        if let Some(receiver) = &self.result_receiver {
            while let Ok(msg) = receiver.try_recv() {
                match msg {
                    SizeMessage::Result(path, info) => {
                        self.cache.insert(path, info);
                        updated = true;
                    }
                    SizeMessage::Done(path) => {
                        if let Ok(mut calculating) = self.calculating.lock() {
                            calculating.retain(|p| p != &path);
                        }
                    }
                }
            }
        }

        updated
    }

    /// Cancel ongoing calculations without blocking.
    ///
    /// Sets the cancel flag so the worker exits its recursive traversal at the
    /// next check point (every ~100 entries), then detaches the thread rather
    /// than joining it.  Already-completed cache entries are preserved;
    /// `metadata_cache` is also preserved so the UI keeps showing permissions
    /// and mtime for directories whose size calculation was stopped.
    pub fn cancel(&mut self) {
        // Signal the running calculation to stop as soon as it next checks.
        self.cancel_flag.store(true, Ordering::Relaxed);

        if let Some(sender) = &self.task_sender {
            let _ = sender.send(TaskMessage::Shutdown);
        }

        self.task_sender = None;
        self.result_receiver = None;
        // Drop without joining — the thread will terminate on its own once it
        // notices cancel_flag or finds task_rx closed.
        self.worker_handle = None;

        if let Ok(mut calculating) = self.calculating.lock() {
            calculating.clear();
        }
    }

    /// Clear both caches and shutdown the worker.
    pub fn clear(&mut self) {
        self.cancel();
        self.cache.clear();
        self.metadata_cache.clear();
    }

    /// Format size in human-readable format
    pub fn format_size(size: u64, is_partial: bool) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        const TB: u64 = GB * 1024;

        let prefix = if is_partial { ">" } else { "" };

        if size >= TB {
            format!("{}{:.1}T", prefix, size as f64 / TB as f64)
        } else if size >= GB {
            format!("{}{:.1}G", prefix, size as f64 / GB as f64)
        } else if size >= MB {
            format!("{}{:.1}M", prefix, size as f64 / MB as f64)
        } else if size >= KB {
            format!("{}{:.1}K", prefix, size as f64 / KB as f64)
        } else {
            format!("{}{}B", prefix, size)
        }
    }
}

impl Drop for DirSizeCache {
    fn drop(&mut self) {
        self.cancel();
    }
}

/// Format filesystem permissions from metadata.
/// Unix: `drwxr-xr-x` style. Windows: `rw` or `ro`.
pub fn format_permissions(metadata: &Metadata) -> String {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode();
        let ft = if metadata.is_dir() {
            'd'
        } else if metadata.file_type().is_symlink() {
            'l'
        } else {
            '-'
        };
        let bits = [
            (mode & 0o400, 'r'),
            (mode & 0o200, 'w'),
            (mode & 0o100, 'x'),
            (mode & 0o040, 'r'),
            (mode & 0o020, 'w'),
            (mode & 0o010, 'x'),
            (mode & 0o004, 'r'),
            (mode & 0o002, 'w'),
            (mode & 0o001, 'x'),
        ];
        let perm: String = bits
            .iter()
            .map(|(b, c)| if *b != 0 { *c } else { '-' })
            .collect();
        format!("{}{}", ft, perm)
    }
    #[cfg(not(unix))]
    {
        if metadata.permissions().readonly() {
            "ro".to_string()
        } else {
            "rw".to_string()
        }
    }
}

/// Format last-modified time as `DD.MM.YYYY HH:MM` in local time.
pub fn format_mtime(metadata: &Metadata) -> String {
    if let Ok(modified) = metadata.modified() {
        let dt: DateTime<Local> = modified.into();
        dt.format("%d.%m.%Y %H:%M").to_string()
    } else {
        String::new()
    }
}

/// Worker thread loop that processes calculation tasks.
fn worker_loop(
    task_rx: Receiver<TaskMessage>,
    result_tx: Sender<SizeMessage>,
    calculating: Arc<Mutex<Vec<PathBuf>>>,
    cancel_flag: Arc<AtomicBool>,
) {
    loop {
        match task_rx.recv() {
            Ok(TaskMessage::Calculate(path)) => {
                let start_time = Instant::now();
                let mut processed = 0usize;

                let result =
                    calculate_dir_size_limited(&path, start_time, &mut processed, &cancel_flag);

                let (permissions, mtime) = if let Ok(meta) = std::fs::metadata(&path) {
                    (format_permissions(&meta), format_mtime(&meta))
                } else {
                    (String::new(), String::new())
                };

                let info = DirInfo {
                    size: result.size,
                    is_partial: result.is_partial,
                    file_count: result.file_count,
                    dir_count: result.dir_count,
                    permissions,
                    mtime,
                };

                // These sends fail silently if cancel() dropped the receiver — that's fine.
                let _ = result_tx.send(SizeMessage::Result(path.clone(), info));
                let _ = result_tx.send(SizeMessage::Done(path));
            }
            Ok(TaskMessage::Shutdown) | Err(_) => {
                if let Ok(mut calc) = calculating.lock() {
                    calc.clear();
                }
                break;
            }
        }
    }
}

/// Calculate total size of a directory recursively with limits.
///
/// `processed` tracks total files seen across recursive calls (for timeout/limit checks).
/// `cancel_flag` is checked at every recursive entry and every 100 entries within a directory.
fn calculate_dir_size_limited(
    path: &Path,
    start_time: Instant,
    processed: &mut usize,
    cancel_flag: &Arc<AtomicBool>,
) -> CalculationResult {
    // Check at the start of every recursive call for fast response to cancellation.
    if cancel_flag.load(Ordering::Relaxed) {
        return CalculationResult {
            size: 0,
            is_partial: true,
            file_count: 0,
            dir_count: 0,
        };
    }
    if start_time.elapsed() > CALCULATION_TIMEOUT {
        return CalculationResult {
            size: 0,
            is_partial: true,
            file_count: 0,
            dir_count: 0,
        };
    }
    if *processed >= MAX_FILES_TO_PROCESS {
        return CalculationResult {
            size: 0,
            is_partial: true,
            file_count: 0,
            dir_count: 0,
        };
    }

    let mut total_size = 0u64;
    let mut is_partial = false;
    let mut file_count = 0u64;
    let mut dir_count = 0u64;

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            // Periodic check — avoids tight-loop overhead while still being responsive.
            if (*processed).is_multiple_of(100)
                && (cancel_flag.load(Ordering::Relaxed)
                    || start_time.elapsed() > CALCULATION_TIMEOUT)
            {
                return CalculationResult {
                    size: total_size,
                    is_partial: true,
                    file_count,
                    dir_count,
                };
            }

            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    total_size += metadata.len();
                    file_count += 1;
                    *processed += 1;
                    if *processed >= MAX_FILES_TO_PROCESS {
                        return CalculationResult {
                            size: total_size,
                            is_partial: true,
                            file_count,
                            dir_count,
                        };
                    }
                } else if metadata.is_dir() {
                    dir_count += 1;
                    let sub = calculate_dir_size_limited(
                        &entry.path(),
                        start_time,
                        processed,
                        cancel_flag,
                    );
                    total_size += sub.size;
                    file_count += sub.file_count;
                    dir_count += sub.dir_count;
                    if sub.is_partial {
                        is_partial = true;
                        break;
                    }
                }
            }
        }
    }

    CalculationResult {
        size: total_size,
        is_partial,
        file_count,
        dir_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_cancel_does_not_block() {
        // Build a directory tree large enough that calculation takes > 50 ms without cancel.
        let base = std::env::temp_dir().join("dtree_test_dir_cancel");
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();

        // Create 5 subdirectories, each with 200 small files (~1000 files total).
        for i in 0..5 {
            let sub = base.join(format!("sub{}", i));
            fs::create_dir_all(&sub).unwrap();
            for j in 0..200 {
                fs::write(sub.join(format!("file{}.txt", j)), b"x").unwrap();
            }
        }

        let mut cache = DirSizeCache::new();
        cache.calculate_async(base.clone());

        // Give the worker thread a moment to start calculating.
        std::thread::sleep(Duration::from_millis(20));

        let t0 = Instant::now();
        cache.cancel();
        let elapsed = t0.elapsed();

        // cancel() must return almost immediately — well under 200 ms.
        assert!(
            elapsed < Duration::from_millis(200),
            "cancel() took too long: {:?}",
            elapsed
        );

        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn test_metadata_available_immediately() {
        let base = std::env::temp_dir().join("dtree_test_dir_meta");
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();

        let mut cache = DirSizeCache::new();
        cache.calculate_async(base.clone());

        // Metadata must be available without waiting for the size calculation.
        assert!(
            cache.get_metadata(&base).is_some(),
            "metadata_cache should be populated immediately after calculate_async"
        );

        cache.cancel();

        // Metadata must survive cancel().
        assert!(
            cache.get_metadata(&base).is_some(),
            "metadata_cache should be preserved after cancel()"
        );

        let _ = fs::remove_dir_all(&base);
    }
}
