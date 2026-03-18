//! Transformation gizmo drag interaction and rendering.
//!
//! This module handles:
//! - Dragging transform gizmo axes and planes
//! - Calculating drag offsets
//! - Rendering temporary axis lines for gizmo interaction
//! - Applying transformations to selected entities
//! - Cleanup of temporary gizmo elements
//!
//! # Main Functions
//!
//! - `drag_transform_gizmo`: Process drag events for transform gizmos
//! - `calculate_drag_offset`: Compute initial drag offset on drag start
//! - `drag_end_cleanup`: Remove drag offset components after drag ends
//! - `apply_transformations`: Apply delta transformations to objects
//! - `dragstart_transform_gizmo`: Handle shift+drag duplication
//! - `draw_axis_lines`: Render temporary axis lines on press
//! - `cleanup_axis_line`: Remove temporary axis lines on release

use super::TransformGizmo;
use crate::{
    gizmos::{GizmoConfig, GizmoMode, GizmoOf, GizmoRoot, GizmoSnap},
    input::GizmoAxis,
    selection::{ActiveSelection, RequestDuplicateAllSelectionEvent, Selected},
    GizmoCamera,
};

/// Marker component for temporary axis line gizmos created during drag interaction.
#[derive(Component)]
pub struct AxisLine;
use bevy::{
    asset::Assets,
    ecs::{
        component::Component,
        entity::ContainsEntity,
        hierarchy::{ChildOf, Children},
        message::MessageWriter,
        observer::On,
        system::Commands,
    },
    gizmos::{retained::Gizmo, GizmoAsset},
    picking::events::{Drag, DragEnd, DragStart, Pointer, Press},
    prelude::{
        Entity, GlobalTransform, Quat, Query, Res, ResMut, Resource, Transform, Vec3, With, Without,
    },
};
use meshflow_vibe_core::UserInput;
use meshflow_vibe_logging::{
    config::{LogCategory, LogLevel, LogType},
    log,
};

/// State resource for tracking transform gizmo duplication operations.
///
/// This resource is used to prevent accidental duplication during drag operations
/// by tracking when a duplication has just occurred.
#[derive(Resource, Default)]
pub struct TransformDuplicationState {
    /// Whether a duplication has just occurred and should be ignored.
    pub just_duplicated: bool,
}

/// Handle drag events for transform gizmos.
///
/// This function processes drag events on transform gizmo entities to:
/// - Calculate the drag offset and direction
/// - Transform selected entities along the appropriate axis or plane
/// - Apply snapping based on configured snap values
/// - Handle camera movement when Ctrl is pressed
///
/// # Arguments
/// * `event` - The drag event containing entity and input information
/// * `command` - Bevy Commands for entity manipulation
/// * `targets` - Query for GizmoOf components to find target entities
/// * `camera_query` - Query for gizmo camera entities and transforms
/// * `objects` - Query for mutable Transform components on objects
/// * `global_transforms` - Query for GlobalTransform components
/// * `parents` - Query for ChildOf components to find hierarchy
/// * `active_selection` - Query for actively selected entity
/// * `other_selected` - Query for other selected entities
/// * `gizmo_snap` - Resource containing snap configuration
/// * `gizmo_data` - Query for gizmo axis, type, offset, and root
/// * `gizmo_config_query` - Query for gizmo configuration
/// * `user_input` - Resource containing current user input state
/// * `duplication_state` - Mutable reference to duplication state resource
pub fn drag_transform_gizmo(
    event: On<Pointer<Drag>>,
    mut command: Commands,
    targets: Query<&GizmoOf>,
    camera_query: Query<(Entity, &GlobalTransform, &bevy::camera::Camera), With<GizmoCamera>>,
    mut objects: Query<&mut Transform>,
    global_transforms: Query<&GlobalTransform>,
    parents: Query<&ChildOf>,
    active_selection: Query<Entity, With<ActiveSelection>>,
    other_selected: Query<Entity, (With<Selected>, Without<ActiveSelection>)>,
    gizmo_snap: Res<GizmoSnap>,
    gizmo_data: Query<(&GizmoAxis, &TransformGizmo, &InitialDragOffset, &GizmoRoot)>,
    gizmo_config_query: Query<&GizmoConfig>,
    user_input: Res<UserInput>,
    mut duplication_state: ResMut<TransformDuplicationState>,
) {
    if event.button != bevy::picking::pointer::PointerButton::Primary {
        return;
    }

    if duplication_state.just_duplicated {
        duplication_state.just_duplicated = false;
        return;
    }
    let Ok((axis, typ, drag_offset, gizmo_root)) = gizmo_data.get(event.entity) else {
        log!(
            LogType::Editor,
            LogLevel::Warning,
            LogCategory::Input,
            "Gizmo Axis data not found for Gizmo entity {:?}",
            event.entity
        );
        return;
    };

    let Ok(gizmo_config) = gizmo_config_query.get(gizmo_root.0) else {
        log!(
            LogType::Editor,
            LogLevel::Warning,
            LogCategory::Input,
            "Gizmo config not found for parent gizmo entity {:?}",
            gizmo_root.0
        );
        return;
    };

    let Ok((c_entity, camera_transform, camera)) = camera_query.single() else {
        log! {
            LogType::Editor,
            LogLevel::Error,
            LogCategory::Input,
            "Gizmo camera not found",
        };
        return;
    };

    let Ok(GizmoOf(target)) = targets.get(event.entity) else {
        log! {
            LogType::Editor,
            LogLevel::Error,
            LogCategory::Input,
            "Gizmo target not found for entity {:?}",
            event.entity
        };
        return;
    };
    let Ok(click_ray) = camera.viewport_to_world(camera_transform, event.pointer_location.position)
    else {
        log! {
            LogType::Editor,
            LogLevel::Error,
            LogCategory::Input,
            "Failed to convert viewport to world coordinates for pointer location: {:?}",
            event.pointer_location.position
        };
        return;
    };

    let mut all_selected_entities = Vec::new();
    all_selected_entities.extend(active_selection.iter());
    all_selected_entities.extend(other_selected.iter());

    // Filter out entities that are children of other selected entities
    let mut root_entities = Vec::new();
    for &entity in &all_selected_entities {
        let mut is_child_of_selected = false;
        if let Ok(parent) = parents.get(entity) {
            if all_selected_entities.contains(&parent.parent()) {
                is_child_of_selected = true;
            }
        }
        if !is_child_of_selected {
            root_entities.push(entity);
        }
    }

    if root_entities.is_empty() {
        log! {
            LogType::Editor,
            LogLevel::Warning,
            LogCategory::Input,
            "No root entities to transform"
        };
        return;
    }

    let mut current_world_pos = {
        let Ok(target_transform) = objects.get(*target) else {
            log! {
                LogType::Editor,
                LogLevel::Error,
                LogCategory::Input,
                "Gizmo target transform not found for entity {:?}",
                target
            };
            return;
        };

        if let Ok(global_transform) = global_transforms.get(*target) {
            global_transform.translation()
        } else {
            target_transform.translation
        }
    };

    let target_rotation = if let Ok(global_transform) = global_transforms.get(*target) {
        global_transform.to_scale_rotation_translation().1
    } else {
        if let Ok(transform) = objects.get(*target) {
            transform.rotation
        } else {
            Quat::IDENTITY
        }
    };

    let (active_axis, normal) = match typ {
        TransformGizmo::Axis => {
            let axis_vec = match gizmo_config.mode() {
                GizmoMode::Local => target_rotation * axis.to_vec3(),
                GizmoMode::Global => axis.to_vec3(),
            };
            (axis_vec, camera_transform.forward().as_vec3())
        }
        TransformGizmo::Plane => {
            let plane_normal = match gizmo_config.mode() {
                GizmoMode::Local => target_rotation * axis.to_vec3(),
                GizmoMode::Global => axis.to_vec3(),
            };
            (plane_normal, plane_normal)
        }
    };

    current_world_pos -= drag_offset.offset();

    let Some(click_distance) = click_ray.intersect_plane(
        current_world_pos,
        bevy::math::primitives::InfinitePlane3d::new(normal),
    ) else {
        return;
    };

    let hit = click_ray.get_point(click_distance);
    let raw_delta = hit - current_world_pos;

    let world_delta = match typ {
        TransformGizmo::Axis => {
            let axis_normalized = active_axis.normalize_or_zero();
            let projection = raw_delta.dot(axis_normalized);

            let snapped_distance = if gizmo_snap.transform_value == 0.0 {
                projection
            } else {
                (projection / gizmo_snap.transform_value).round() * gizmo_snap.transform_value
            };

            axis_normalized * snapped_distance
        }
        TransformGizmo::Plane => {
            let plane_normal_normalized = normal.normalize_or_zero();
            let normal_component = raw_delta.dot(plane_normal_normalized);
            let projected = raw_delta - (plane_normal_normalized * normal_component);
            snap_gizmo(projected, gizmo_snap.transform_value)
        }
    };

    // Apply the delta to all root selected entities
    let mut world_delta = world_delta;
    if world_delta.length() > 0.0 {
        for &entity in &root_entities {
            if let Ok(parent) = parents.get(entity) {
                if let Ok(parent_global) = global_transforms.get(parent.parent()) {
                    let parent_rotation_inv =
                        parent_global.to_scale_rotation_translation().1.inverse();
                    let parent_local_delta = parent_rotation_inv * world_delta;
                    world_delta = parent_local_delta;
                }
            }
            command.entity(entity).insert(TransitionDelta(world_delta));
        }
    }

    if user_input.ctrl_left.any {
        if let Ok(mut camera_transform) = objects.get_mut(c_entity) {
            camera_transform.translation += world_delta;
        }
    }
}

/// Calculate and store the initial drag offset when a drag operation starts.
///
/// This function computes the offset between the gizmo's origin and the cursor
/// position at the start of a drag, storing it as a component on the gizmo entity.
/// If the cursor position cannot be resolved, it falls back to using the center
/// of the target object.
///
/// # Arguments
/// * `event` - The drag start event
/// * `command` - Bevy Commands for adding components
/// * `object` - Query for GlobalTransform of entities with children
/// * `gizmo_data` - Query for gizmo entity and GizmoOf components
pub fn calculate_drag_offset(
    event: On<Pointer<DragStart>>,
    mut command: Commands,
    object: Query<&GlobalTransform, With<Children>>,
    gizmo_data: Query<(Entity, &GizmoOf), With<GizmoAxis>>,
) {
    if event.button != bevy::picking::pointer::PointerButton::Primary {
        return;
    }
    let Ok((entity, parent)) = gizmo_data.get(event.entity) else {
        log!(
            LogType::Editor,
            LogLevel::Warning,
            LogCategory::Input,
            "Gizmo Axis data not found for Gizmo entity {:?}",
            event.entity
        );
        return;
    };
    let Ok(object_transform) = object.get(parent.entity()) else {
        log! {
            LogType::Editor,
            LogLevel::Error,
            LogCategory::Input,
            "Gizmo target not found for entity {:?}",
            event.entity
        };
        return;
    };

    // Fallback to the center of the target object if can't resolve position.
    let cursor_postion = event.hit.position.unwrap_or(object_transform.translation());
    command.entity(entity).insert(InitialDragOffset(
        object_transform.translation() - cursor_postion,
    ));
}

/// Clean up initial drag offset components after a drag operation ends.
///
/// This function removes `InitialDragOffset` components from all gizmo entities
/// when the drag operation ends, ensuring clean state for the next interaction.
///
/// # Arguments
/// * `event` - The drag end event
/// * `command` - Bevy Commands for removing components
/// * `gizmo_data` - Query for entities with InitialDragOffset components
pub fn drag_end_cleanup(
    event: On<Pointer<DragEnd>>,
    mut command: Commands,
    gizmo_data: Query<Entity, With<InitialDragOffset>>,
) {
    if event.button != bevy::picking::pointer::PointerButton::Primary {
        return;
    }
    for gizmo_entity in gizmo_data {
        command.entity(gizmo_entity).remove::<InitialDragOffset>();
    }
}

/// Apply transformation deltas to objects and clean up delta components.
///
/// This function iterates over all objects with TransitionDelta components,
/// applies the delta translation to their Transform, and removes the delta
/// component to prevent double application.
///
/// # Arguments
/// * `command` - Bevy Commands for removing components
/// * `objects` - Query for entities with mutable Transform and TransitionDelta
pub fn apply_transformations(
    mut command: Commands,
    objects: Query<(Entity, &mut Transform, &TransitionDelta)>,
) {
    for (entity, mut transform, transition_delta) in objects {
        transform.translation += transition_delta.0;
        command.entity(entity).remove::<TransitionDelta>();
    }
}

/// Handle drag start for duplicating selected entities.
///
/// This function creates a duplicate of all selected entities when:
/// - The mouse middle button is pressed
/// - The Shift key is held down
///
/// This provides a quick way to duplicate objects while manipulating gizmos.
///
/// # Arguments
/// * `event` - The drag start event
/// * `targets` - Query for GizmoOf components
/// * `gizmo_data` - Query for gizmo axis and type
/// * `user_input` - Resource containing user input state
/// * `dispatch` - Message writer for duplication events
/// * `duplication_state` - Mutable reference to duplication state
pub fn dragstart_transform_gizmo(
    event: On<Pointer<DragStart>>,
    targets: Query<&GizmoOf>,
    gizmo_data: Query<(&GizmoAxis, &TransformGizmo)>,
    user_input: Res<UserInput>,
    mut dispatch: MessageWriter<RequestDuplicateAllSelectionEvent>,
    mut duplication_state: ResMut<TransformDuplicationState>,
) {
    if user_input.mouse_middle.any || !user_input.shift_left.pressed {
        return;
    }
    let Ok(_) = gizmo_data.get(event.entity) else {
        return;
    };
    let Ok(GizmoOf(_target)) = targets.get(event.entity) else {
        return;
    };
    log!("Attempting Drag Duplicate");
    dispatch.write(RequestDuplicateAllSelectionEvent);
    duplication_state.just_duplicated = true;
}

/// Snap a vector value to the nearest increment.
///
/// # Arguments
/// * `value` - The vector value to snap
/// * `inc` - The increment to snap to (0.0 returns value unchanged)
///
/// # Returns
/// The snapped vector value
fn snap_gizmo(value: Vec3, inc: f32) -> Vec3 {
    if inc == 0.0 {
        value
    } else {
        (value / inc).round() * inc
    }
}

/// Draw temporary axis lines when pressing on transform gizmo elements.
///
/// This function creates visual axis lines at the press location to help
/// users understand which axis or plane they are interacting with. The lines
/// are spawned as temporary gizmos and will be cleaned up when the mouse
/// button is released.
///
/// # Arguments
/// * `event` - The press event
/// * `gizmo_data` - Query for gizmo axis, target, type, and root
/// * `gizmo_config_query` - Query for gizmo configuration
/// * `bevy_gizmo` - Mutable resource for gizmo assets
/// * `commands` - Bevy Commands for spawning entities
/// * `origin` - Query for origin transform
pub fn draw_axis_lines(
    event: On<Pointer<Press>>,
    gizmo_data: Query<(&GizmoAxis, &GizmoOf, &TransformGizmo, &GizmoRoot), With<TransformGizmo>>,
    gizmo_config_query: Query<&GizmoConfig>,
    mut bevy_gizmo: ResMut<Assets<GizmoAsset>>,
    mut commands: Commands,
    origin: Query<&GlobalTransform>,
) {
    if event.button != bevy::picking::pointer::PointerButton::Primary {
        return;
    }

    let Ok((axis, root, transform, gizmo_root)) = gizmo_data.get(event.entity) else {
        return;
    };
    if let GizmoAxis::All = axis {
        return;
    }

    let Ok(gizmo_config) = gizmo_config_query.get(gizmo_root.0) else {
        log! {
            LogType::Editor,
            LogLevel::Warning,
            LogCategory::Input,
            "Gizmo config not found for parent gizmo entity {:?}",
            gizmo_root.0
        };
        return;
    };

    let Ok(origin) = origin.get(root.get()) else {
        log! {
            LogType::Editor,
            LogLevel::Error,
            LogCategory::Input,
            "Gizmo origin transform not found for entity {:?}",
            root.0
        };
        return;
    };

    let entity_rotation = origin.to_scale_rotation_translation().1;

    let mut asset = GizmoAsset::new();
    match transform {
        TransformGizmo::Axis => {
            render_line(
                &mut asset,
                &axis,
                origin,
                entity_rotation,
                gizmo_config.mode(),
            );
        }
        TransformGizmo::Plane => {
            let (a, b) = axis.plane();
            render_line(&mut asset, &a, origin, entity_rotation, gizmo_config.mode());
            render_line(&mut asset, &b, origin, entity_rotation, gizmo_config.mode());
        }
    }

    commands.spawn((
        axis.clone(),
        GizmoOf(root.0),
        Gizmo {
            handle: bevy_gizmo.add(asset),
            ..Default::default()
        },
        AxisLine,
    ));
}

/// Render a series of line segments along a gizmo axis.
///
/// This function draws multiple line segments spaced by `step` along the
/// axis direction, from `-max_distance` to `+max_distance` from the origin.
/// The lines are rendered in the appropriate color for the axis.
///
/// # Arguments
/// * `asset` - Mutable reference to the gizmo asset to add lines to
/// * `axis` - The gizmo axis to render along
/// * `origin` - The origin transform for the gizmo
/// * `entity_rotation` - The rotation of the entity in local/global space
/// * `mode` - The gizmo mode (Local or Global)
fn render_line(
    asset: &mut GizmoAsset,
    axis: &GizmoAxis,
    origin: &GlobalTransform,
    entity_rotation: Quat,
    mode: GizmoMode,
) {
    let step = 10.0;
    let max_distance = 1000.0;
    let mut current = -max_distance;

    let axis_direction = match mode {
        GizmoMode::Local => entity_rotation * axis.to_vec3(),
        GizmoMode::Global => axis.to_vec3(),
    };

    while current < max_distance {
        asset.line(
            origin.translation() + axis_direction * current,
            origin.translation() + axis_direction * (current + step),
            axis.color(),
        );
        current += step;
    }
}

/// Clean up temporary axis line gizmos when the mouse button is released.
///
/// This function despawns all entities with the `AxisLine` marker component
/// when the left mouse button is released, removing the temporary visual
/// axis lines drawn during drag interactions.
///
/// # Arguments
/// * `commands` - Bevy Commands for despawning entities
/// * `query` - Query for entities with AxisLine component
/// * `user_input` - Resource containing user input state
pub fn cleanup_axis_line(
    mut commands: Commands,
    query: Query<Entity, With<AxisLine>>,
    user_input: Res<UserInput>,
) {
    if user_input.mouse_left.just_released {
        for entity in query.iter() {
            commands.entity(entity).try_despawn();
        }
    }
}

/// Component storing the transformation delta for an object.
///
/// This component is added to objects when they are being transformed
/// via gizmo drag and is removed after the transformation is applied.
#[derive(Component)]
pub struct TransitionDelta(Vec3);

/// Component storing the initial drag offset for a gizmo.
///
/// This component is added to gizmo entities when a drag operation starts,
/// storing the offset between the gizmo origin and the cursor position.
/// It is used to calculate accurate transformations during drag operations.
#[derive(Component)]
pub struct InitialDragOffset(Vec3);

impl InitialDragOffset {
    /// Get the initial drag offset as a Vec3.
    ///
    /// # Returns
    /// The offset vector representing the difference between the gizmo
    /// origin and the cursor position at drag start.
    pub fn offset(&self) -> Vec3 {
        self.0
    }
}
