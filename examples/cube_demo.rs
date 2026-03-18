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
use meshflow_vibe_core::entities::editable::types::{Camera3D, DirLight};

fn main() {
    let mut app = App::new();
    register_editor_components!();

    app.add_plugins(DefaultPlugins)
        .add_plugins(meshflow_vibe::MeshflowVibe {
            active: true,
            default_world: "default".to_string(),
            logging: true,
        })
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let transform =
        Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y);
    Camera3D::default().spawn_from_new_identity(&mut commands, transform);

    let light_transform = Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);
    DirLight::default().spawn_from_new_identity(&mut commands, light_transform);

    commands.spawn((
        Name::new("Cube"),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}
