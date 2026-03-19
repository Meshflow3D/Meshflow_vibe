//! Sub-selection mode types and state management for Edit Mode.
//!
//! This module defines the sub-selection modes (Vertex, Edge, Face) that can be
//! active while in Edit Mode. Exactly one mode must be active at any time.
//!
//! # Architecture
//!
//! - `SubSelectionMode` - Resource struct containing the current sub-selection mode
//! - `EditSession` - Resource tracking current edit session
//! - `SwitchSubSelectionMode` - Event for mode transitions
//! - `OnSubSelectionModeSwitched` - Observer event for mode changes
//!
//! # Usage Pattern
//!
//! 1. Enter Edit Mode first (via `EnterEditMode` event)
//! 2. Switch modes: trigger `SwitchSubSelectionMode` with new mode
//! 3. Observers fire `OnSubSelectionModeSwitched` for cleanup/updates
//! 4. Leaving Edit Mode clears sub-selection state

use bevy::prelude::{Component, Entity, Message, Resource};
use serde::{Deserialize, Serialize};

/// Sub-selection mode for Edit Mode.
///
/// Exactly one mode must be active at a time while Edit Mode is active.
/// Switching modes clears incompatible selections and visuals.
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubSelectionMode {
    /// The currently active sub-selection mode
    pub mode: Mode,
}

impl SubSelectionMode {
    /// Create a new sub-selection mode with the given mode
    pub fn new(mode: Mode) -> Self {
        Self { mode }
    }

    /// Get the current mode
    pub fn get(&self) -> Mode {
        self.mode
    }

    /// Check if currently in vertex selection mode
    pub fn is_vertex(&self) -> bool {
        self.mode == Mode::Vertex
    }

    /// Check if currently in edge selection mode
    pub fn is_edge(&self) -> bool {
        self.mode == Mode::Edge
    }

    /// Check if currently in face selection mode
    pub fn is_face(&self) -> bool {
        self.mode == Mode::Face
    }

    /// Switch to a new mode, returning the old mode for potential cleanup
    pub fn switch(&mut self, new_mode: Mode) -> Mode {
        let old_mode = self.mode;
        self.mode = new_mode;
        old_mode
    }
}

/// Available sub-selection modes in Edit Mode.
///
/// - `Vertex`: Select and edit individual vertices
/// - `Edge`: Select and edit individual edges
/// - `Face`: Select and edit individual faces
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    /// Vertex selection mode
    Vertex,
    /// Edge selection mode
    Edge,
    /// Face selection mode
    Face,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Vertex
    }
}

impl Mode {
    /// Get the keyboard shortcut character for this mode
    pub fn shortcut_char(&self) -> char {
        match self {
            Mode::Vertex => 'V',
            Mode::Edge => 'E',
            Mode::Face => 'F',
        }
    }

    /// Get the display name for this mode
    pub fn display_name(&self) -> &'static str {
        match self {
            Mode::Vertex => "Vertex",
            Mode::Edge => "Edge",
            Mode::Face => "Face",
        }
    }

    /// Parse mode from a character shortcut
    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'V' => Some(Mode::Vertex),
            'E' => Some(Mode::Edge),
            'F' => Some(Mode::Face),
            _ => None,
        }
    }
}

/// Event to request switching to a new sub-selection mode.
///
/// This event should be triggered when the user wants to change the
/// active sub-selection mode (e.g., via keyboard shortcut or UI button).
#[derive(Message, Debug, Clone, Copy)]
pub struct SwitchSubSelectionMode {
    /// The new mode to switch to
    pub mode: Mode,
}

/// Observer event fired when a sub-selection mode is switched.
///
/// Systems can observe this event to:
/// - Clear selections from the old mode
/// - Spawn/hide mode-specific gizmos
/// - Update UI state
/// - Clean up stale visuals
#[derive(Message, Debug, Clone, Copy)]
pub struct OnSubSelectionModeSwitched {
    /// The entity being edited
    pub entity: Entity,
    /// The old mode being switched from
    pub old_mode: Mode,
    /// The new mode being switched to
    pub new_mode: Mode,
}

/// Component added to entities that have mode-specific selection state.
///
/// This is used to track which entities have active selections in
/// the current sub-selection mode, allowing for proper cleanup when
/// switching modes or leaving Edit Mode.
#[derive(Component, Debug)]
pub struct ModeSelectionState {
    /// The sub-selection mode this state belongs to
    pub mode: Mode,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_shortcut_chars() {
        assert_eq!(Mode::Vertex.shortcut_char(), 'V');
        assert_eq!(Mode::Edge.shortcut_char(), 'E');
        assert_eq!(Mode::Face.shortcut_char(), 'F');
    }

    #[test]
    fn test_mode_from_char() {
        assert_eq!(Mode::from_char('v'), Some(Mode::Vertex));
        assert_eq!(Mode::from_char('V'), Some(Mode::Vertex));
        assert_eq!(Mode::from_char('e'), Some(Mode::Edge));
        assert_eq!(Mode::from_char('E'), Some(Mode::Edge));
        assert_eq!(Mode::from_char('f'), Some(Mode::Face));
        assert_eq!(Mode::from_char('F'), Some(Mode::Face));
        assert_eq!(Mode::from_char('Q'), None);
    }

    #[test]
    fn test_mode_display_names() {
        assert_eq!(Mode::Vertex.display_name(), "Vertex");
        assert_eq!(Mode::Edge.display_name(), "Edge");
        assert_eq!(Mode::Face.display_name(), "Face");
    }

    #[test]
    fn test_sub_selection_mode_switch() {
        let mut state = SubSelectionMode::default();

        assert_eq!(state.get(), Mode::Vertex);

        let old = state.switch(Mode::Edge);
        assert_eq!(old, Mode::Vertex);
        assert_eq!(state.get(), Mode::Edge);

        let old = state.switch(Mode::Face);
        assert_eq!(old, Mode::Edge);
        assert_eq!(state.get(), Mode::Face);
    }

    #[test]
    fn test_sub_selection_mode_accessors() {
        let vertex = SubSelectionMode::new(Mode::Vertex);
        assert!(vertex.is_vertex());
        assert!(!vertex.is_edge());
        assert!(!vertex.is_face());

        let edge = SubSelectionMode::new(Mode::Edge);
        assert!(!edge.is_vertex());
        assert!(edge.is_edge());
        assert!(!edge.is_face());

        let face = SubSelectionMode::new(Mode::Face);
        assert!(!face.is_vertex());
        assert!(!face.is_edge());
        assert!(face.is_face());
    }

    #[test]
    fn test_mode_switching_preserves_old_mode() {
        let mut state = SubSelectionMode::default();

        // Switch from Vertex to Edge
        let old = state.switch(Mode::Edge);
        assert_eq!(old, Mode::Vertex);
        assert_eq!(state.get(), Mode::Edge);

        // Switch from Edge to Face
        let old = state.switch(Mode::Face);
        assert_eq!(old, Mode::Edge);
        assert_eq!(state.get(), Mode::Face);

        // Switch back to Vertex
        let old = state.switch(Mode::Vertex);
        assert_eq!(old, Mode::Face);
        assert_eq!(state.get(), Mode::Vertex);
    }

    #[test]
    fn test_default_mode_is_vertex() {
        let default = SubSelectionMode::default();
        assert_eq!(default.get(), Mode::Vertex);
        assert!(default.is_vertex());
    }

    #[test]
    fn test_mode_from_char_case_insensitive() {
        assert_eq!(Mode::from_char('v'), Some(Mode::Vertex));
        assert_eq!(Mode::from_char('V'), Some(Mode::Vertex));
        assert_eq!(Mode::from_char('e'), Some(Mode::Edge));
        assert_eq!(Mode::from_char('E'), Some(Mode::Edge));
        assert_eq!(Mode::from_char('f'), Some(Mode::Face));
        assert_eq!(Mode::from_char('F'), Some(Mode::Face));
    }

    #[test]
    fn test_mode_from_char_invalid() {
        assert_eq!(Mode::from_char('Q'), None);
        assert_eq!(Mode::from_char('1'), None);
        assert_eq!(Mode::from_char(' '), None);
        assert_eq!(Mode::from_char('a'), None);
    }
}
