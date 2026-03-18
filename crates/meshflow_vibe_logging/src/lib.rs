//! # meshflow_vibe_logging
//!
//! A comprehensive logging system for the Meshflow Vibe application.
//! Provides categorized, typed logging with support for multiple output channels
//! (stdout, file, buffer) and rich formatting options.
//!
//! ## Features
//!
//! - **Multi-channel output**: Logs are simultaneously written to stdout, file, and an in-memory buffer
//! - **Categorized logging**: Organize logs by category (Entity, Asset, UI, Input, System, Network, etc.)
//! - **Level-based filtering**: Filter logs by severity (Info, OK, Warning, Error, Critical)
//! - **Type separation**: Distinguish between Editor and Game logging contexts
//! - **Colored console output**: Visual differentiation with color-coded levels and categories
//! - **Persistent storage**: Logs are automatically written to a configuration file
//! - **In-memory buffer**: Maintains up to 7,500 recent entries for UI display
//!
//! ## Usage
//!
//! ### Basic Usage
//!
//! ```rust,no_run
//! use meshflow_vibe_logging::{log, setup_logging, LogCategory, LogLevel, LogType};
//!
//! // Initialize the logging system
//! setup_logging();
//!
//! // Log a message with default parameters (Game type, Info level, Blank category)
//! log!("This is a simple log message");
//!
//! // Log with explicit category and level
//! log!(
//!     LogType::Game,
//!     LogLevel::Info,
//!     LogCategory::System,
//!     "System initialized successfully"
//! );
//!
//! // Log with category, level, and type
//! log!(
//!     LogLevel::Warning,
//!     LogCategory::Asset,
//!     "Asset loading may take a moment"
//! );
//!
//! // Log with category only (defaults to Game type, Info level)
//! log!(cat: LogCategory::UI, "UI element created");
//! ```
//!
//! ### Advanced Usage
//!
//! ```rust,no_run
//! use meshflow_vibe_logging::{
//!     log, setup_logging, LogCategory, LogLevel, LogType,
//!     LOG_BUFFER, push_log, LogEntry
//! };
//! use std::sync::Mutex;
//!
//! setup_logging();
//!
//! // Access the log buffer (useful for UI display)
//! {
//!     let buffer = LOG_BUFFER.lock().unwrap();
//!     for entry in buffer.iter().take(10) {
//!         println!("{}: {}", entry.category, entry.message);
//!     }
//! }
//!
//! // Manually push a log entry
//! let entry = LogEntry {
//!     timestamp: "12/31/2025-23:59:59 ".to_string(),
//!     log_type: LogType::Editor,
//!     level: LogLevel::OK,
//!     category: LogCategory::Input,
//!     message: "User input processed".to_string(),
//! };
//! push_log(entry);
//!
//! // Customize logging configuration
//! meshflow_vibe_logging::disable_log_level(LogLevel::Info); // Hide info logs
//! meshflow_vibe_logging::disable_log_category(LogCategory::Debug); // Hide debug logs
//! ```
//!
//! ### Log Categories
//!
//! | Category | Description | Color |
//! |----------|-------------|-------|
//! | Entity | Entity-related operations | Purple |
//! | Asset | Asset loading and management | Yellow |
//! | UI | User interface events | Green |
//! | Input | User input events | Blue |
//! | System | System-level events | Gray |
//! | Network | Network operations | Blue |
//! | Other | Miscellaneous events | Orange |
//! | Debug | Debug information | Gray |
//! | Blank | Separator/decorative messages | White |
//!
//! ## Log Levels
//!
//! | Level | Description | Color |
//! |-------|-------------|-------|
//! | Info | Informational messages | Gray |
//! | OK | Success messages | Green |
//! | Warning | Warning conditions | Yellow |
//! | Error | Error conditions | Red |
//! | Critical | Critical failures | Red (bold) |
//!
//! ## File Storage
//!
//! Logs are automatically stored in:
//! - **Linux**: `~/.config/meshflow_vibe_logging/app.log`
//! - **macOS**: `~/Library/Application Support/meshflow_vibe_logging/app.log`
//! - **Windows**: `%APPDATA%/meshflow_vibe_logging/app.log`
//!
//! ## License
//!
//! This logging crate is part of the Meshflow Vibe application.

//! # Changes
//!
//! ## Turn 1 (2026-03-18)
//! - Added comprehensive crate-level documentation with examples
//! - Documented LogCategory enum with all variants
//! - Documented LogLevel enum with all variants  
//! - Documented LogType enum with all variants
//! - Documented RgbaColor struct with all constants
//! - Documented LogEntry struct and all fields
//! - Documented LOG_BUFFER with thread safety notes
//! - Documented push_log function with examples
//! - Documented log function with thread safety notes
//! - Documented write_to_file function with usage examples
//! - Documented log! macro with all call patterns and examples
//! - Documented setup_logging, disable_log_category, disable_log_level, disable_log_type functions
//!
//! **Verification**: `cargo doc -p meshflow_vibe_logging --no-deps` succeeded with no warnings

pub mod buffer;
pub mod config;
pub mod entry;
pub mod file;
pub mod macros;
pub mod output;

pub use buffer::{push_log, LOG_BUFFER};
pub use config::{
    disable_log_category, disable_log_level, disable_log_type, setup_logging, LogCategory,
    LogLevel, LogType, RgbaColor,
};
pub use entry::LogEntry;
pub use output::log;
