use crate::entities::{
    EntitySaveReadyData, GraniteTypes, IdentityData, SaveSettings, SceneData, SceneMetadata,
    TransformData,
};
use crate::shared::version::Version;

/// Tests for serialization structures and behavior.
///
/// These tests cover:
/// - SceneMetadata serialization/deserialization
/// - SceneData structure validation
/// - EntitySaveReadyData serialization edges
/// - TransformData conversion
use bevy::prelude::{Quat, Vec3};
use uuid::Uuid;

// ============================================================================
// TransformData Tests
// ============================================================================

#[test]
fn test_transform_data_to_bevy() {
    let transform = TransformData {
        position: Vec3::new(1.0, 2.0, 3.0),
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    let bevy_transform = transform.to_bevy();

    assert_eq!(bevy_transform.translation, Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(bevy_transform.rotation, Quat::IDENTITY);
    assert_eq!(bevy_transform.scale, Vec3::ONE);
}

#[test]
fn test_transform_data_default() {
    let transform = TransformData::default();

    assert_eq!(transform.position, Vec3::ZERO);
    assert_eq!(transform.rotation, Quat::IDENTITY);
    assert_eq!(transform.scale, Vec3::ZERO);
}

#[test]
fn test_transform_data_equality() {
    let t1 = TransformData {
        position: Vec3::new(1.0, 2.0, 3.0),
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    let t2 = TransformData {
        position: Vec3::new(1.0, 2.0, 3.0),
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    assert_eq!(t1, t2);
}

#[test]
fn test_transform_data_inequality() {
    let t1 = TransformData {
        position: Vec3::new(1.0, 2.0, 3.0),
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    let t2 = TransformData {
        position: Vec3::new(1.0, 2.0, 4.0), // Different z
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    assert_ne!(t1, t2);
}

#[test]
fn test_transform_data_clone() {
    let t1 = TransformData {
        position: Vec3::new(1.0, 2.0, 3.0),
        rotation: Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
        scale: Vec3::new(2.0, 2.0, 2.0),
    };

    let t2 = t1.clone();
    assert_eq!(t1, t2);
}

// ============================================================================
// IdentityData Tests
// ============================================================================

#[test]
fn test_identity_data_default() {
    let identity = IdentityData::default();

    assert_eq!(identity.uuid, Uuid::nil());
    assert_eq!(identity.name, String::new());
}

#[test]
fn test_identity_data_equality() {
    let uuid = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();

    let i1 = IdentityData {
        uuid,
        name: "TestEntity".to_string(),
        class: GraniteTypes::Empty(Default::default()),
    };

    let i2 = IdentityData {
        uuid,
        name: "TestEntity".to_string(),
        class: GraniteTypes::Empty(Default::default()),
    };

    assert_eq!(i1, i2);
}

#[test]
fn test_identity_data_inequality_name() {
    let uuid = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();

    let i1 = IdentityData {
        uuid,
        name: "Entity1".to_string(),
        class: GraniteTypes::Empty(Default::default()),
    };

    let i2 = IdentityData {
        uuid,
        name: "Entity2".to_string(),
        class: GraniteTypes::Empty(Default::default()),
    };

    assert_ne!(i1, i2);
}

#[test]
fn test_identity_data_clone() {
    let uuid = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();

    let i1 = IdentityData {
        uuid,
        name: "Test".to_string(),
        class: GraniteTypes::Empty(Default::default()),
    };

    let i2 = i1.clone();
    assert_eq!(i1, i2);
}

// ============================================================================
// SceneMetadata Tests
// ============================================================================

#[test]
fn test_scene_metadata_default_values() {
    let metadata = SceneMetadata {
        format_version: Version::V0_1_4,
        entity_count: 0,
    };

    assert_eq!(metadata.format_version, Version::V0_1_4);
    assert_eq!(metadata.entity_count, 0);
}

#[test]
fn test_scene_metadata_with_entities() {
    let metadata = SceneMetadata {
        format_version: Version::V0_1_4,
        entity_count: 42,
    };

    assert_eq!(metadata.format_version, Version::V0_1_4);
    assert_eq!(metadata.entity_count, 42);
}

#[test]
fn test_scene_metadata_serialization() {
    let metadata = SceneMetadata {
        format_version: Version::V0_1_4,
        entity_count: 10,
    };

    let serialized = serde_json::to_string(&metadata).unwrap();
    let deserialized: SceneMetadata = serde_json::from_str(&serialized).unwrap();

    assert_eq!(metadata, deserialized);
}

#[test]
fn test_scene_metadata_serialization_v0_1_5() {
    let metadata = SceneMetadata {
        format_version: Version::V0_1_5,
        entity_count: 5,
    };

    let serialized = serde_json::to_string(&metadata).unwrap();
    let deserialized: SceneMetadata = serde_json::from_str(&serialized).unwrap();

    assert_eq!(metadata, deserialized);
}

#[test]
fn test_scene_metadata_equality() {
    let m1 = SceneMetadata {
        format_version: Version::V0_1_4,
        entity_count: 5,
    };

    let m2 = SceneMetadata {
        format_version: Version::V0_1_4,
        entity_count: 5,
    };

    assert_eq!(m1, m2);
}

#[test]
fn test_scene_metadata_inequality_version() {
    let m1 = SceneMetadata {
        format_version: Version::V0_1_4,
        entity_count: 5,
    };

    let m2 = SceneMetadata {
        format_version: Version::V0_1_5,
        entity_count: 5,
    };

    assert_ne!(m1, m2);
}

#[test]
fn test_scene_metadata_inequality_count() {
    let m1 = SceneMetadata {
        format_version: Version::V0_1_4,
        entity_count: 5,
    };

    let m2 = SceneMetadata {
        format_version: Version::V0_1_4,
        entity_count: 10,
    };

    assert_ne!(m1, m2);
}

// ============================================================================
// EntitySaveReadyData Tests
// ============================================================================

#[test]
fn test_entity_save_ready_data_minimal() {
    let uuid = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();

    let entity = EntitySaveReadyData {
        identity: IdentityData {
            uuid,
            name: "Test".to_string(),
            class: GraniteTypes::Empty(Default::default()),
        },
        transform: TransformData::default(),
        parent: None,
        components: None,
    };

    assert_eq!(entity.identity.uuid, uuid);
    assert_eq!(entity.identity.name, "Test");
    assert!(entity.parent.is_none());
    assert!(entity.components.is_none());
}

#[test]
fn test_entity_save_ready_data_with_parent() {
    let uuid = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();
    let parent_uuid = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174001").unwrap();

    let entity = EntitySaveReadyData {
        identity: IdentityData {
            uuid,
            name: "Test".to_string(),
            class: GraniteTypes::Empty(Default::default()),
        },
        transform: TransformData::default(),
        parent: Some(parent_uuid),
        components: None,
    };

    assert_eq!(entity.identity.uuid, uuid);
    assert_eq!(entity.parent, Some(parent_uuid));
}

#[test]
fn test_entity_save_ready_data_with_components() {
    let uuid = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();

    let mut components = std::collections::HashMap::new();
    components.insert("key1".to_string(), "value1".to_string());
    components.insert("key2".to_string(), "value2".to_string());

    let entity = EntitySaveReadyData {
        identity: IdentityData {
            uuid,
            name: "Test".to_string(),
            class: GraniteTypes::Empty(Default::default()),
        },
        transform: TransformData::default(),
        parent: None,
        components: Some(components),
    };

    assert!(entity.components.is_some());
    assert_eq!(entity.components.as_ref().unwrap().len(), 2);
}

#[test]
fn test_entity_save_ready_data_serialization() {
    let uuid = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();

    let entity = EntitySaveReadyData {
        identity: IdentityData {
            uuid,
            name: "Test".to_string(),
            class: GraniteTypes::Empty(Default::default()),
        },
        transform: TransformData {
            position: Vec3::new(1.0, 2.0, 3.0),
            rotation: Quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
            scale: Vec3::ONE,
        },
        parent: None,
        components: None,
    };

    let serialized = ron::to_string(&entity).unwrap();
    let deserialized: EntitySaveReadyData = ron::from_str(&serialized).unwrap();

    assert_eq!(entity.identity.uuid, deserialized.identity.uuid);
    assert_eq!(entity.identity.name, deserialized.identity.name);
    assert_eq!(entity.transform, deserialized.transform);
}

#[test]
fn test_entity_save_ready_data_serialization_with_parent() {
    let uuid = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();
    let parent_uuid = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174001").unwrap();

    let entity = EntitySaveReadyData {
        identity: IdentityData {
            uuid,
            name: "Test".to_string(),
            class: GraniteTypes::Empty(Default::default()),
        },
        transform: TransformData::default(),
        parent: Some(parent_uuid),
        components: None,
    };

    let serialized = ron::to_string(&entity).unwrap();
    let deserialized: EntitySaveReadyData = ron::from_str(&serialized).unwrap();

    assert_eq!(entity.parent, deserialized.parent);
}

#[test]
fn test_entity_save_ready_data_serialization_with_components() {
    let uuid = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();

    let mut components = std::collections::HashMap::new();
    components.insert("mesh".to_string(), "cube.ron".to_string());

    let entity = EntitySaveReadyData {
        identity: IdentityData {
            uuid,
            name: "Test".to_string(),
            class: GraniteTypes::Empty(Default::default()),
        },
        transform: TransformData::default(),
        parent: None,
        components: Some(components),
    };

    let serialized = ron::to_string(&entity).unwrap();
    let deserialized: EntitySaveReadyData = ron::from_str(&serialized).unwrap();

    assert!(deserialized.components.is_some());
    assert_eq!(
        deserialized.components.as_ref().unwrap().get("mesh"),
        Some(&"cube.ron".to_string())
    );
}

// ============================================================================
// SceneData Tests
// ============================================================================

#[test]
fn test_scene_data_empty() {
    let metadata = SceneMetadata {
        format_version: Version::V0_1_4,
        entity_count: 0,
    };

    let scene = SceneData {
        metadata,
        entities: Vec::new(),
    };

    assert_eq!(scene.entities.len(), 0);
    assert_eq!(scene.metadata.entity_count, 0);
}

#[test]
fn test_scene_data_with_entities() {
    let uuid = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();

    let metadata = SceneMetadata {
        format_version: Version::V0_1_4,
        entity_count: 1,
    };

    let entity = EntitySaveReadyData {
        identity: IdentityData {
            uuid,
            name: "Test".to_string(),
            class: GraniteTypes::Empty(Default::default()),
        },
        transform: TransformData::default(),
        parent: None,
        components: None,
    };

    let scene = SceneData {
        metadata,
        entities: vec![entity],
    };

    assert_eq!(scene.entities.len(), 1);
    assert_eq!(scene.metadata.entity_count, 1);
}

#[test]
fn test_scene_data_serialization() {
    let metadata = SceneMetadata {
        format_version: Version::V0_1_4,
        entity_count: 0,
    };

    let scene = SceneData {
        metadata,
        entities: Vec::new(),
    };

    let serialized = ron::to_string(&scene).unwrap();
    let deserialized: SceneData = ron::from_str(&serialized).unwrap();

    assert_eq!(scene.metadata, deserialized.metadata);
    assert_eq!(scene.entities.len(), deserialized.entities.len());
}

#[test]
fn test_scene_data_serialization_with_entities() {
    let uuid = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();

    let metadata = SceneMetadata {
        format_version: Version::V0_1_4,
        entity_count: 1,
    };

    let entity = EntitySaveReadyData {
        identity: IdentityData {
            uuid,
            name: "Test".to_string(),
            class: GraniteTypes::Empty(Default::default()),
        },
        transform: TransformData::default(),
        parent: None,
        components: None,
    };

    let scene = SceneData {
        metadata,
        entities: vec![entity],
    };

    let serialized = ron::to_string(&scene).unwrap();
    let deserialized: SceneData = ron::from_str(&serialized).unwrap();

    assert_eq!(
        scene.metadata.format_version,
        deserialized.metadata.format_version
    );
    assert_eq!(
        scene.metadata.entity_count,
        deserialized.metadata.entity_count
    );
    assert_eq!(
        scene.entities[0].identity.uuid,
        deserialized.entities[0].identity.uuid
    );
}

// ============================================================================
// SaveSettings Tests
// ============================================================================

#[test]
fn test_save_settings_default() {
    let settings: SaveSettings = Default::default();
    assert_eq!(settings, SaveSettings::Runtime);
}

#[test]
fn test_save_settings_equality() {
    assert_eq!(SaveSettings::Runtime, SaveSettings::Runtime);
    assert_eq!(
        SaveSettings::PreserveDiskFull,
        SaveSettings::PreserveDiskFull
    );
    assert_eq!(
        SaveSettings::PreserveDiskTransform,
        SaveSettings::PreserveDiskTransform
    );
}

#[test]
fn test_save_settings_inequality() {
    assert_ne!(SaveSettings::Runtime, SaveSettings::PreserveDiskFull);
    assert_ne!(SaveSettings::Runtime, SaveSettings::PreserveDiskTransform);
    assert_ne!(
        SaveSettings::PreserveDiskFull,
        SaveSettings::PreserveDiskTransform
    );
}

#[test]
fn test_save_settings_clone() {
    let settings = SaveSettings::PreserveDiskFull;
    let settings2 = settings.clone();
    assert_eq!(settings, settings2);
}
