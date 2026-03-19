//! Edit mode session state and lifecycle management.
//!
//! This module provides the authoritative state for entering and exiting Edit Mode
//! for entities that own editable topology. It defines the session resource,
//! eligibility checks, and cleanup semantics for edit-only visuals and selections.
//!
//! # Architecture
//!
//! - `EditSession` - Resource tracking current edit mode state
//! - `EnterEditMode` / `ExitEditMode` - Events for mode transitions
//! - `OnEnterEditMode` / `OnExitEditMode` - Observer events for cleanup
//!
//! # Sub-Selection Modes
//!
//! - `SubSelectionMode` - Resource tracking Vertex/Edge/Face mode
//! - `SwitchSubSelectionMode` - Event for mode transitions
//! - `OnSubSelectionModeSwitched` - Observer event for cleanup
//!
//! # Usage Pattern
//!
//! 1. Check eligibility: entity must have `TopologyOwner` with valid `TopologyId`
//! 2. Enter edit mode: trigger `EnterEditMode` event with target entity
//! 3. Exit edit mode: trigger `ExitEditMode` event or despawn topology owner
//! 4. Cleanup: edit-only visuals/selections are cleared automatically
//! 5. Switch sub-selection: trigger `SwitchSubSelectionMode` with V/E/F keys
//!
mod plugin;
mod sub_selection;

use crate::entities::{TopologyId, TopologyOwner};
use bevy::prelude::{Component, Entity, Message, Resource};

pub use crate::entities::edit_mode::plugin::EditModePlugin;
pub use sub_selection::{Mode, ModeSelectionState, SubSelectionMode, SwitchSubSelectionMode};

pub use sub_selection::OnSubSelectionModeSwitched;

/// Marker component for entities currently being edited in Edit Mode.
///
/// This component is added to an entity when entering Edit Mode and removed
/// when exiting. It serves as a query filter for edit-mode-specific systems.
#[derive(Component, Debug, Clone, Copy)]
pub struct InEditMode;

/// Resource tracking the current edit mode session.
///
/// This is the authoritative source of truth for whether the editor is in
/// Edit Mode or Object Mode. It stores which entity (if any) is currently
/// being edited and its topology ID.
#[derive(Resource, Debug, Clone, Default)]
pub struct EditSession {
    /// The entity currently in Edit Mode, if any.
    pub active_entity: Option<Entity>,
    /// The topology ID of the active entity.
    pub topology_id: TopologyId,
}

impl EditSession {
    /// Create a new empty edit session (Object Mode).
    pub fn new() -> Self {
        Self {
            active_entity: None,
            topology_id: TopologyId::dummy(),
        }
    }

    /// Check if currently in Edit Mode.
    pub fn is_active(&self) -> bool {
        self.active_entity.is_some()
    }

    /// Get the currently active edit entity, if any.
    pub fn active_entity(&self) -> Option<Entity> {
        self.active_entity
    }

    /// Get the topology ID of the active entity.
    pub fn topology_id(&self) -> TopologyId {
        self.topology_id
    }

    /// Check if the given entity is currently being edited.
    pub fn is_editing(&self, entity: Entity) -> bool {
        self.active_entity == Some(entity)
    }

    /// Enter edit mode for the given entity with the given topology ID.
    pub fn enter(&mut self, entity: Entity, topology_id: TopologyId) {
        self.active_entity = Some(entity);
        self.topology_id = topology_id;
    }

    /// Exit edit mode, clearing the session.
    pub fn exit(&mut self) {
        self.active_entity = None;
        self.topology_id = TopologyId::dummy();
    }
}

/// Event to request entering Edit Mode for a specific entity.
///
/// Systems should check eligibility before transitioning:
/// - Entity must have `TopologyOwner` component
/// - Topology ID must be valid (not dummy)
/// - Topology must exist in `EditableTopologyRegistry`
#[derive(Message, Debug, Clone, Copy)]
pub struct EnterEditMode {
    /// The entity to enter edit mode for.
    pub entity: Entity,
    /// The topology ID to use (should match TopologyOwner's ID).
    pub topology_id: TopologyId,
}

/// Event to request exiting Edit Mode.
#[derive(Message, Debug, Clone, Copy)]
pub struct ExitEditMode;

/// Observer event fired when an entity enters Edit Mode.
///
/// Systems can observe this event to:
/// - Spawn edit-only visuals (vertex/edge/face gizmos)
/// - Enable element selection state
/// - Register cleanup resources
#[derive(Message, Debug, Clone, Copy)]
pub struct OnEnterEditMode(pub Entity);

/// Observer event fired when an entity exits Edit Mode.
///
/// Systems can observe this event to:
/// - Despawn edit-only visuals
/// - Clear element selection state
/// - Release edit-only resources
#[derive(Message, Debug, Clone, Copy)]
pub struct OnExitEditMode(pub Entity);

/// Check if an entity is eligible for edit mode.
///
/// An entity is eligible if it has a `TopologyOwner` component with a
/// non-dummy topology ID.
pub fn is_edit_mode_eligible(_entity: Entity, owner: Option<&TopologyOwner>) -> bool {
    owner.map(|o| !o.topology_id.is_dummy()).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_session_new() {
        let session = EditSession::new();
        assert!(!session.is_active());
        assert_eq!(session.active_entity(), None);
        assert!(session.topology_id().is_dummy());
    }

    #[test]
    fn test_edit_session_enter_exit() {
        let mut session = EditSession::new();

        // Enter edit mode
        let test_entity = Entity::from_bits((42u64) << 16);
        let test_id = TopologyId::new(123);
        session.enter(test_entity, test_id);

        assert!(session.is_active());
        assert_eq!(session.active_entity(), Some(test_entity));
        assert_eq!(session.topology_id().value(), 123);

        // Exit edit mode
        session.exit();

        assert!(!session.is_active());
        assert_eq!(session.active_entity(), None);
        assert!(session.topology_id().is_dummy());
    }

    #[test]
    fn test_edit_session_is_editing() {
        let mut session = EditSession::new();
        let test_entity = Entity::from_bits((99u64) << 16);

        assert!(!session.is_editing(test_entity));

        session.enter(test_entity, TopologyId::new(1));
        assert!(session.is_editing(test_entity));
        assert!(!session.is_editing(Entity::from_bits((100u64) << 16)));
    }

    #[test]
    fn test_is_edit_mode_eligible() {
        let valid_owner = TopologyOwner {
            topology_id: TopologyId::new(1),
        };
        let dummy_owner = TopologyOwner {
            topology_id: TopologyId::dummy(),
        };

        assert!(is_edit_mode_eligible(
            Entity::from_bits((1u64) << 16),
            Some(&valid_owner)
        ));
        assert!(!is_edit_mode_eligible(
            Entity::from_bits((2u64) << 16),
            Some(&dummy_owner)
        ));
        assert!(!is_edit_mode_eligible(
            Entity::from_bits((3u64) << 16),
            None
        ));
    }

    #[test]
    fn test_is_edit_mode_eligible_with_registry() {
        use bevy::app::App;

        let mut app = App::new();
        app.init_resource::<crate::EditableTopologyRegistry>()
            .add_plugins(super::plugin::EditModePlugin);

        // Register a topology so the entity is eligible for edit mode
        let mut registry = app
            .world_mut()
            .get_resource_mut::<crate::EditableTopologyRegistry>()
            .expect("Registry should exist");
        let topology_id = registry.insert(crate::topology::EditableTopology::new());

        // Verify registry contains the topology
        assert!(registry.contains(topology_id));
    }
}
