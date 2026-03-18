//! Events related to gizmo interactions and lifecycle.
//!
//! These message events are used to communicate gizmo state changes
//! between systems without direct coupling.

use super::GizmoType;
use bevy::prelude::{Entity, Message};

/// Event sent when rotation gizmo drag interaction is initialized.
#[derive(Message)]
pub struct RotateInitDragEvent;

/// Event sent while rotation gizmo is being dragged.
#[derive(Message)]
pub struct RotateDraggingEvent;

/// Event sent when rotation gizmo drag interaction is reset.
#[derive(Message)]
pub struct RotateResetDragEvent;

/// Event sent when transform gizmo drag interaction is initialized.
#[derive(Message)]
pub struct TransformInitDragEvent;

/// Event sent while transform gizmo is being dragged.
#[derive(Message)]
pub struct TransformDraggingEvent;

/// Event sent when transform gizmo drag interaction is reset.
#[derive(Message)]
pub struct TransformResetDragEvent;

/// Event sent when a gizmo should be spawned for an entity.
///
/// Contains the entity ID that should have a gizmo created for it.
#[derive(Message)]
pub struct SpawnGizmoEvent(pub Entity);

/// Event sent when a gizmo should be despawned.
///
/// Contains the type of gizmo to despawn, allowing for selective
/// removal when multiple gizmo types exist for an entity.
#[derive(Message)]
pub struct DespawnGizmoEvent(pub GizmoType);
