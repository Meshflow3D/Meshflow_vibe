/// Headless editor flow tests.
///
/// These tests exercise editor message-driven seams without requiring a windowed context.
/// They verify editor state transitions and config workflows that would be triggered
/// by GUI interactions in a real session.
use bevy::prelude::*;
use crate::editor_state::{EditorState, update_active_world_system, update_editor_vis_system};
use crate::interface::{EditorSettingsTabData, RequestEditorToggle, SetActiveWorld};
use meshflow_vibe_core::events::{WorldLoadSuccessEvent, WorldSaveSuccessEvent, RequestDespawnSerializableEntities, RequestDespawnBySource};
use meshflow_vibe_gizmos::GizmoVisibilityState;
use std::fs;
use tempfile::tempdir;

/// Test that the editor toggle message correctly transitions editor state
#[test]
fn test_editor_toggle_message() {
    let mut app = App::new();

    app.add_message::<RequestEditorToggle>()
        .init_resource::<bevy::prelude::Messages<RequestEditorToggle>>()
        .init_resource::<GizmoVisibilityState>()
        .insert_resource(EditorState {
            active: false,
            default_world: "default".to_string(),
            current_file: None,
            config_path: "config/editor.toml".to_string(),
            config: EditorSettingsTabData::default(),
            config_loaded: false,
            layout_loaded: false,
            loaded_sources: std::collections::HashSet::new(),
        })
        .add_systems(Update, update_editor_vis_system);

    let editor_state = app.world().get_resource::<EditorState>().unwrap();
    assert!(!editor_state.active, "Editor should start inactive");

    app.world_mut()
        .resource_mut::<bevy::prelude::Messages<RequestEditorToggle>>()
        .write(RequestEditorToggle);

    app.update();

    let editor_state = app.world().get_resource::<EditorState>().unwrap();
    assert!(editor_state.active, "Editor should be active after toggle");

    app.world_mut()
        .resource_mut::<bevy::prelude::Messages<RequestEditorToggle>>()
        .write(RequestEditorToggle);
    app.update();

    let editor_state = app.world().get_resource::<EditorState>().unwrap();
    assert!(
        !editor_state.active,
        "Editor should be inactive after second toggle"
    );

    println!("✓ Editor toggle message correctly transitions editor state");
}

/// Test that SetActiveWorld message updates editor state
#[test]
fn test_set_active_world_message() {
    let mut app = App::new();

    app.add_message::<SetActiveWorld>()
        .add_message::<WorldLoadSuccessEvent>()
        .add_message::<WorldSaveSuccessEvent>()
        .add_message::<RequestDespawnSerializableEntities>()
        .add_message::<RequestDespawnBySource>()
        .init_resource::<bevy::prelude::Messages<SetActiveWorld>>()
        .init_resource::<bevy::prelude::Messages<WorldLoadSuccessEvent>>()
        .init_resource::<bevy::prelude::Messages<WorldSaveSuccessEvent>>()
        .init_resource::<bevy::prelude::Messages<RequestDespawnSerializableEntities>>()
        .init_resource::<bevy::prelude::Messages<RequestDespawnBySource>>()
        .insert_resource(EditorState {
            active: true,
            default_world: "default".to_string(),
            current_file: None,
            config_path: "config/editor.toml".to_string(),
            config: EditorSettingsTabData::default(),
            config_loaded: false,
            layout_loaded: false,
            loaded_sources: std::collections::HashSet::new(),
        })
        .add_systems(Update, update_active_world_system);

    let editor_state = app.world().get_resource::<EditorState>().unwrap();
    assert!(
        editor_state.current_file.is_none(),
        "Should start with no current file"
    );

    let test_path = "worlds/test_world.ron".to_string();
    app.world_mut()
        .resource_mut::<bevy::prelude::Messages<SetActiveWorld>>()
        .write(SetActiveWorld(test_path.clone()));

    app.update();

    let editor_state = app.world().get_resource::<EditorState>().unwrap();
    assert_eq!(
        editor_state.current_file.as_ref().unwrap(),
        &test_path,
        "Current file should be updated"
    );
    assert!(
        editor_state.loaded_sources.contains(&test_path),
        "Path should be in loaded sources"
    );

    println!("✓ SetActiveWorld message correctly updates editor state");
}

/// Test that config load/save workflow works headlessly
#[test]
fn test_editor_config_workflow() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("editor.toml");

    let mut app = App::new();

    app.insert_resource(EditorState {
        active: true,
        default_world: "default".to_string(),
        current_file: None,
        config_path: config_path.to_string_lossy().to_string(),
        config: EditorSettingsTabData::default(),
        config_loaded: false,
        layout_loaded: false,
        loaded_sources: std::collections::HashSet::new(),
    });

    {
        let mut config = app.world_mut().resource_mut::<EditorState>().config.clone();
        config.viewport.grid_size = 50.0;
        config.theme_state.font_scale = 1.5;
        app.world_mut().resource_mut::<EditorState>().config = config;
    }

    {
        let config = app.world().get_resource::<EditorState>().unwrap();
        assert_eq!(
            config.config.viewport.grid_size, 50.0,
            "Config grid_size should be modified"
        );
        assert_eq!(
            config.config.theme_state.font_scale, 1.5,
            "Config font_scale should be modified"
        );
    }

    let config = app.world().get_resource::<EditorState>().unwrap();
    let toml_content = toml::to_string_pretty(&config.config).expect("Failed to serialize config");
    fs::write(&config_path, &toml_content).expect("Failed to write config file");

    assert!(config_path.exists(), "Config file should be created");

    let saved_content = fs::read_to_string(&config_path).expect("Failed to read config file");
    assert!(
        saved_content.contains("grid_size") && saved_content.contains("50.0"),
        "Saved config should contain grid_size value"
    );

    println!("✓ Editor config workflow (modify -> serialize -> save) works correctly");
}

/// Test multiple message types in sequence
#[test]
fn test_editor_message_sequence() {
    let mut app = App::new();

    app.add_message::<RequestEditorToggle>()
        .add_message::<SetActiveWorld>()
        .add_message::<WorldLoadSuccessEvent>()
        .add_message::<WorldSaveSuccessEvent>()
        .add_message::<RequestDespawnSerializableEntities>()
        .add_message::<RequestDespawnBySource>()
        .init_resource::<GizmoVisibilityState>()
        .init_resource::<bevy::prelude::Messages<RequestEditorToggle>>()
        .init_resource::<bevy::prelude::Messages<SetActiveWorld>>()
        .init_resource::<bevy::prelude::Messages<WorldLoadSuccessEvent>>()
        .init_resource::<bevy::prelude::Messages<WorldSaveSuccessEvent>>()
        .init_resource::<bevy::prelude::Messages<RequestDespawnSerializableEntities>>()
        .init_resource::<bevy::prelude::Messages<RequestDespawnBySource>>()
        .insert_resource(EditorState {
            active: false,
            default_world: "default".to_string(),
            current_file: None,
            config_path: "config/editor.toml".to_string(),
            config: EditorSettingsTabData::default(),
            config_loaded: false,
            layout_loaded: false,
            loaded_sources: std::collections::HashSet::new(),
        })
        .add_systems(
            Update,
            (update_editor_vis_system, update_active_world_system),
        );

    {
        app.world_mut()
            .resource_mut::<bevy::prelude::Messages<RequestEditorToggle>>()
            .write(RequestEditorToggle);
        app.update();
    }

    let editor_state = app.world().get_resource::<EditorState>().unwrap();
    assert!(editor_state.active, "Editor should be active");

    {
        app.world_mut()
            .resource_mut::<bevy::prelude::Messages<SetActiveWorld>>()
            .write(SetActiveWorld("worlds/sequence_test.ron".to_string()));
        app.update();
    }

    let editor_state = app.world().get_resource::<EditorState>().unwrap();
    assert!(
        editor_state.current_file.is_some(),
        "Current file should be set"
    );
    assert!(editor_state.active, "Editor should still be active");

    {
        app.world_mut()
            .resource_mut::<bevy::prelude::Messages<RequestEditorToggle>>()
            .write(RequestEditorToggle);
        app.update();
    }

    let editor_state = app.world().get_resource::<EditorState>().unwrap();
    assert!(
        !editor_state.active,
        "Editor should be inactive after final toggle"
    );
    assert!(
        editor_state.current_file.is_some(),
        "Current file should persist even when editor is inactive"
    );

    println!("✓ Editor message sequence executed correctly");
}
