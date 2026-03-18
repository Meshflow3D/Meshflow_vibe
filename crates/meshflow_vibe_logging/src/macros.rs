/// Main logging macro for the meshflow_vibe_logging crate.
///
/// This macro provides a convenient interface for logging messages with
/// varying levels of specificity. It supports multiple call patterns:
///
/// # Call Patterns
///
/// ## Pattern 1: Full Specification
///
/// ```rust,no_run
/// use meshflow_vibe_logging::{log, LogType, LogLevel, LogCategory};
///
/// log!(
///     LogType::Game,
///     LogLevel::Error,
///     LogCategory::Asset,
///     "Failed to load asset: {}",
///     "player_model.obj"
/// );
/// ```
///
/// ## Pattern 2: Level and Category
///
/// ```rust,no_run
/// use meshflow_vibe_logging::{log, LogLevel, LogCategory};
///
/// log!(
///     LogLevel::Warning,
///     LogCategory::UI,
///     "UI element not found"
/// );
/// ```
///
/// ## Pattern 3: Category Only
///
/// ```rust,no_run
/// use meshflow_vibe_logging::{log, LogCategory};
///
/// log!(cat: LogCategory::Input, "Mouse moved to ({}, {})", 100, 200);
/// ```
///
/// ## Pattern 4: Message Only (Most Common)
///
/// ```rust,no_run
/// use meshflow_vibe_logging::log;
///
/// log!("Application started successfully");
/// log!("User {} logged in", "john_doe");
/// log!("Processing {} items", 42);
/// ```
///
/// # Default Values
///
/// When not specified, the following defaults are used:
/// - **Log Type**: `LogType::Game`
/// - **Log Level**: `LogLevel::Info`
/// - **Log Category**: `LogCategory::Blank`
///
/// # Formatting
///
/// The macro supports Rust's standard formatting syntax via the `$($arg:tt)+`
/// pattern, which accepts format strings and arguments:
///
/// ```rust,no_run
/// use meshflow_vibe_logging::log;
///
/// let error_code = 42;
/// log!("Error {}: Operation failed", error_code);
/// ```
///
/// # Output
///
/// Each log call:
/// 1. Creates a `LogEntry` with a timestamp
/// 2. Adds the entry to the in-memory buffer
/// 3. Writes to the persistent log file
/// 4. Outputs to console (if not filtered out)
///
/// # Thread Safety
///
/// The macro is thread-safe and can be called from any thread.
#[macro_export]
macro_rules! log {
    // Full specification: type, level, category, message + args
    ($type:expr, $level:expr, $category:expr, $($arg:tt)+) => {
        $crate::output::log($type, $level, $category, format!($($arg)+));
    };

    // Level, category, message + args (defaults to Game type)
    ($level:expr, $category:expr, $($arg:tt)+) => {
        $crate::output::log($crate::config::LogType::Game, $level, $category, format!($($arg)+));
    };

    // Category, message + args (defaults to Game type, Info level)
    (cat: $category:expr, $($arg:tt)+) => {
        $crate::output::log($crate::config::LogType::Game, $crate::config::LogLevel::Info, $category, format!($($arg)+));
    };

    // Just message + args (defaults: Game type, Info level, Blank category)
    ($($arg:tt)+) => {
        $crate::output::log(
            $crate::config::LogType::Game,
            $crate::config::LogLevel::Info,
            $crate::config::LogCategory::Blank,
            format!($($arg)+)
        );
    };
}
