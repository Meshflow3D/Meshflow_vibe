//! Cube Demo - Meshflow Vibe Simple Demo
//!
//! This example demonstrates the Meshflow Vibe editor with a simple cube scene.
//! It creates a minimal scene with a cube, light, and camera.
//!
//! ## Features Demonstrated
//!
//! - Basic editor initialization
//! - Simple scene with cube, light, and camera
//! - Editor plugin integration
//!
//! ## Running the Example
//!
//! ```bash
//! cargo run --example cube_demo
//! ```

use bevy::prelude::*;
use meshflow_vibe::prelude::*;

fn main() {
    let mut app = App::new();
    register_editor_components!();

    app.add_plugins(DefaultPlugins)
        .add_plugins(meshflow_vibe::MeshflowVibe {
            default_world: "".to_string(),
            ..Default::default()
        })
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));

    // Light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Cube with mesh
    commands.spawn((
        Name::new("Cube"),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}
