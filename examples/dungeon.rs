//dungeon.scene designed by Noah Booker
use bevy::prelude::*;
use meshflow_vibe::prelude::*;
use meshflow_vibe_core::entities::SaveSettings;

const STARTING_WORLD: &str = "scenes/dungeon.scene";

#[granite_component]
struct MyTestComponent {
    value: i32,
}

#[granite_component("default")]
struct AnotherComponent {
    message: String,
}

impl Default for AnotherComponent {
    fn default() -> Self {
        AnotherComponent {
            message: "Hello, Meshflow Vibe!".to_string(),
        }
    }
}

fn main() {
    let mut app = App::new();
    register_editor_components!();

    app.add_plugins(DefaultPlugins)
        .add_plugins(meshflow_vibe::MeshflowVibe {
            default_world: STARTING_WORLD.to_string(),
            ..Default::default()
        })
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut open_event: MessageWriter<RequestLoadEvent>) {
    open_event.write(RequestLoadEvent(
        STARTING_WORLD.to_string(),
        SaveSettings::Runtime,
        None,
    ));
}
