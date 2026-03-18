use super::{
    collect_components_system, open_world_batch_reader, open_world_reader, reload_world_system,
    save_data_ready_system, save_request_system, SaveWorldRequestData,
};
use bevy::app::{App, Plugin, Update};

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // Resources
            //
            .init_resource::<SaveWorldRequestData>()
            //
            // Schedule system
            //
            .add_systems(Update, (open_world_reader, open_world_batch_reader))
            .add_systems(
                Update,
                (
                    collect_components_system,
                    reload_world_system,
                    save_request_system,
                    save_data_ready_system,
                ),
            );
    }
}
