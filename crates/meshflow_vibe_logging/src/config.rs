use std::collections::HashSet;
use std::sync::Mutex;

use crate::log;

/// Represents the category of a log entry, used for organizing and filtering logs.
///
/// Categories help distinguish between different types of events in the application.
/// Each category has a associated color for console output.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum LogCategory {
    /// Entity-related operations and events.
    /// Used for logging entity behavior, state changes, and interactions.
    Entity,

    /// Asset loading, management, and related operations.
    /// Used for tracking asset imports, loading times, and issues.
    Asset,

    /// User interface events and operations.
    /// Used for UI state changes, element creation, and user interactions.
    UI,

    /// User input events (keyboard, mouse, controller).
    /// Used for tracking user actions and input processing.
    Input,

    /// System-level operations and initialization events.
    /// Used for startup, shutdown, and core system functionality.
    System,

    /// Network operations and communication events.
    /// Used for multiplayer, API calls, and data synchronization.
    Network,

    /// Miscellaneous events that don't fit other categories.
    /// Used for unclassified or general-purpose logging.
    Other,

    /// Debug information for development and troubleshooting.
    /// Typically filtered out in production builds.
    Debug,

    /// Blank or separator messages for visual organization.
    /// Used for creating visual breaks in log output.
    Blank,
}

/// Represents the severity level of a log entry.
///
/// Levels are used to filter and prioritize log messages.
/// Higher levels indicate more severe issues.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LogLevel {
    /// Informational messages about normal operation.
    /// Used for tracking program flow and general information.
    Info,

    /// Success messages indicating operations completed successfully.
    /// Used to confirm important actions were completed.
    OK,

    /// Warning messages about potential issues or unusual conditions.
    /// Indicates something may need attention but isn't critical.
    Warning,

    /// Error messages indicating something went wrong.
    /// Operations may have failed or encountered problems.
    Error,

    /// Critical errors that may cause system instability or failure.
    /// Requires immediate attention and may require restart.
    Critical,
}

/// Represents the type or context of the application generating the log.
///
/// This distinction helps separate logging from different parts of the application.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum LogType {
    /// Logs from the game/runtime context.
    /// Used for in-game events, physics, gameplay logic, etc.
    Game,

    /// Logs from the editor/context.
    /// Used for editor tools, scene manipulation, and development features.
    Editor,
}

/// RGBA color representation for log categorization.
///
/// Used to assign consistent colors to log categories and levels
/// for visual differentiation in the console and UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbaColor(pub u8, pub u8, pub u8, pub u8);

impl RgbaColor {
    /// White color (alpha: 255)
    pub const WHITE: Self = Self(240, 240, 240, 255);
    /// Black color (alpha: 255)
    pub const BLACK: Self = Self(20, 20, 20, 255);
    /// Red color (alpha: 255)
    pub const RED: Self = Self(200, 50, 50, 255);
    /// Green color (alpha: 255)
    pub const GREEN: Self = Self(50, 200, 50, 255);
    /// Blue color (alpha: 255)
    pub const BLUE: Self = Self(50, 50, 200, 255);
    /// Yellow color (alpha: 255)
    pub const YELLOW: Self = Self(220, 220, 50, 255);
    /// Cyan color (alpha: 255)
    pub const CYAN: Self = Self(50, 200, 200, 255);
    /// Magenta color (alpha: 255)
    pub const MAGENTA: Self = Self(200, 50, 200, 255);
    /// Gray color (alpha: 255)
    pub const GRAY: Self = Self(100, 100, 100, 255);
}

impl LogType {
    /// Returns all available log types.
    ///
    /// # Returns
    /// A vector containing `Game` and `Editor` variants.
    pub fn all() -> Vec<Self> {
        vec![Self::Game, Self::Editor]
    }
}

impl LogCategory {
    /// Returns all available log categories.
    ///
    /// # Returns
    /// A vector containing all category variants in a standard order.
    pub fn all() -> Vec<Self> {
        vec![
            Self::Entity,
            Self::Asset,
            Self::UI,
            Self::Input,
            Self::System,
            Self::Network,
            Self::Other,
            Self::Debug,
            Self::Blank,
        ]
    }

    /// Returns the UI color associated with this category.
    ///
    /// # Returns
    /// An `RgbaColor` representing the visual identifier for this category.
    pub fn ui_color(&self) -> RgbaColor {
        match self {
            LogCategory::Entity => RgbaColor(180, 0, 255, 255),
            LogCategory::Asset => RgbaColor(255, 193, 7, 255),
            LogCategory::UI => RgbaColor::GREEN,
            LogCategory::Input => RgbaColor::BLUE,
            LogCategory::System => RgbaColor::GRAY,
            LogCategory::Network => RgbaColor::BLUE,
            LogCategory::Other => RgbaColor(255, 152, 0, 255),
            LogCategory::Blank => RgbaColor::WHITE,
            LogCategory::Debug => RgbaColor::GRAY,
        }
    }
}

impl LogLevel {
    /// Returns all available log levels.
    ///
    /// # Returns
    /// A vector containing all level variants ordered from least to most severe.
    pub fn all() -> Vec<Self> {
        vec![
            Self::OK,
            Self::Warning,
            Self::Error,
            Self::Critical,
            Self::Info,
        ]
    }

    /// Returns only the Info level.
    ///
    /// # Returns
    /// A vector containing only `LogLevel::Info`.
    pub fn info() -> Vec<Self> {
        vec![Self::Info]
    }

    /// Returns the minimal set of levels (excludes Info).
    ///
    /// Useful for production builds where informational messages are not needed.
    ///
    /// # Returns
    /// A vector containing OK, Warning, Error, and Critical levels.
    pub fn minimal() -> Vec<Self> {
        vec![Self::OK, Self::Warning, Self::Error, Self::Critical]
    }

    /// Returns levels that indicate problems (Warning and above).
    ///
    /// Useful for filtering to show only issue-related messages.
    ///
    /// # Returns
    /// A vector containing Warning, Error, and Critical levels.
    pub fn errors() -> Vec<Self> {
        vec![Self::Warning, Self::Error, Self::Critical]
    }

    /// Returns the UI color associated with this log level.
    ///
    /// # Returns
    /// An `RgbaColor` representing the visual identifier for this level.
    pub fn ui_color(&self) -> RgbaColor {
        match self {
            LogLevel::Info => RgbaColor::GRAY,
            LogLevel::OK => RgbaColor::GREEN,
            LogLevel::Warning => RgbaColor::YELLOW,
            LogLevel::Error => RgbaColor::RED,
            LogLevel::Critical => RgbaColor(255, 0, 0, 255),
        }
    }
}

// Configuration and filtering for log output
// -----------------------------------------------------------------------------------------------------------------------

/// Thread-safe set of enabled log categories.
///
/// By default, all categories are enabled after calling `setup_logging()`.
/// Use `disable_log_category()` to filter out specific categories.
pub static ENABLED_LOG_CATEGORIES: std::sync::LazyLock<Mutex<HashSet<LogCategory>>> =
    std::sync::LazyLock::new(|| Mutex::new(HashSet::new()));

/// Thread-safe set of enabled log levels.
///
/// By default, all levels are enabled after calling `setup_logging()`.
/// Use `disable_log_level()` to filter out specific severity levels.
pub static ENABLED_LOG_LEVELS: std::sync::LazyLock<Mutex<HashSet<LogLevel>>> =
    std::sync::LazyLock::new(|| Mutex::new(HashSet::new()));

/// Thread-safe set of enabled log types.
///
/// By default, all types are enabled after calling `setup_logging()`.
/// Use `disable_log_type()` to filter out specific log types.
pub static ENABLED_LOG_TYPES: std::sync::LazyLock<Mutex<HashSet<LogType>>> =
    std::sync::LazyLock::new(|| Mutex::new(HashSet::new()));

/// Disables a specific log category from being output.
///
/// This function removes the category from the enabled set, preventing
/// logs of that category from appearing in stdout.
///
/// # Arguments
///
/// * `category` - The `LogCategory` to disable
///
/// # Example
///
/// ```rust,no_run
/// use meshflow_vibe_logging::{setup_logging, disable_log_category, LogCategory};
///
/// setup_logging();
/// disable_log_category(LogCategory::Debug); // Hide debug logs
/// ```
pub fn disable_log_category(category: LogCategory) {
    let mut set = ENABLED_LOG_CATEGORIES.lock().unwrap();
    set.remove(&category);
}

/// Disables a specific log level from being output.
///
/// This function removes the level from the enabled set, preventing
/// logs of that severity from appearing in stdout.
///
/// # Arguments
///
/// * `level` - The `LogLevel` to disable
///
/// # Example
///
/// ```rust,no_run
/// use meshflow_vibe_logging::{setup_logging, disable_log_level, LogLevel};
///
/// setup_logging();
/// disable_log_level(LogLevel::Info); // Hide info logs
/// ```
pub fn disable_log_level(level: LogLevel) {
    let mut set = ENABLED_LOG_LEVELS.lock().unwrap();
    set.remove(&level);
}

/// Disables a specific log type from being output.
///
/// This function removes the type from the enabled set, preventing
/// logs of that type from appearing in stdout.
///
/// # Arguments
///
/// * `type_` - The `LogType` to disable
///
/// # Example
///
/// ```rust,no_run
/// use meshflow_vibe_logging::{setup_logging, disable_log_type, LogType};
///
/// setup_logging();
/// disable_log_type(LogType::Editor); // Hide editor logs
/// ```
pub fn disable_log_type(r#type: LogType) {
    let mut set = ENABLED_LOG_TYPES.lock().unwrap();
    set.remove(&r#type);
}

/// Initializes the logging system with default settings.
///
/// This function enables all log categories, levels, and types.
/// It should be called early in the application lifecycle.
///
/// # Example
///
/// ```rust,no_run
/// use meshflow_vibe_logging::setup_logging;
///
/// // Initialize logging before any log calls
/// setup_logging();
///
/// // Now you can use the log! macro
/// log!("Logging is ready");
/// ```
pub fn setup_logging() {
    let categories = LogCategory::all();
    let levels = LogLevel::all();
    let types = LogType::all();

    {
        let mut category_set = ENABLED_LOG_CATEGORIES.lock().unwrap();
        for category in categories.iter() {
            category_set.insert(*category);
        }
    }

    {
        let mut level_set = ENABLED_LOG_LEVELS.lock().unwrap();
        for level in levels.iter() {
            level_set.insert(*level);
        }
    }

    {
        let mut types_set = ENABLED_LOG_TYPES.lock().unwrap();
        for r#type in types.iter() {
            types_set.insert(*r#type);
        }
    }
    log!(
        LogType::Game,
        LogLevel::Info,
        LogCategory::Blank,
        "--------------------"
    );
    log!(
        LogType::Game,
        LogLevel::Info,
        LogCategory::System,
        "Logging initialized and setup, this is a new session."
    );
    log!(
        LogType::Game,
        LogLevel::Info,
        LogCategory::Blank,
        "--------------------"
    );
}
// -----------------------------------------------------------------------------------------------------------------------
