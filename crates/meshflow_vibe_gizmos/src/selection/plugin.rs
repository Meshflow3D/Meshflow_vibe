use super::{
    apply_pending_parents, duplicate_all_selection_system, duplicate_entity_system,
    handle_picking_selection, select_entity, RaycastCursorLast, RaycastCursorPos,
    RequestDuplicateAllSelectionEvent, RequestDuplicateEntityEvent,
};
use crate::{
    gizmos::vertex::{components::SelectedVertex, config::VertexSelectionState},
    is_gizmos_active,
    selection::manager::deselect_entity,
};
use bevy::{
    app::{App, Plugin, PostUpdate, Update},
    ecs::schedule::IntoScheduleConfigs,
    ecs::{
        message::MessageReader, prelude::Commands, prelude::Entity, prelude::Query,
        prelude::ResMut, prelude::With,
    },
    math::Vec3,
};
use meshflow_vibe_core::entities::edit_mode::OnExitEditMode;

/// Cleanup on edit mode exit.
///
/// Removes stale edit-mode-specific element-selection state and visuals:
/// - Clears all `SelectedVertex` components (vertex selections)
/// - Resets `VertexSelectionState` resource (vertex selections, midpoints)
/// - Preserves object-mode entity selection (`Selected` and `ActiveSelection`)
/// - This prevents stale vertex/edge/face selections from persisting in object mode.
fn cleanup_selection_on_edit_mode_exit(
    mut messages: MessageReader<OnExitEditMode>,
    mut commands: Commands,
    selected_vertices: Query<Entity, With<SelectedVertex>>,
    mut vertex_state: ResMut<VertexSelectionState>,
) {
    for _event in messages.read() {
        // Clear vertex selections only (edit-mode specific)
        for entity in selected_vertices.iter() {
            commands.entity(entity).remove::<SelectedVertex>();
        }
        // Reset vertex selection state
        vertex_state.selected_vertices.clear();
        vertex_state.midpoint_world = None;
    }
}

pub struct SelectionPlugin;
impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // Events
            //
            .add_message::<RequestDuplicateEntityEvent>()
            .add_message::<RequestDuplicateAllSelectionEvent>()
            //
            // Resources
            //
            .insert_resource(RaycastCursorLast {
                position: Vec3::ZERO,
            })
            .insert_resource(RaycastCursorPos {
                position: Vec3::ZERO,
            })
            //
            // Events
            //
            //
            // Schedule system
            //
            .add_systems(
                Update,
                (duplicate_entity_system, duplicate_all_selection_system).run_if(is_gizmos_active),
            )
            .add_systems(PostUpdate, (apply_pending_parents).run_if(is_gizmos_active))
            .add_observer(handle_picking_selection)
            .add_observer(super::manager::single_active)
            .add_observer(select_entity)
            .add_observer(deselect_entity)
            .add_systems(PostUpdate, cleanup_selection_on_edit_mode_exit);
    }
}
