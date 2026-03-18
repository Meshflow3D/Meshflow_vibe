use crate::config::{LogCategory, LogLevel, LogType};

/// Represents a single log entry in the logging system.
///
/// `LogEntry` is the fundamental unit of logging, containing all information
/// about a log message including its timestamp, type, level, category, and content.
///
/// # Structure
///
/// Log entries are used in multiple places:
/// - **In-memory buffer**: For UI display and recent log access
/// - **File output**: For persistent storage
/// - **Console output**: For real-time monitoring
///
/// # Example
///
/// ```rust,no_run
/// use meshflow_vibe_logging::{LogEntry, LogCategory, LogLevel, LogType};
///
/// let entry = LogEntry {
///     timestamp: "12/31/2025-23:59:59 ".to_string(),
///     log_type: LogType::Game,
///     level: LogLevel::Info,
///     category: LogCategory::System,
///     message: "Application started".to_string(),
/// };
/// ```
#[derive(Clone, Debug)]
pub struct LogEntry {
    /// Timestamp in the format "MM/DD/YYYY-HH:MM:SS "
    /// Generated automatically when the entry is created.
    pub timestamp: String,

    /// The type of application context (Game or Editor)
    pub log_type: LogType,

    /// The severity level of the log message
    pub level: LogLevel,

    /// The category of the log message
    pub category: LogCategory,

    /// The actual log message content
    pub message: String,
}
