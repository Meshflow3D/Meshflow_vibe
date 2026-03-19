//! # Meshflow Vibe Gizmos
//!
//! This crate provides a 3D transformation gizmo system for Bevy applications.
//! Gizmos are visual tools that allow users to translate, rotate, and scale entities
//! in 3D space through interactive handles.
//!
//! ## Overview
//!
//! The system consists of several key components:
//!
//! - **Gizmo Types**: Transform (translate/scale), Rotate, Pointer, and None
//! - **Gizmo Modes**: Local (object space) and Global (world space)
//! - **Selection System**: Manages which entities have gizmos active
//! - **Event System**: Handles gizmo spawning, despawning, and drag interactions
//!
//! ## Usage
//!
//! Add the `MeshflowVibeGizmos` plugin to your Bevy app:
//!
//! ```rust,no_run
//! # #[allow(clippy::needless_doctest_main)]
//! use bevy::prelude::*;
//! use meshflow_vibe_gizmos::MeshflowVibeGizmos;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(MeshflowVibeGizmos)
//!         .run();
//! }
//! ```
//!
//! ## Architecture
//!
//! The gizmo system uses Bevy's entity-component-system (ECS) architecture:
//!
//! - `GizmoOf`: Marks entities that have associated gizmos
//! - `Gizmos`: Groups all gizmo child entities for an object
//! - `GizmoRoot`: Parent entity for gizmo hierarchy
//! - `GizmoChildren`: Child handles (arrows, rings, etc.)
//!
//! ## Camera Setup
//!
//! This crate provides a dedicated gizmo camera (`GizmoCamera`) for rendering
//! gizmos. When using without the editor sister plugin, you must manually
//! sync the gizmo camera with your main camera.

use crate::camera::{add_gizmo_camera, watch_for_main_camera_addition, MainCameraAdded};
use bevy::app::{Plugin, Update};
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::picking::mesh_picking::MeshPickingPlugin;
use bevy::prelude::{App, Res, Resource};

// Modules
pub mod camera;
pub mod gizmos;
mod input;
pub mod selection;
mod ui;

// Re-export
pub use camera::GizmoCamera;
pub use gizmos::{
    despawn_rotate_gizmo, GizmoChildren, GizmoMesh, GizmoRoot, GizmoSnap, GizmoType,
    NewGizmoConfig, RotateGizmo, TransformGizmo,
};
pub use input::{watch_gizmo_change, DragState, GizmoAxis};
pub use selection::{
    ActiveSelection, EntityEvents, RequestDuplicateAllSelectionEvent, RequestDuplicateEntityEvent,
    Selected,
};

// Internal plugins
use gizmos::vertex::VertexVisualizationPlugin;
use gizmos::GizmoPlugin;
use input::InputPlugin;
use selection::SelectionPlugin;
use ui::UIPlugin;

/// Resource to control gizmo visibility
///
/// When the editor is toggled off, this will be set to false.
/// It's a resource instead of an argument to the plugin because it
/// may need to be updated dynamically from external systems.
#[derive(Resource, Clone)]
pub struct GizmoVisibilityState {
    /// Whether gizmos are currently visible and active
    pub active: bool,
}
impl Default for GizmoVisibilityState {
    fn default() -> Self {
        Self { active: true }
    }
}

/// Main plugin for the Meshflow Vibe Gizmos system.
///
/// This plugin sets up the complete gizmo system including:
/// - Mesh picking for gizmo interaction
/// - Gizmo rendering and management
/// - Vertex visualization for accurate picking
/// - Selection system
/// - UI components
/// - Input handling
///
/// ## Important Notes
///
/// This plugin does NOT automatically sync the gizmo camera to the main camera.
/// If you are using this without the editor sister plugin, you need to handle
/// the camera syncing manually by listening for `MainCameraAdded` events.
pub struct MeshflowVibeGizmos;
impl Plugin for MeshflowVibeGizmos {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins(MeshPickingPlugin) // Raycasting plugin
            //
            // internal
            .add_plugins(GizmoPlugin)
            .add_plugins(VertexVisualizationPlugin) // Vertex picking must be BEFORE SelectionPlugin for priority
            .add_plugins(SelectionPlugin)
            .add_plugins(UIPlugin)
            .add_plugins(InputPlugin) // Optional
            .add_message::<MainCameraAdded>()
            //
            .add_systems(
                Update,
                watch_for_main_camera_addition.run_if(is_gizmos_active),
            )
            .add_systems(Update, add_gizmo_camera.run_if(is_gizmos_active));
    }
}

fn is_gizmos_active(gizmos_state: Res<GizmoVisibilityState>) -> bool {
    gizmos_state.active
}
