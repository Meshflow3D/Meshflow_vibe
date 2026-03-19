use super::{
    config::{VertexSelectionState, VertexVisualizationConfig},
    interaction::{deselect_all_vertices, handle_vertex_click, update_vertex_colors},
    midpoint::calculate_vertex_midpoint,
    spawn::{
        cleanup_deselected_entity_vertices, cull_vertices_by_distance,
        despawn_vertex_visualizations, spawn_vertex_visualizations,
    },
};
use bevy::app::{App, Plugin, Update};

pub struct VertexVisualizationPlugin;

impl Plugin for VertexVisualizationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(VertexVisualizationConfig::default())
            .insert_resource(VertexSelectionState::default())
            // Systems
            .add_systems(
                Update,
                (spawn_vertex_visualizations, despawn_vertex_visualizations),
            )
            .add_systems(
                Update,
                (
                    cleanup_deselected_entity_vertices,
                    cull_vertices_by_distance,
                ),
            )
            .add_systems(
                Update,
                (
                    update_vertex_colors,
                    deselect_all_vertices,
                    calculate_vertex_midpoint,
                ),
            )
            // Observer for vertex clicks
            .add_observer(handle_vertex_click);
    }
}
