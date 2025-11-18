// Voltech file parsing and database integration module
pub mod parser;
pub mod file_watcher;
pub mod operations;
pub mod commands;
pub mod queries;

use std::sync::Arc;
use tokio::task::JoinHandle;
use tokio::sync::Mutex;

/// State for the voltech file watcher background task
pub struct WatcherState {
    /// Handle to the background watcher task
    pub handle: JoinHandle<()>,
    /// Pause signal for coordinating with import operations
    pub pause_signal: Arc<Mutex<bool>>,
    /// Unique identifier for this watcher instance (for master/follower pattern)
    pub instance_id: String,
}

impl WatcherState {
    pub fn new(handle: JoinHandle<()>, pause_signal: Arc<Mutex<bool>>, instance_id: String) -> Self {
        Self {
            handle,
            pause_signal,
            instance_id,
        }
    }
}
