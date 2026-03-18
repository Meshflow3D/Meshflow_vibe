# Meshflow Vibe 

[![Version](https://img.shields.io/crates/v/meshflow_vibe.svg)](https://crates.io/crates/meshflow_vibe)
[![License](https://img.shields.io/crates/l/meshflow_vibe)](LICENSE-MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/Meshflow3D/Meshflow_vibe/ci.yml?branch=main)](https://github.com/Meshflow3D/Meshflow_vibe/actions)
[![Bevy 0.18](https://img.shields.io/badge/Bevy-0.18.1-orange.svg)](https://bevyengine.org)

This crate provides a way to interactively create, edit, save, and load Bevy data in 3D.

> [!CAUTION]
> This is in early development and you will likely encounter bugs

![Main Editor](./screenshots/Image_4.png)

---

## Getting Started

### Prerequisites

- Rust 1.75 or later
- Bevy 0.18.1 (see [Compatibility](#compatibility))
- A Windows, macOS, or Linux system with graphics support

### Installation

Add `meshflow_vibe` to your project's `Cargo.toml`:

```toml
[dependencies]
bevy = "0.18.1"
meshflow_vibe = { git = "https://github.com/Meshflow3D/Meshflow_vibe", branch = "main" }
serde = "*"
```

### Feature Sets

This crate supports three optional feature sets:

| Feature   | Description                                           | Includes              |
|-----------|-------------------------------------------------------|-----------------------|
| `gizmos`  | Visual debugging tools and entity manipulation        | Core functionality    |
| `core`    | Bare serialization/deserialization with macros & logging | None (base)         |
| `editor`  | Full in-game editor with UI components                | Core + Gizmos         |
| `default` | All features enabled                                  | Core + Editor + Gizmos|

**Example: Core only (no editor or gizmos)**

```toml
[dependencies]
meshflow_vibe = { git = "https://github.com/Meshflow3D/Meshflow_vibe", branch = "main", default-features = false, features = ["core"] }
```

**Example: Gizmos only**

```toml
[dependencies]
meshflow_vibe = { git = "https://github.com/Meshflow3D/Meshflow_vibe", branch = "main", default-features = false, features = ["gizmos"] }
```

---

## Quick Start

1. Navigate to your project's `Cargo.toml` and add the dependency
2. Register editor components in your `main()` function
3. Add the `MeshflowVibe` plugin group to your Bevy app

```rust
use bevy::prelude::*;
use meshflow_vibe::prelude::*;

const STARTING_WORLD: &str = "scenes/starting.scene";

fn main() {
    let mut app = App::new();
    register_editor_components!();

    app.add_plugins(DefaultPlugins)
        .add_plugins(MeshflowVibe {
            default_world: STARTING_WORLD.to_string(),
            ..Default::default()
        })
        .run();
}
```

---

## Examples

Check out the [examples](https://github.com/Meshflow3D/Meshflow_vibe/tree/main/examples) which showcase how to set up a project.

### Dungeon Example

The `dungeon` example provides a simple entry point file with code ready to start editing. Make sure to copy over the relevant `assets` subfolder or you will get errors.

```bash
# If you clone this repo directly, you can use the example argument
cargo run --release --example dungeon
```

---

## Running Tests

This project uses Rust's built-in test infrastructure. To run all tests locally:

```bash
cargo test --workspace --all-targets
```

The test suite includes unit tests, integration tests, and headless Bevy tests that run without requiring a display. These tests are automatically run in CI on every push and pull request to `main`.

---

## Compatibility

| Bevy Version | meshflow_vibe Version |
|--------------|----------------------|
| 0.18         | 0.3.1                |
| 0.17         | 0.3.0                |
| 0.16         | 0.2.0 - 0.2.2        |
| 0.14         | 0.1.0                |

---

## Feature Comparison

| Feature                  | Core | Gizmos | Editor | Default |
|--------------------------|------|--------|--------|---------|
| Logging System           | ✅   | ✅     | ✅     | ✅      |
| Serialization Macros     | ✅   | ✅     | ✅     | ✅      |
| Scene Serialization      | ✅   | ✅     | ✅     | ✅      |
| Visual Debugging         |      | ✅     | ✅     | ✅      |
| Entity Manipulation      |      | ✅     | ✅     | ✅      |
| In-Game Editor UI        |      |        | ✅     | ✅      |
| Entity Management        |      |        | ✅     | ✅      |
| World Save/Load          | ✅   | ✅     | ✅     | ✅      |

---

## API Documentation

Full API documentation is available at:
- [API Docs](https://docs.rs/meshflow_vibe) - Crates.io documentation
- [GitHub Repository](https://github.com/Meshflow3D/Meshflow_vibe) - Source code and examples

Generate local documentation with:

```bash
cargo doc --no-deps --open
```

---

## Features

### Scene Serialization

An entity is stored as three main parts:

- **Identity**: Contains the entity's name, uuid, and type/class (such as Camera, Light, OBJ). This class data contains everything necessary to rebuild this bundle and any other adjacently relevant data. Not everything is currently available in classes.
- **Transform**: Describes the entity's position, rotation, and scale. This determines where the entity is located and how it is oriented in the world.
- **Components**: (Optional) Holds additional data or behaviors attached to the entity. This is where you extend the entity's functionality via the `#[granite_component]` macro.

A scene file contains metadata and a list of serializable entity data. Check out the [assets/scenes](https://github.com/Meshflow3D/Meshflow_vibe/tree/main/assets/scenes) for scene examples.

### Callable Events

While comprehensive documentation is currently unavailable, here are some helpful events you can use to interact with the editor:

#### Editor Control Events

- `RequestEditorToggle` - Toggle the editor UI on/off
- `RequestToggleCameraSync` - Toggle camera synchronization between editor and main camera

#### Entity Selection Events

- `RequestSelectEntityEvent` - Select an entity (additive for multi-selection)
- `RequestDeselectEntityEvent` - Deselect a specific entity
- `RequestDeselectAllEntitiesEvent` - Clear all entity selections
- `RequestCameraEntityFrame` - Frame the UI camera to focus on active entity

#### Entity Duplication Events

- `RequestDuplicateEntityEvent` - Duplicate a specific entity
- `RequestDuplicateAllSelectionEvent` - Duplicate all currently selected entities

#### Entity Hierarchy Events

- `RequestNewParent` - Request to set active as parent for selected entities
- `RequestRemoveParents` - Remove parent relationships from selected entities
- `RequestRemoveChildren` - Remove child relationships from selected entities

#### World Management Events

- `RequestSaveEvent` - Save the specific world
- `RequestLoadEvent` - Load a world from specified path
- `RequestReloadEvent` - Reload a world from specified path
- `WorldLoadSuccessEvent` - Event sent when world loading completes successfully
- `WorldSaveSuccessEvent` - Event sent when world saving completes successfully
- `RequestDespawnSerializableEntities` - Event to despawn all serializable entities
- `RequestDespawnBySource` - Event to despawn a specific source that is loaded

### Custom UI Callable Events

> Only Bevy Event unit structs are supported for UI button rendering.

With version 0.2.x, there is a new window that renders users buttons that are clickable. Create a struct that holds your events, and add `#[ui_callable_events]`. This will add all the events to the events window as clickable, and will dispatch said event in your struct.

Make sure to call UI registration before the plugin gets initialized in your app if your using this. `DebugEvents::register_ui();`.

**Example:**

```rust
use bevy::prelude::*;
use meshflow_vibe::prelude::*;

#[derive(Event, Default)]
pub struct DebugRequestPlayer;

#[derive(Event, Default)]
pub struct DebugRequestRemovePlayer;

#[ui_callable_events] 
pub struct DebugEvents {
    pub spawn_player: DebugRequestPlayer,
    pub remove_player: DebugRequestRemovePlayer,
}

pub fn debug_callable_watcher(
    mut despawn: MessageReader<DebugRequestRemovePlayer>,
    mut spawn: MessageReader<DebugRequestPlayer>,
    mut commands: Commands,
    mut player_start: Query<(&GlobalTransform, &mut PlayerSpawner)>,
    mut world_state: ResMut<WorldState>,
) {
    for _ in despawn.read() {
        commands.send_event(RequestDespawnBySource(PLAYER_PREFAB.to_string()));
    }

    for _ in spawn.read() {
        spawn_player(&mut commands, &mut world_state, &mut player_start);
    }
}
```

---

## Screenshots

![Editor UI](./screenshots/Image_1.png)
![Entity Tree](./screenshots/Image_2.png)
![Component View](./screenshots/Image_3.png)
![Custom UI Events](./screenshots/Image_5.png)

---

## License

Meshflow Vibe is free and open source. Except when noted, all assets are licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

**Any contributions by you, shall be dual licensed as above, without any additional terms or conditions.**

---

## Contributors

<a href="https://github.com/Meshflow3D/Meshflow_vibe/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Meshflow3D/Meshflow_vibe" />
</a>

---

## Support

If you have any feedback or questions, please reach out via:

- [GitHub Issues](https://github.com/Meshflow3D/Meshflow_vibe/issues) - Bug reports and feature requests
- [GitHub Discussions](https://github.com/Meshflow3D/Meshflow_vibe/discussions) - Questions and general discussion

---

## Special Thanks

- Noah
- Silas
- Ethan
- Max

### Creator

[@BlakeDarrow](https://www.youtube.com/@blakedarrow) on YouTube
