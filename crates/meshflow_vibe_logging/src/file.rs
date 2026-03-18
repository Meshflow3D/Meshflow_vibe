use crate::{LogCategory, LogEntry};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::OnceLock;

/// Storage for the log file path.
///
/// Uses `OnceLock` to ensure the path is only computed once and is
/// safe for concurrent access.
static LOG_FILE_PATH: OnceLock<PathBuf> = OnceLock::new();

/// Gets the path to the log file.
///
/// The log file is stored in the platform-specific config directory:
/// - **Linux**: `~/.config/meshflow_vibe_logging/app.log`
/// - **macOS**: `~/Library/Application Support/meshflow_vibe_logging/app.log`
/// - **Windows**: `%APPDATA%/meshflow_vibe_logging/app.log`
///
/// If the config directory cannot be determined, falls back to the
/// current working directory.
///
/// The directory is created automatically if it doesn't exist.
///
/// # Returns
///
/// A reference to the `PathBuf` representing the log file path.
fn get_log_path() -> &'static PathBuf {
    LOG_FILE_PATH.get_or_init(|| {
        let path = dirs::config_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap())
            .join("meshflow_vibe_logging")
            .join("app.log");

        if let Some(parent) = path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!("Failed to create log directory: {}", e);
            }
        }

        path
    })
}

/// Writes a log entry to the persistent log file.
///
/// This function appends the log entry to the log file. Blank category
/// entries are skipped to avoid unnecessary file I/O.
///
/// # Arguments
///
/// * `entry` - A reference to the `LogEntry` to write
///
/// # Returns
///
/// * `Ok(())` if the entry was written successfully
/// * `Err(std::io::Error)` if there was a file I/O error
///
/// # Example
///
/// ```rust,no_run
/// use meshflow_vibe_logging::{LogEntry, LogCategory, LogLevel, LogType, write_to_file};
///
/// let entry = LogEntry {
///     timestamp: "12/31/2025-23:59:59 ".to_string(),
///     log_type: LogType::Game,
///     level: LogLevel::Info,
///     category: LogCategory::System,
///     message: "Test log".to_string(),
/// };
///
/// if let Err(e) = write_to_file(&entry) {
///     eprintln!("Failed to write log: {}", e);
/// }
/// ```
pub fn write_to_file(entry: &LogEntry) -> Result<(), std::io::Error> {
    if entry.category == LogCategory::Blank {
        return Ok(()); // Skip blank log entries
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(get_log_path())?;

    let log_line = format!(
        "{}{:?} {:?} {:?} {}\n",
        entry.timestamp, entry.log_type, entry.level, entry.category, entry.message
    );

    file.write_all(log_line.as_bytes())?;
    Ok(())
}
