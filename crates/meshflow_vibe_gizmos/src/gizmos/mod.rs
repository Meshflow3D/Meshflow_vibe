//! Gizmo-related types, components, and events.
//!
//! This module defines the core types for the gizmo system including:
//! - `GizmoType`: The type of gizmo (Transform, Rotate, Pointer, None)
//! - `GizmoMode`: The coordinate space (Local/Global)
//! - `GizmoConfig`: Configuration for individual gizmos
//! - Entity components for managing gizmo hierarchies

use bevy::{
    camera::visibility::RenderLayers,
    ecs::{component::Component, entity::Entity, lifecycle::HookContext, resource::Resource},
    prelude::{Deref, DerefMut},
};

pub mod distance_scaling;
pub mod events;
pub mod manager;
pub mod plugin;
pub mod rotate;
pub mod transform;
pub mod vertex;

/// The type of gizmo to display.
///
/// This enum determines what kind of transformation gizmo is shown:
/// - `Transform`: Shows translation and scaling handles (arrows and boxes)
/// - `Rotate`: Shows rotation handles (rings)
/// - `Pointer`: No visible gizmo, only interaction support
/// - `None`: Completely disabled
#[derive(Clone, Default, Debug, Copy, PartialEq)]
pub enum GizmoType {
    /// Translation/scale gizmo with arrows and bounding box
    Transform,
    /// Rotation gizmo with circular rings
    Rotate,
    /// Pointer-only mode, no visual gizmo
    #[default]
    Pointer,
    /// Completely disabled
    None,
}

/// Resource for setting the new active gizmo type.
///
/// This is a wrapper around `GizmoType` that allows systems to
/// request a change in the active gizmo type.
#[derive(Resource, Deref, DerefMut, Clone, Copy)]
pub struct NewGizmoType(pub GizmoType);

/// The coordinate space mode for gizmo operations.
///
/// - `Local`: Operations occur in the entity's local coordinate space
/// - `Global`: Operations occur in world coordinate space
#[derive(Clone, Default, Debug, Copy, PartialEq)]
pub enum GizmoMode {
    /// Object/local space - axes align with entity's local orientation
    Local,
    /// World/global space - axes align with world orientation
    #[default]
    Global,
}

/// Configuration for creating new gizmos.
///
/// This resource contains the default values used when spawning gizmos:
/// - `speed_scale`: Multiplier for transformation speed
/// - `distance_scale`: Multiplier for gizmo size based on camera distance
/// - `mode`: Default coordinate space (Local/Global)
#[derive(Resource)]
pub struct NewGizmoConfig {
    /// Speed multiplier for gizmo interactions
    pub speed_scale: f32,
    /// Distance scaling factor for gizmo size
    pub distance_scale: f32,
    /// Default coordinate space mode
    pub mode: GizmoMode,
}

impl NewGizmoConfig {
    /// Create a rotation gizmo configuration from this config.
    pub fn rotation(&self) -> GizmoConfig {
        GizmoConfig::Rotate {
            speed_scale: self.speed_scale,
            distance_scale: self.distance_scale,
            mode: self.mode,
        }
    }
    /// Create a transform gizmo configuration from this config.
    pub fn transform(&self) -> GizmoConfig {
        GizmoConfig::Transform {
            distance_scale: self.distance_scale,
            mode: self.mode,
        }
    }
}

/// Configuration for an individual gizmo instance.
///
/// This enum specifies the exact configuration for a gizmo,
/// including its type, mode, and scaling parameters.
#[derive(Component, Clone, Copy, Debug)]
pub enum GizmoConfig {
    /// Pointer-only mode
    Pointer,
    /// Disabled
    None,
    /// Rotation gizmo with speed, distance scaling, and mode
    Rotate {
        /// Speed multiplier for rotation
        speed_scale: f32,
        /// Distance scaling factor
        distance_scale: f32,
        /// Coordinate space mode
        mode: GizmoMode,
    },
    /// Transform gizmo with distance scaling and mode
    Transform {
        /// Distance scaling factor
        distance_scale: f32,
        /// Coordinate space mode
        mode: GizmoMode,
    },
}

impl GizmoConfig {
    /// Get the gizmo type from this configuration.
    pub fn gizmo_type(&self) -> GizmoType {
        match self {
            GizmoConfig::Pointer => GizmoType::Pointer,
            GizmoConfig::None => GizmoType::None,
            GizmoConfig::Rotate { .. } => GizmoType::Rotate,
            GizmoConfig::Transform { .. } => GizmoType::Transform,
        }
    }

    /// Get the coordinate space mode from this configuration.
    pub fn mode(&self) -> GizmoMode {
        match self {
            GizmoConfig::Pointer => GizmoMode::Global,
            GizmoConfig::None => GizmoMode::Global,
            GizmoConfig::Rotate { mode, .. } => *mode,
            GizmoConfig::Transform { mode, .. } => *mode,
        }
    }

    /// Change the gizmo type while preserving scaling parameters.
    pub fn set_type(&mut self, new_type: GizmoType, default_config: &NewGizmoConfig) {
        *self = match new_type {
            GizmoType::Pointer => GizmoConfig::Pointer,
            GizmoType::None => GizmoConfig::None,
            GizmoType::Rotate => GizmoConfig::Rotate {
                speed_scale: default_config.speed_scale,
                distance_scale: default_config.distance_scale,
                mode: default_config.mode,
            },
            GizmoType::Transform => GizmoConfig::Transform {
                distance_scale: default_config.distance_scale,
                mode: default_config.mode,
            },
        }
    }

    /// Change the coordinate space mode.
    pub fn set_mode(&mut self, new_mode: GizmoMode) {
        match self {
            GizmoConfig::Pointer => {}
            GizmoConfig::None => {}
            GizmoConfig::Rotate { ref mut mode, .. } => {
                *mode = new_mode;
            }
            GizmoConfig::Transform { ref mut mode, .. } => {
                *mode = new_mode;
            }
        }
    }
}

/// Resource tracking the last selected gizmo type.
#[derive(Resource)]
pub struct LastSelectedGizmo {
    /// The most recently selected gizmo type
    pub value: GizmoType,
}

/// Component marking the root entity of a gizmo hierarchy.
///
/// This is a relationship component that stores the parent entity ID
/// for gizmo child entities (arrows, rings, handles, etc.). It works
/// with `GizmoChildren` to maintain the gizmo hierarchy.
#[derive(Component, Clone, Copy, Debug)]
#[relationship(relationship_target = GizmoChildren)]
pub struct GizmoRoot(pub Entity);

/// Marker component for gizmo mesh entities.
///
/// This component is added to mesh entities that form part of a gizmo.
#[derive(Component)]
pub struct GizmoMesh;

/// Component marking entities that are children of a gizmo root.
///
/// This is a relationship target component that holds a list of
/// child entity IDs that make up the gizmo (handles, arrows, rings, etc.).
#[derive(Component)]
#[relationship_target(relationship = GizmoRoot)]
pub struct GizmoChildren(Vec<Entity>);

/// Resource containing snap angle/value thresholds.
#[derive(Resource)]
pub struct GizmoSnap {
    /// Snap angle in degrees for rotation gizmo
    pub rotate_value: f32,
    /// Snap value for transform gizmo
    pub transform_value: f32,
}

/// Component marking entities that have an associated gizmo.
///
/// This component stores the entity ID of the target object that
/// the gizmo manipulates. It's automatically added with `EditorIgnore`
/// and `RenderLayers` to prevent the gizmo itself from being
/// picked or rendered in the main view.
#[derive(Component, Deref, Clone, Copy)]
#[relationship(relationship_target = Gizmos)]
#[component(on_add = Self::on_add)]
#[require(meshflow_vibe_core::EditorIgnore, RenderLayers = RenderLayers::layer(14))]
pub struct GizmoOf(pub Entity);

impl GizmoOf {
    /// Called when this component is added to an entity.
    ///
    /// Automatically adds `EditorIgnore` flags to prevent the gizmo
    /// from being picked or affected by editor operations.
    fn on_add(mut world: bevy::ecs::world::DeferredWorld, ctx: HookContext) {
        let mut ignore = world
            .get_mut::<EditorIgnore>(ctx.entity)
            .expect("EditorIgnore is required component");
        ignore.insert(EditorIgnore::GIZMO | EditorIgnore::PICKING);
    }

    /// Get the target entity ID that this gizmo manipulates.
    pub fn get(&self) -> Entity {
        self.0
    }
}

/// Component marking the root of a gizmo hierarchy.
///
/// This is a relationship target component that holds a list of
/// all `GizmoOf` entities that are part of this gizmo group.
#[derive(Component)]
#[relationship_target(relationship = GizmoOf)]
pub struct Gizmos(Vec<Entity>);

impl Gizmos {
    /// Get a slice of all entity IDs in this gizmo group.
    pub fn entities(&self) -> &[Entity] {
        &self.0
    }
}

/// Scale gizmos based on camera distance to maintain visibility.
pub use distance_scaling::scale_gizmo_by_camera_distance_system;

/// Events related to gizmo interactions and lifecycle.
pub use events::{
    DespawnGizmoEvent, RotateDraggingEvent, RotateInitDragEvent, RotateResetDragEvent,
    SpawnGizmoEvent, TransformDraggingEvent, TransformInitDragEvent, TransformResetDragEvent,
};

/// Gizmo manager functions for accessing events and state.
pub use manager::{gizmo_changed_watcher, gizmo_events};

use meshflow_vibe_core::EditorIgnore;

/// The main gizmo plugin that sets up the gizmo system.
pub use plugin::GizmoPlugin;

/// Rotate gizmo related exports.
pub use rotate::{
    despawn_rotate_gizmo, handle_init_rotate_drag, handle_rotate_dragging, handle_rotate_input,
    handle_rotate_reset, register_embedded_rotate_gizmo_mesh, spawn_rotate_gizmo,
    update_gizmo_rotation_for_mode as update_rotate_gizmo_rotation_for_mode, RotateGizmo,
    RotateGizmoParent,
};

/// Transform gizmo related exports.
pub use transform::{
    despawn_transform_gizmo, spawn_transform_gizmo,
    update_gizmo_rotation_for_mode as update_transform_gizmo_rotation_for_mode,
    PreviousTransformGizmo, TransformGizmo, TransformGizmoParent,
};
