/// Test utilities for meshflow_vibe_core.
///
/// This module provides helpers for writing deterministic, headless tests
/// that don't require a windowed Bevy session.
pub mod basic;
pub mod fixtures;
pub mod helpers;

// Re-export commonly used items from fixtures
pub use fixtures::*;
// Re-export from helpers
pub use helpers::{app_should_exit, run_iterations};
