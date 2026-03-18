use crate::entry::LogEntry;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

/// Maximum number of log entries to keep in the in-memory buffer.
///
/// When this limit is exceeded, older entries are automatically removed
/// to prevent unbounded memory growth.
const MAX_LOG_ENTRIES: usize = 7_500;

/// In-memory buffer for storing recent log entries.
///
/// This buffer maintains up to 7,500 of the most recent log entries and is
/// primarily used for UI display and programmatic access to log history.
///
/// # Thread Safety
///
/// The buffer is wrapped in an `Arc<Mutex<>>` for safe concurrent access
/// from multiple threads.
///
/// # Usage
///
/// ```rust,no_run
/// use meshflow_vibe_logging::LOG_BUFFER;
/// use std::sync::MutexGuard;
///
/// // Read from the buffer
/// {
///     let buffer = LOG_BUFFER.lock().unwrap();
///     for entry in buffer.iter() {
///         println!("{:?}: {}", entry.category, entry.message);
///     }
/// }
///
/// // Get the last N entries
/// {
///     let buffer = LOG_BUFFER.lock().unwrap();
///     let recent: Vec<_> = buffer.iter().rev().take(10).collect();
/// }
/// ```
lazy_static! {
    pub static ref LOG_BUFFER: Arc<Mutex<Vec<LogEntry>>> = Arc::new(Mutex::new(Vec::new()));
}

/// Adds a log entry to the in-memory buffer.
///
/// This function automatically maintains the buffer size by removing
/// older entries when the maximum capacity is exceeded.
///
/// # Arguments
///
/// * `entry` - The `LogEntry` to add to the buffer
///
/// # Example
///
/// ```rust,no_run
/// use meshflow_vibe_logging::{push_log, LogEntry, LogCategory, LogLevel, LogType};
///
/// let entry = LogEntry {
///     timestamp: "12/31/2025-23:59:59 ".to_string(),
///     log_type: LogType::Game,
///     level: LogLevel::Info,
///     category: LogCategory::System,
///     message: "Test log entry".to_string(),
/// };
/// push_log(entry);
/// ```
pub fn push_log(entry: LogEntry) {
    let mut buffer = LOG_BUFFER.lock().unwrap();
    buffer.push(entry);

    let len = buffer.len();
    if len > MAX_LOG_ENTRIES {
        buffer.drain(0..(len - MAX_LOG_ENTRIES));
    }
}
