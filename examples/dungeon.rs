//! Dungeon Example - Meshflow Vibe Editor Demonstration
//!
//! This example showcases the Meshflow Vibe editor with a dungeon scene.
//! It demonstrates custom component registration and scene loading functionality.
//!
//! ## Features Demonstrated
//!
//! - Custom component registration with `granite_component`
//! - Default component configuration
//! - Scene loading via `RequestLoadEvent`
//! - Editor plugin integration
//!
//! ## Running the Example
//!
//! To run this example:
//!
//! ```bash
//! cargo run --example dungeon
//! ```
//!
//! This will start the Meshflow Vibe editor loaded with the dungeon scene
//! located at `scenes/dungeon.scene`.
//!
//! ## Architecture
//!
//! The example follows a simple initialization flow:
//!
//! 1. Create a Bevy `App` and register editor components
//! 2. Add default Bevy plugins and Meshflow Vibe plugin
//! 3. On startup, send a `RequestLoadEvent` to load the dungeon scene
//! 4. The scene loads with `SaveSettings::Runtime` for runtime modifications
//!
//! ## Custom Components
//!
//! ### MyTestComponent
//! A simple custom component with an integer value field.
//! Registered without a default name, using the struct name.
//!
//! ### AnotherComponent
//! A component with a default configuration. Registered with
//! the name "default" and implements `Default` to provide
//! initial values for its fields.
//!
//! ## Scene Loading
//!
//! The example loads `scenes/dungeon.scene` at startup using
//! the `RequestLoadEvent` message system. The `SaveSettings::Runtime`
//! setting allows the scene to be modified during runtime.

use bevy::prelude::*;
use meshflow_vibe::prelude::*;
use meshflow_vibe_core::entities::SaveSettings;

/// Path to the starting dungeon scene file
const STARTING_WORLD: &str = "scenes/dungeon.scene";

/// A simple custom component demonstrating basic component registration.
///
/// This component can be added to entities to store an integer value.
/// It is registered without an explicit name, so it will appear
/// in the editor as "MyTestComponent".
#[granite_component]
struct MyTestComponent {
    /// The integer value stored by this component
    value: i32,
}

/// A custom component with default configuration.
///
/// Registered with the name "default", this component demonstrates
/// how to provide default values for component fields. When added
/// to an entity via the editor, it will use these defaults
/// unless explicitly overridden.
#[granite_component("default")]
struct AnotherComponent {
    /// The message string stored by this component
    message: String,
}

impl Default for AnotherComponent {
    /// Creates a new `AnotherComponent` with default values.
    ///
    /// The default message is set to "Hello, Meshflow Vibe!"
    /// which demonstrates how components initialize when added
    /// through the editor interface.
    fn default() -> Self {
        AnotherComponent {
            message: "Hello, Meshflow Vibe!".to_string(),
        }
    }
}

/// Main entry point for the dungeon example.
///
/// This function initializes the Meshflow Vibe editor application
/// with the following steps:
///
/// 1. Creates a new Bevy `App`
/// 2. Registers all editor components via the `register_editor_components!` macro
/// 3. Adds default Bevy plugins (window, logging, asset handling, etc.)
/// 4. Configures and adds the Meshflow Vibe plugin with the dungeon scene
///    as the default world to load
/// 5. Registers the `setup` system to run on startup
/// 6. Runs the application
///
/// The resulting application displays the Meshflow Vibe editor with
/// the dungeon scene loaded, allowing interactive editing of the scene
/// and its entities.
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

/// Setup system that runs on startup to load the dungeon scene.
///
/// This system is triggered when the app starts and sends a
/// `RequestLoadEvent` message to load the dungeon scene.
///
/// # Parameters
///
/// * `open_event` - A `MessageWriter` that writes to the `RequestLoadEvent` channel
///
/// The event includes:
/// - The path to the scene file (`STARTING_WORLD`)
/// - `SaveSettings::Runtime` to allow runtime modifications
/// - `None` for any parent world reference
fn setup(mut open_event: MessageWriter<RequestLoadEvent>) {
    open_event.write(RequestLoadEvent(
        STARTING_WORLD.to_string(),
        SaveSettings::Runtime,
        None,
    ));
}
