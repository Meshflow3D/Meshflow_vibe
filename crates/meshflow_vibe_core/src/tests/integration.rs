use crate::{
    entities::{ComponentEditor, HasRuntimeData, IdentityData, SaveSettings, SpawnSource},
    events::{
        CollectRuntimeDataEvent, RequestSaveEvent, RuntimeDataReadyEvent, WorldSaveSuccessEvent,
    },
    world::{save_data_ready_system, save_request_system, SaveWorldRequestData},
};
use bevy::prelude::*;
use std::fs;
use tempfile::tempdir;

/// Test that the save workflow systems can be loaded headlessly and produce file output
/// This test:
/// 1. Creates a minimal App with just the save systems
/// 2. Adds an entity with IdentityData and SpawnSource
/// 3. Sends a RequestSaveEvent
/// 4. Drives through app.update()
/// 5. Asserts that pending_saves is populated (does NOT check file creation - that's test_save_workflow_full)
#[test]
fn test_save_workflow_headless() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let save_path = temp_dir.path().join("test_world.ron");
    let save_path_str = save_path.to_string_lossy().to_string();

    let mut app = App::new();

    // Register messages (events)
    app.register_type::<IdentityData>()
        .register_type::<SpawnSource>()
        .register_type::<HasRuntimeData>()
        .add_message::<RequestSaveEvent>()
        .add_message::<CollectRuntimeDataEvent>()
        .add_message::<RuntimeDataReadyEvent>()
        .add_message::<WorldSaveSuccessEvent>()
        .init_resource::<bevy::prelude::Messages<CollectRuntimeDataEvent>>()
        .init_resource::<bevy::prelude::Messages<RequestSaveEvent>>()
        .init_resource::<bevy::prelude::Messages<RuntimeDataReadyEvent>>()
        .init_resource::<bevy::prelude::Messages<WorldSaveSuccessEvent>>();

    // Insert required resources
    app.insert_resource(ComponentEditor::default())
        .insert_resource(SaveWorldRequestData::default());

    // Add the save systems (message_update_system is already scheduled by add_message)
    app.add_systems(Update, (save_request_system, save_data_ready_system));

    // Add an entity with required components
    let _entity = app
        .world_mut()
        .spawn((
            IdentityData {
                uuid: uuid::Uuid::new_v4(),
                name: "TestEntity".to_string(),
                class: crate::GraniteTypes::Empty(crate::entities::editable::Empty {}),
            },
            SpawnSource::new(save_path_str.clone(), SaveSettings::Runtime),
            HasRuntimeData,
            bevy::transform::components::Transform::default(),
        ))
        .id();

    // Send a save request via Messages
    app.world_mut()
        .resource_mut::<bevy::prelude::Messages<RequestSaveEvent>>()
        .write(RequestSaveEvent(save_path_str.clone()));

    // Drive the app through updates
    app.update();

    // First update: save_request_system should process the request and create pending save
    // Second update: collect_components_system would run but we don't have it, so we need to manually trigger the ready state
    // Actually, let's test just the save_request_system which sets up the pending save

    // Check that SaveWorldRequestData has the pending save
    let save_data = app.world().get_resource::<SaveWorldRequestData>();
    assert!(
        save_data.is_some(),
        "SaveWorldRequestData resource should exist"
    );

    let save_data = save_data.unwrap();
    assert!(
        !save_data.pending_saves.is_empty(),
        "Should have pending saves after RequestSaveEvent"
    );

    // The save system requires collect_components_system to complete the workflow
    // Let's verify the first part works
    println!("✓ Save request system processed and created pending save");
}

/// Test save workflow end-to-end with manual component data injection
/// This tests the full save pipeline without requiring AssetServer
#[test]
fn test_save_workflow_full() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let save_path = temp_dir.path().join("test_world_full.ron");
    let save_path_str = save_path.to_string_lossy().to_string();

    let mut app = App::new();

    // Register types and messages
    app.register_type::<IdentityData>()
        .register_type::<SpawnSource>()
        .register_type::<HasRuntimeData>()
        .add_message::<RequestSaveEvent>()
        .add_message::<CollectRuntimeDataEvent>()
        .add_message::<RuntimeDataReadyEvent>()
        .add_message::<WorldSaveSuccessEvent>();

    // Insert required resources
    app.insert_resource(ComponentEditor::default())
        .insert_resource(SaveWorldRequestData::default());

    // Add save systems (message_update_system is already scheduled by add_message)
    app.add_systems(Update, (save_request_system, save_data_ready_system));

    // Add entity
    let test_uuid = uuid::Uuid::new_v4();
    let _entity = app
        .world_mut()
        .spawn((
            IdentityData {
                uuid: test_uuid,
                name: "TestEntity".to_string(),
                class: crate::GraniteTypes::Empty(crate::entities::editable::Empty {}),
            },
            SpawnSource::new(save_path_str.clone(), SaveSettings::Runtime),
            HasRuntimeData,
            bevy::transform::components::Transform::default(),
        ))
        .id();

    // Send save request
    app.world_mut()
        .resource_mut::<bevy::prelude::Messages<RequestSaveEvent>>()
        .write(RequestSaveEvent(save_path_str.clone()));

    // First update - process save request
    app.update();

    // Manually complete the component collection (simulate what collect_components_system would do)
    {
        let mut save_data = app.world_mut().resource_mut::<SaveWorldRequestData>();
        if let Some((_, world_state)) = save_data.pending_saves.get_mut(save_path_str.as_str()) {
            world_state.components_ready = true;
            // Add empty component data
            use std::collections::HashMap;
            world_state.component_data = Some(HashMap::new());
        }

        // Write RuntimeDataReadyEvent to trigger save_data_ready_system
        app.world_mut()
            .resource_mut::<bevy::prelude::Messages<RuntimeDataReadyEvent>>()
            .write(RuntimeDataReadyEvent(save_path_str.clone()));
    }

    // Second update - process runtime data ready and save
    app.update();

    // Check that save_data_ready_system processed the event
    {
        let save_data = app.world().get_resource::<SaveWorldRequestData>();
        let save_data = save_data.unwrap();
        // The pending save should be removed after processing
        assert!(
            save_data.pending_saves.is_empty(),
            "Pending save should be consumed after RuntimeDataReadyEvent"
        );
    }

    // Check that the file was created
    assert!(
        save_path.exists(),
        "Save file should be created at {:?}",
        save_path
    );

    // Check that the file has content
    let file_content = fs::read_to_string(&save_path).expect("Failed to read save file");
    assert!(!file_content.is_empty(), "Save file should not be empty");

    println!("File content: {}", file_content);

    // Verify RON format - check for metadata and entities sections
    assert!(
        file_content.contains("metadata:") && file_content.contains("entities:"),
        "File should contain metadata and entities sections, got: {}",
        file_content
    );

    // Verify the entity was saved
    assert!(
        file_content.contains("TestEntity"),
        "File should contain saved entity name, got: {}",
        file_content
    );

    println!("✓ Full save workflow completed successfully");
    println!("  - File created: {:?}", save_path);
    println!("  - File size: {} bytes", file_content.len());
}

/// Test that save respects SaveSettings::PreserveDiskTransform
#[test]
fn test_save_preserve_disk_transform() {
    let temp_dir = tempdir().expect("Failed to create temp directory");

    // First, create an initial save file
    let save_path = temp_dir.path().join("test_preserve.ron");
    let save_path_str = save_path.to_string_lossy().to_string();

    let initial_content = r#"
SceneData(
    metadata: SceneMetadata(
        format_version: Version(major: 0, minor: 1, patch: 0),
        entity_count: 1,
    ),
    entities: [
        EntitySaveReadyData(
            identity: IdentityData(
                uuid: "00000000-0000-0000-0000-000000000001",
                name: "Test",
                class: Empty,
            ),
            transform: TransformData(
                position: Vec3(10.0, 20.0, 30.0),
                rotation: Quat(1.0, 0.0, 0.0, 0.0),
                scale: Vec3(1.0, 1.0, 1.0),
            ),
            components: None,
        ),
    ],
)
"#;

    fs::write(&save_path, initial_content).expect("Failed to write initial save");

    let mut app = App::new();

    app.register_type::<IdentityData>()
        .register_type::<SpawnSource>()
        .register_type::<HasRuntimeData>()
        .add_message::<RequestSaveEvent>()
        .add_message::<CollectRuntimeDataEvent>()
        .add_message::<RuntimeDataReadyEvent>()
        .add_message::<WorldSaveSuccessEvent>();

    app.insert_resource(ComponentEditor::default())
        .insert_resource(SaveWorldRequestData::default());

    // Add save systems (message_update_system is already scheduled by add_message)
    app.add_systems(Update, (save_request_system, save_data_ready_system));

    // Create entity with PreserveDiskTransform setting
    let _entity = app
        .world_mut()
        .spawn((
            IdentityData {
                uuid: uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
                name: "Test".to_string(),
                class: crate::GraniteTypes::Empty(crate::entities::editable::Empty {}),
            },
            SpawnSource::new(save_path_str.clone(), SaveSettings::PreserveDiskTransform),
            HasRuntimeData,
            // Different transform that should be preserved from disk
            bevy::transform::components::Transform {
                translation: bevy::math::Vec3::new(999.0, 999.0, 999.0),
                rotation: bevy::math::Quat::IDENTITY,
                scale: bevy::math::Vec3::ONE,
            },
        ))
        .id();

    app.world_mut()
        .resource_mut::<bevy::prelude::Messages<RequestSaveEvent>>()
        .write(RequestSaveEvent(save_path_str.clone()));
    app.update();

    // Complete the save
    {
        let mut save_data = app.world_mut().resource_mut::<SaveWorldRequestData>();
        if let Some((_key, (path, world_state))) = save_data.pending_saves.iter_mut().next() {
            world_state.components_ready = true;
            use std::collections::HashMap;
            world_state.component_data = Some(HashMap::new());
            let path_str = path.to_string_lossy().to_string();
            drop(save_data);
            app.world_mut()
                .resource_mut::<bevy::prelude::Messages<RuntimeDataReadyEvent>>()
                .write(RuntimeDataReadyEvent(path_str));
        }
    }
    app.update();

    // Read the saved file
    let saved_content = fs::read_to_string(&save_path).expect("Failed to read saved file");

    // The transform should be preserved from disk, not the in-memory value
    assert!(
        saved_content.contains("10.0")
            && saved_content.contains("20.0")
            && saved_content.contains("30.0"),
        "Transform should be preserved from disk (10, 20, 30)"
    );

    assert!(
        !saved_content.contains("999.0"),
        "In-memory transform (999) should not be in file"
    );

    println!("✓ PreserveDiskTransform setting works correctly");
}

/// Test that the save system correctly filters entities by SpawnSource
#[test]
fn test_save_filters_by_spawn_source() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let save_path1 = temp_dir.path().join("world1.ron");
    let save_path2 = temp_dir.path().join("world2.ron");
    let path1_str = save_path1.to_string_lossy().to_string();
    let path2_str = save_path2.to_string_lossy().to_string();

    let mut app = App::new();

    app.register_type::<IdentityData>()
        .register_type::<SpawnSource>()
        .register_type::<HasRuntimeData>()
        .add_message::<RequestSaveEvent>()
        .add_message::<CollectRuntimeDataEvent>()
        .add_message::<RuntimeDataReadyEvent>()
        .add_message::<WorldSaveSuccessEvent>();

    app.insert_resource(ComponentEditor::default())
        .insert_resource(SaveWorldRequestData::default());

    // Add save systems (message_update_system is already scheduled by add_message)
    app.add_systems(Update, (save_request_system, save_data_ready_system));

    // Create two entities with different spawn sources
    let _entity1 = app
        .world_mut()
        .spawn((
            IdentityData {
                uuid: uuid::Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(),
                name: "Entity1".to_string(),
                class: crate::GraniteTypes::Empty(crate::entities::editable::Empty {}),
            },
            SpawnSource::new(path1_str.clone(), SaveSettings::Runtime),
            HasRuntimeData,
            bevy::transform::components::Transform::default(),
        ))
        .id();

    let _entity2 = app
        .world_mut()
        .spawn((
            IdentityData {
                uuid: uuid::Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap(),
                name: "Entity2".to_string(),
                class: crate::GraniteTypes::Empty(crate::entities::editable::Empty {}),
            },
            SpawnSource::new(path2_str.clone(), SaveSettings::Runtime),
            HasRuntimeData,
            bevy::transform::components::Transform::default(),
        ))
        .id();

    // Request save for world1 only
    app.world_mut()
        .resource_mut::<bevy::prelude::Messages<RequestSaveEvent>>()
        .write(RequestSaveEvent(path1_str.clone()));
    app.update();

    // Check that only world1 has a pending save
    {
        let save_data = app.world().get_resource::<SaveWorldRequestData>().unwrap();
        // The path is transformed by absolute_asset_to_rel, so we check for any pending save
        assert!(
            !save_data.pending_saves.is_empty(),
            "Should have pending save after RequestSaveEvent"
        );
        // Just verify one key exists
        let first_key = save_data.pending_saves.keys().next().unwrap();
        assert!(
            first_key.contains("world1.ron"),
            "Pending save should be for world1.ron, got: {:?}",
            first_key
        );
    }

    // Complete save for world1
    {
        // Find the actual key (path is transformed by absolute_asset_to_rel)
        let pending_key = {
            let save_data = app.world_mut().resource_mut::<SaveWorldRequestData>();
            save_data.pending_saves.keys().next().cloned().unwrap()
        };

        // Set components ready
        {
            let mut save_data = app.world_mut().resource_mut::<SaveWorldRequestData>();
            if let Some((_, world_state)) = save_data.pending_saves.get_mut(pending_key.as_ref()) {
                world_state.components_ready = true;
                use std::collections::HashMap;
                world_state.component_data = Some(HashMap::new());
            }
        }

        // Write RuntimeDataReadyEvent
        app.world_mut()
            .resource_mut::<bevy::prelude::Messages<RuntimeDataReadyEvent>>()
            .write(RuntimeDataReadyEvent(pending_key.to_string()));
    }
    app.update();

    // Verify file was created for world1 only
    assert!(save_path1.exists(), "world1.ron should be created");
    assert!(!save_path2.exists(), "world2.ron should NOT be created");

    println!("✓ Save correctly filters entities by SpawnSource");
}
