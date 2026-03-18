use crate::entry::LogEntry;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

/// Maximum number of log entries to keep in the in-memory buffer.
///
/// When this limit is exceeded, older entries are automatically removed
/// to prevent unbounded memory growth.
const MAX_LOG_ENTRIES: usize = 7_500;

lazy_static! {
    pub static ref LOG_BUFFER: Arc<Mutex<Vec<LogEntry>>> = Arc::new(Mutex::new(Vec::new()));
}

/// Adds a log entry to the in-memory buffer.
pub fn push_log(entry: LogEntry) {
    let mut buffer = LOG_BUFFER.lock().unwrap();
    buffer.push(entry);

    let len = buffer.len();
    if len > MAX_LOG_ENTRIES {
        buffer.drain(0..(len - MAX_LOG_ENTRIES));
    }
}
