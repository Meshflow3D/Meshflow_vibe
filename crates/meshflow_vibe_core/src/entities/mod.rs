use std::borrow::Cow;
use std::collections::HashMap;

use crate::topology::EditableTopology;

use bevy::{
    ecs::{component::Component, resource::Resource},
    prelude::{
        Quat, ReflectComponent, ReflectDefault, ReflectDeserialize, ReflectFromReflect,
        ReflectSerialize, Vec3,
    },
    reflect::Reflect,
    transform::components::Transform,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod component_editor;
pub mod deserialize;
pub mod editable;
pub mod generate_tangents;
pub mod lifecycle;
pub mod plugin;
pub mod serialize;
pub use editable::*;

/// Main camera
#[derive(Reflect, Serialize, Deserialize, Debug, Clone, Component, Default, PartialEq)]
#[reflect(Component, Serialize, Deserialize, Default, FromReflect)]
pub struct MainCamera;

#[derive(Reflect, Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[reflect(Serialize, Deserialize, FromReflect)]
pub enum SaveSettings {
    PreserveDiskFull,
    PreserveDiskTransform,
    #[default]
    Runtime,
}

/// Tracks the source/origin of an entity
/// String is relative path from /assets
#[derive(Reflect, Serialize, Deserialize, Debug, Clone, Component, Default, PartialEq)]
#[reflect(Component, Serialize, Deserialize, Default, FromReflect)]
pub struct SpawnSource(Cow<'static, str>, SaveSettings);
impl SpawnSource {
    pub fn new(path: impl Into<Cow<'static, str>>, spawn_as: SaveSettings) -> Self {
        Self(path.into(), spawn_as)
    }

    pub fn str_ref(&self) -> &str {
        self.0.as_ref()
    }

    pub fn save_settings_ref(&self) -> &SaveSettings {
        &self.1
    }
}

impl core::ops::Deref for SpawnSource {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

/// Camera for UI Editor Elements
#[derive(Reflect, Serialize, Deserialize, Debug, Clone, Component, Default, PartialEq)]
#[reflect(Component, Serialize, Deserialize, Default, FromReflect)]
pub struct UICamera;

/// Tag entity with this to hide in node tree editor
#[derive(Component, PartialEq, Eq)]
pub struct TreeHiddenEntity;

/// Internal note for editor use
#[derive(Reflect, Serialize, Deserialize, Debug, Clone, Component, Default, PartialEq)]
#[reflect(Component, Serialize, Deserialize, Default, FromReflect)]
pub struct InternalNote(pub String);

// --------------------------------------------------------------------------------------------

//
// Actual structure of serialized entity data
// Serialized data contains three main parts:
// 1. IdentityData - Contains the name, uuid, and class type of the entity.
// 2. TransformData - Contains the position, rotation, and scale of the entity.
// 3. Reflected components - Contains any additional components that are reflected and serialized.
// 4. Parent - Contains the parent entity UUID if this entity is a child of another entity.

/// Actual Saved Identity data
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[reflect(Component)]
pub struct IdentityData {
    pub uuid: Uuid,
    pub name: String,
    pub class: GraniteTypes,
}

/// Actual Saved GlobalTransform data
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[reflect(Component)]
pub struct TransformData {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl TransformData {
    pub fn to_bevy(&self) -> Transform {
        Transform {
            translation: self.position,
            rotation: self.rotation,
            scale: self.scale,
        }
    }
}

// Actual component data is gathered from the ComponentEditor and serialized using bevy reflect

// --------------------------------------------------------------------------------------------

// Main component to flag entities that have granite editor components
#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct HasRuntimeData;

// If create material on import true, where should that material name come from?
#[derive(Serialize, Deserialize, Clone, Copy, Default, PartialEq, Debug)]
pub enum MaterialNameSource {
    FileName,
    #[default]
    FileContents,
    DefaultMaterial,
    SaveData,
}
impl MaterialNameSource {
    pub fn ui_selectable() -> Vec<Self> {
        vec![
            MaterialNameSource::FileName,
            MaterialNameSource::FileContents,
            MaterialNameSource::DefaultMaterial,
        ]
    }
}

// We send this with the prompt for additional settings on objects that need disk path
#[derive(Debug, Clone, Resource, Serialize, Deserialize, PartialEq)]
pub struct PromptImportSettings {
    pub create_mat_on_import: bool,
    pub material_name_source: MaterialNameSource,
}
impl Default for PromptImportSettings {
    fn default() -> Self {
        Self {
            create_mat_on_import: true,
            material_name_source: MaterialNameSource::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PromptData {
    pub file: Option<String>,
    pub import_settings: PromptImportSettings,
}
impl Default for PromptData {
    fn default() -> Self {
        Self {
            file: None,
            import_settings: PromptImportSettings::default(),
        }
    }
}

// Re-exports
pub use component_editor::{
    is_bridge_component_check, BridgeTag, ComponentEditor, ExposedToEditor, ReflectedComponent,
};
pub use deserialize::{deserialize_entities, GraniteEditorSerdeEntity};
pub use editable::{
    Camera3D, DirLight, Empty, GraniteTypes, PointLightData, RectBrush, VolumetricFog, OBJ,
};
pub use generate_tangents::{generate_tangents_system, NeedsTangents};
pub use lifecycle::{
    despawn_entities_by_source_system, despawn_entities_system,
    despawn_recursive_serializable_entities,
};
pub use plugin::EntityPlugin;
pub use serialize::{serialize_entities, EntitySaveReadyData, SceneData, SceneMetadata};

// Im adding this so you cant select the editor camera
// and to stop a crash because you can select a gizmo that then despawns its self
bitflags::bitflags! {
    /// A marker component that an entity should be ignored by the editor
    /// This will be more powerful then not having Bridge
    /// As this is explicitly added to an entity
    #[derive(bevy::ecs::component::Component, Default)]
    pub struct EditorIgnore: usize {
        const GIZMO = 1;
        const PICKING = 2;
        const SERIALIZE = 3;
    }
}

/// Component that marks an entity as owning editable topology data.
///
/// This component indicates that the entity is the authoritative source for
/// mesh topology during edit mode. The topology data is separate from Bevy
/// render meshes and serves as the source of truth for edit operations.
///
/// # Ownership Contract
///
/// - Entities with `TopologyOwner` are the source of truth for their mesh topology
/// - The actual topology data is stored in a separate `TopologyHandle` resource
/// - Render meshes are derived from or synchronized with this topology
/// - Edit operations should modify topology through the owner, not the render mesh
#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct TopologyOwner {
    /// Unique identifier for this topology instance
    pub topology_id: TopologyId,
}

/// Unique identifier for an editable topology instance.
///
/// This ID is used to lookup the actual topology data stored in a resource.
/// It is independent of Bevy entity IDs and remains stable across topology
/// modifications.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TopologyId(u64);

impl TopologyId {
    /// Create a new topology ID from a u64 value.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Create a dummy/invalid topology ID.
    pub fn dummy() -> Self {
        Self(u64::MAX)
    }

    /// Get the underlying u64 value.
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Check if this is a dummy/invalid ID.
    pub fn is_dummy(&self) -> bool {
        self.0 == u64::MAX
    }
}

impl Default for TopologyId {
    fn default() -> Self {
        Self::dummy()
    }
}

/// Resource that stores editable topology data for all entities with topology owners.
///
/// This resource acts as a registry for topology data, mapping topology IDs to
/// their corresponding `EditableTopology` instances. It is separate from Bevy
/// entities to allow topology data to persist independently of entity lifecycle.
///
/// # Usage Pattern
///
/// 1. When an entity gets a `TopologyOwner`, a new `EditableTopology` is created
/// 2. The topology is stored in this resource with a unique ID
/// 3. Edit operations query this resource to get/modify topology
/// 4. Render meshes can be derived from or synchronized with topology data
#[derive(Resource, Clone, Debug)]
pub struct EditableTopologyRegistry {
    next_id: u64,
    topologies: HashMap<TopologyId, EditableTopology>,
}

impl EditableTopologyRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            next_id: 0,
            topologies: HashMap::new(),
        }
    }

    /// Insert a new topology and return its ID.
    pub fn insert(&mut self, topology: EditableTopology) -> TopologyId {
        let id = TopologyId(self.next_id);
        self.next_id += 1;
        self.topologies.insert(id, topology);
        id
    }

    /// Get a reference to a topology by ID.
    pub fn get(&self, id: TopologyId) -> Option<&EditableTopology> {
        self.topologies.get(&id)
    }

    /// Get a mutable reference to a topology by ID.
    pub fn get_mut(&mut self, id: TopologyId) -> Option<&mut EditableTopology> {
        self.topologies.get_mut(&id)
    }

    /// Remove a topology by ID and return it if it existed.
    pub fn remove(&mut self, id: TopologyId) -> Option<EditableTopology> {
        self.topologies.remove(&id)
    }

    /// Check if a topology exists for the given ID.
    pub fn contains(&self, id: TopologyId) -> bool {
        self.topologies.contains_key(&id)
    }

    /// Get the number of topologies in the registry.
    pub fn len(&self) -> usize {
        self.topologies.len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.topologies.is_empty()
    }

    /// Get an iterator over all topology IDs.
    pub fn ids(&self) -> impl Iterator<Item = TopologyId> + '_ {
        self.topologies.keys().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topology_id_new() {
        let id = TopologyId::new(42);
        assert_eq!(id.value(), 42);
        assert!(!id.is_dummy());
    }

    #[test]
    fn test_topology_id_dummy() {
        let id = TopologyId::dummy();
        assert!(id.is_dummy());
        assert_eq!(id.value(), u64::MAX);
    }

    #[test]
    fn test_topology_owner_default() {
        let owner = TopologyOwner::default();
        assert!(owner.topology_id.is_dummy());
    }

    #[test]
    fn test_topology_registry_new() {
        let registry = EditableTopologyRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_topology_registry_insert_and_get() {
        let mut registry = EditableTopologyRegistry::new();
        let topology = EditableTopology::new();
        let id = registry.insert(topology.clone());

        assert!(registry.contains(id));
        assert_eq!(registry.len(), 1);
        assert!(!registry.is_empty());

        let retrieved = registry.get(id).expect("Topology should exist");
        assert_eq!(retrieved, &topology);
    }

    #[test]
    fn test_topology_registry_get_mut() {
        let mut registry = EditableTopologyRegistry::new();
        let id = registry.insert(EditableTopology::new());

        let topology = registry.get_mut(id).expect("Topology should exist");
        assert!(topology.is_valid());
    }

    #[test]
    fn test_topology_registry_remove() {
        let mut registry = EditableTopologyRegistry::new();
        let topology = EditableTopology::new();
        let id = registry.insert(topology.clone());

        let removed = registry.remove(id).expect("Should remove topology");
        assert_eq!(removed, topology);
        assert!(!registry.contains(id));
        assert!(registry.is_empty());
    }

    #[test]
    fn test_topology_registry_remove_nonexistent() {
        let mut registry = EditableTopologyRegistry::new();
        let id = TopologyId::new(999);

        let removed = registry.remove(id);
        assert!(removed.is_none());
        assert!(registry.is_empty());
    }

    #[test]
    fn test_topology_registry_ids_iterator() {
        let mut registry = EditableTopologyRegistry::new();
        let id1 = registry.insert(EditableTopology::new());
        let id2 = registry.insert(EditableTopology::new());
        let id3 = registry.insert(EditableTopology::new());

        let ids: Vec<TopologyId> = registry.ids().collect();
        assert_eq!(ids.len(), 3);
        assert!(ids.contains(&id1));
        assert!(ids.contains(&id2));
        assert!(ids.contains(&id3));
    }

    #[test]
    fn test_topology_registry_multiple_operations() {
        let mut registry = EditableTopologyRegistry::new();

        let id1 = registry.insert(EditableTopology::new());
        let id2 = registry.insert(EditableTopology::new());
        let id3 = registry.insert(EditableTopology::new());

        assert_eq!(registry.len(), 3);

        assert!(registry.get(id1).is_some());
        assert!(registry.get(id2).is_some());
        assert!(registry.get(id3).is_some());

        registry.remove(id2).expect("Should remove id2");
        assert_eq!(registry.len(), 2);
        assert!(registry.get(id2).is_none());
        assert!(registry.get(id1).is_some());
        assert!(registry.get(id3).is_some());

        let id4 = registry.insert(EditableTopology::new());
        assert_eq!(id4.value(), 3);
    }
}
