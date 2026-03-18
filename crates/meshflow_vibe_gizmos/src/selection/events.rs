//! Selection-related events.
//!
//! These events handle entity selection, deselection, and duplication requests.

use bevy::{
    ecs::event::Event,
    prelude::{Entity, Message},
};

/// Events related to entity selection in the gizmo system.
///
/// This enum represents all possible selection operations:
/// - Single entity selection/deselection
/// - Range selection/deselection (multiple entities)
/// - Deselect all
#[derive(Event)]
pub enum EntityEvents {
    /// Select a single entity.
    ///
    /// If `additive` is true, this entity is added to the current selection.
    /// If false, this entity becomes the sole selection.
    Select { target: Entity, additive: bool },

    /// Select multiple entities in a range.
    ///
    /// If `additive` is true, all entities in the range are added to the current selection.
    /// If false, these entities become the sole selection.
    SelectRange { range: Vec<Entity>, additive: bool },

    /// Deselect a single entity.
    Deselect { target: Entity },

    /// Deselect multiple entities.
    DeselectRange { range: Vec<Entity> },

    /// Deselect all entities.
    DeselectAll,
}

/// Request to duplicate a specific entity.
#[derive(Message)]
pub struct RequestDuplicateEntityEvent {
    /// The entity ID to duplicate
    pub entity: Entity,
}

/// Request to duplicate all currently selected entities.
#[derive(Message)]
pub struct RequestDuplicateAllSelectionEvent;
