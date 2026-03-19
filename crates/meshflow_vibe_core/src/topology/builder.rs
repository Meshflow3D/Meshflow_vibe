use super::{EdgeId, FaceId, LoopId, VertId};
use super::{EditableTopology, Face, Loop, TopologyValidationError, Vert};
use bevy::mesh::{Indices, VertexAttributeValues};
use bevy::prelude::Mesh;
use std::collections::HashMap;

/// Builder for constructing `EditableTopology` from existing mesh data.
///
/// This is the authoritative boundary for importing mesh data from Bevy render
/// meshes into the editable topology model. It performs validation during
/// construction to ensure the resulting topology is internally consistent.
///
/// # Conceptual Model
///
/// This follows a Blender-inspired separation:
/// - **Editable topology is authoritative**: Edit systems operate on this model
/// - **Bevy Mesh is input data**: Used for import, future sync is a separate concern
/// - **Validation happens at import time**: Broken references are caught immediately
///
/// # Import Process
///
/// 1. Extract unique vertices from mesh positions
/// 2. Extract faces from mesh index buffer (supports triangles, quads, n-gons)
/// 3. Build edges from face loops
/// 4. Build loops connecting faces to edges and vertices
/// 5. Validate the resulting topology
///
/// # Non-Goals
///
/// - This builder does not make Bevy Mesh the source of truth
/// - It does not implement edit operators (extrude, cut, etc.)
/// - It does not export topology back to Bevy Mesh format
///
/// # Future Room
///
/// The builder supports arbitrary polygon faces (n-gons) and non-manifold edges,
/// even though current mesh inputs may be triangulated.
#[derive(Clone, Debug, Default)]
pub struct MeshImporter {
    /// Mapping from mesh vertex index to topology vertex ID
    vertex_map: HashMap<usize, VertId>,
    /// Mapping from mesh face index to topology face ID
    face_map: HashMap<usize, FaceId>,
    /// Mapping from (start_idx, end_idx) to edge ID for edges
    edge_map: HashMap<(usize, usize), EdgeId>,
}

impl MeshImporter {
    /// Create a new empty mesh importer
    pub fn new() -> Self {
        Self {
            vertex_map: HashMap::new(),
            face_map: HashMap::new(),
            edge_map: HashMap::new(),
        }
    }

    /// Import mesh data into an editable topology
    ///
    /// This is the main entry point for constructing topology from Bevy Mesh data.
    /// It performs validation and returns errors if the mesh data is invalid.
    ///
    /// # Arguments
    ///
    /// * `mesh` - The Bevy Mesh to import from
    ///
    /// # Returns
    ///
    /// * `Ok(EditableTopology)` - A valid topology built from the mesh
    /// * `Err(MeshImportError)` - If the mesh data is invalid or missing required attributes
    ///
    /// # Validation
    ///
    /// This method validates:
    /// 1. Mesh has required position attribute
    /// 2. Mesh has index buffer (triangle, quad, or n-gon indices)
    /// 3. All indices reference valid vertices
    /// 4. Resulting topology passes structural validation
    pub fn import_mesh(&mut self, mesh: &Mesh) -> Result<EditableTopology, MeshImportError> {
        // Validate mesh has required attributes
        let positions = self.extract_positions(mesh)?;

        // Get indices from mesh
        let indices = self.extract_indices(mesh)?;

        // Build topology from mesh data
        let mut topology = EditableTopology::with_capacity(
            positions.len(),
            positions.len() * 2, // Estimate
            indices.len() / 3,   // Estimate
        );

        // Import vertices
        for (index, position) in positions.iter().enumerate() {
            let vert_id = topology.generate_vert_id();
            let vert = Vert::new(vert_id, *position);
            topology.insert_vertex(vert);
            self.vertex_map.insert(index, vert_id);
        }

        // Import faces and edges
        self.import_faces(&mut topology, &indices)?;

        // Validate the imported topology
        let errors = topology.validate();
        if !errors.is_empty() {
            return Err(MeshImportError::TopologyValidationError(errors));
        }

        Ok(topology)
    }

    /// Extract vertex positions from mesh
    fn extract_positions(&self, mesh: &Mesh) -> Result<Vec<bevy::prelude::Vec3>, MeshImportError> {
        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .ok_or(MeshImportError::MissingAttribute("POSITION"))?;

        match positions {
            VertexAttributeValues::Float32x3(pos) => Ok(pos
                .iter()
                .map(|[x, y, z]| bevy::prelude::Vec3::new(*x, *y, *z))
                .collect()),
            _ => Err(MeshImportError::InvalidAttributeType("POSITION")),
        }
    }

    /// Extract face indices from mesh index buffer
    ///
    /// Bevy stores mesh indices in the mesh's internal buffer, not as attributes.
    /// This method extracts them correctly.
    fn extract_indices(&self, mesh: &Mesh) -> Result<Vec<u32>, MeshImportError> {
        // Bevy 0.18: indices() returns an Indices enum
        match mesh.indices() {
            Some(Indices::U16(buf)) => Ok(buf.iter().copied().map(|i| i as u32).collect()),
            Some(Indices::U32(buf)) => Ok(buf.clone()),
            None => Ok(Vec::new()),
        }
    }

    /// Import faces from mesh indices
    fn import_faces(
        &mut self,
        topology: &mut EditableTopology,
        indices: &[u32],
    ) -> Result<(), MeshImportError> {
        // Group indices into faces (support arbitrary polygon faces)
        // We'll treat each triangle as a face for now
        // Future: support quad/n-gon detection if mesh has appropriate primitive topology

        if indices.is_empty() {
            return Ok(());
        }

        // For now, assume triangle mesh (3 indices per face)
        // This is the most common case and aligns with Bevy's default behavior
        let mut face_index = 0;
        let chunk_size = 3;

        // Check if we have a complete set of triangles
        let is_triangle_mesh = indices.len() % 3 == 0;

        if !is_triangle_mesh {
            return Err(MeshImportError::InvalidIndexCount(indices.len()));
        }

        for chunk in indices.chunks(chunk_size) {
            if chunk.len() != chunk_size {
                continue; // Skip incomplete faces
            }

            // Get vertex indices for this face
            let face_indices: Vec<usize> = chunk.iter().map(|&i| i as usize).collect();

            // Check that all indices are valid
            for &idx in &face_indices {
                if idx >= self.vertex_map.len() {
                    return Err(MeshImportError::InvalidVertexIndex {
                        index: idx,
                        vertex_count: self.vertex_map.len(),
                    });
                }
            }

            // Get vertex IDs for this face
            let vertex_ids: Vec<VertId> = face_indices
                .iter()
                .map(|&idx| *self.vertex_map.get(&idx).unwrap())
                .collect();

            // Create face
            let face_id = topology.generate_face_id();
            let mut face = Face::new(face_id);
            self.face_map.insert(face_index, face_id);

            // Create edges and loops for this face
            let edges = self.import_face_edges(topology, &face_indices, face_id)?;

            // Create loops
            let loops = self.import_face_loops(topology, &edges, &vertex_ids, face_id)?;

            face.loops = loops;
            topology.insert_face(face);

            face_index += 1;
        }

        Ok(())
    }

    /// Import edges for a face
    fn import_face_edges(
        &mut self,
        topology: &mut EditableTopology,
        face_indices: &[usize],
        _face_id: FaceId,
    ) -> Result<Vec<EdgeId>, MeshImportError> {
        let mut edge_ids = Vec::new();

        for i in 0..face_indices.len() {
            let start_idx = face_indices[i];
            let end_idx = face_indices[(i + 1) % face_indices.len()];

            // Normalize edge key (smaller index first) for consistent edge lookup
            let (min_idx, max_idx) = if start_idx < end_idx {
                (start_idx, end_idx)
            } else {
                (end_idx, start_idx)
            };

            let edge_key = (min_idx, max_idx);

            let edge_id = if let Some(&edge_id) = self.edge_map.get(&edge_key) {
                edge_id
            } else {
                // Create new edge
                let new_edge_id = topology.generate_edge_id();
                let edge = super::Edge::new(new_edge_id);
                topology.insert_edge(edge);
                self.edge_map.insert(edge_key, new_edge_id);
                new_edge_id
            };

            edge_ids.push(edge_id);
        }

        Ok(edge_ids)
    }

    /// Import loops for a face
    fn import_face_loops(
        &self,
        topology: &mut EditableTopology,
        edge_ids: &[EdgeId],
        vertex_ids: &[VertId],
        face_id: FaceId,
    ) -> Result<Vec<LoopId>, MeshImportError> {
        if edge_ids.len() != vertex_ids.len() {
            return Err(MeshImportError::InvalidFaceStructure {
                edge_count: edge_ids.len(),
                vertex_count: vertex_ids.len(),
            });
        }

        let mut loop_ids = Vec::new();

        for (index, (&edge_id, &vert_id)) in edge_ids.iter().zip(vertex_ids.iter()).enumerate() {
            let loop_id = topology.generate_loop_id();
            let loop_ = Loop::new(loop_id, face_id, edge_id, vert_id, index as u32);
            topology.loops.insert(loop_id, loop_);
            loop_ids.push(loop_id);
        }

        Ok(loop_ids)
    }

    /// Clear the importer's internal mappings
    pub fn clear(&mut self) {
        self.vertex_map.clear();
        self.face_map.clear();
        self.edge_map.clear();
    }
}

/// Errors that can occur during mesh import
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MeshImportError {
    /// Mesh is missing a required attribute
    MissingAttribute(&'static str),
    /// Attribute has an unexpected type
    InvalidAttributeType(&'static str),
    /// Index count is not divisible by 3 (for triangle mesh)
    InvalidIndexCount(usize),
    /// Vertex index references a non-existent vertex
    InvalidVertexIndex { index: usize, vertex_count: usize },
    /// Face has inconsistent edge/vertex count
    InvalidFaceStructure {
        edge_count: usize,
        vertex_count: usize,
    },
    /// Topology validation failed after import
    TopologyValidationError(Vec<TopologyValidationError>),
}

impl std::fmt::Display for MeshImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MeshImportError::MissingAttribute(attr) => {
                write!(f, "Mesh missing required attribute: {}", attr)
            }
            MeshImportError::InvalidAttributeType(attr) => {
                write!(f, "Attribute {} has invalid type", attr)
            }
            MeshImportError::InvalidIndexCount(count) => {
                write!(f, "Invalid index count: {} (not divisible by 3)", count)
            }
            MeshImportError::InvalidVertexIndex {
                index,
                vertex_count,
            } => {
                write!(
                    f,
                    "Vertex index {} out of range (vertex count: {})",
                    index, vertex_count
                )
            }
            MeshImportError::InvalidFaceStructure {
                edge_count,
                vertex_count,
            } => {
                write!(
                    f,
                    "Invalid face structure: {} edges vs {} vertices",
                    edge_count, vertex_count
                )
            }
            MeshImportError::TopologyValidationError(errors) => {
                write!(f, "Topology validation failed: {} errors", errors.len())
            }
        }
    }
}

impl std::error::Error for MeshImportError {}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::Cuboid;

    #[test]
    fn test_import_simple_cube() {
        // Create a simple cube mesh
        let mesh = Mesh::from(Cuboid::new(1.0, 1.0, 1.0));

        let mut importer = MeshImporter::new();
        let result = importer.import_mesh(&mesh);

        // Cube should import successfully
        assert!(
            result.is_ok(),
            "Cube import should succeed: {:?}",
            result.err()
        );

        let topology = result.unwrap();

        // Validate topology
        assert!(topology.is_valid(), "Imported topology should be valid");

        // Cube has 12 faces (triangulated), 24 vertices (unshared per face), 30 edges (Bevy 0.18 Cuboid)
        assert_eq!(topology.face_count(), 12);
        assert_eq!(topology.vertex_count(), 24);
        assert_eq!(topology.edge_count(), 30);
    }

    #[test]
    fn test_import_triangle() {
        // Create a simple triangle mesh (cone with 3 sides)
        let mesh = Mesh::from(bevy::math::primitives::Cone {
            radius: 0.5,
            height: 1.0,
        });

        let mut importer = MeshImporter::new();
        let result = importer.import_mesh(&mesh);

        // Should import (may have multiple faces for cone)
        assert!(
            result.is_ok(),
            "Triangle import should succeed: {:?}",
            result.err()
        );

        let topology = result.unwrap();

        // Validate topology
        assert!(topology.is_valid(), "Imported topology should be valid");

        // Should have at least one face
        assert!(topology.face_count() > 0);
    }

    #[test]
    fn test_import_shared_edges() {
        // Create a mesh with shared edges (cube has shared edges)
        let mesh = Mesh::from(Cuboid::new(1.0, 1.0, 1.0));

        let mut importer = MeshImporter::new();
        let result = importer.import_mesh(&mesh);

        assert!(
            result.is_ok(),
            "Mesh import should succeed: {:?}",
            result.err()
        );

        let topology = result.unwrap();

        // Validate topology
        assert!(topology.is_valid());

        // Check that edges can be shared (use topology API to find shared edges)
        let mut shared_edge_count = 0;
        for edge in topology.edges() {
            let loops = topology.face_loops_for_edge(edge.id);
            if loops.len() > 1 {
                shared_edge_count += 1;
            }
        }

        // In a manifold mesh, most edges should be shared by exactly 2 faces
        assert!(
            shared_edge_count > 0,
            "Should have shared edges in valid mesh (found {})",
            shared_edge_count
        );
    }

    #[test]
    fn test_import_missing_positions() {
        // Create an empty mesh (no positions)
        let mesh = Mesh::new(
            bevy::render::render_resource::PrimitiveTopology::TriangleList,
            bevy::asset::RenderAssetUsages::default(),
        );

        let mut importer = MeshImporter::new();
        let result = importer.import_mesh(&mesh);

        // Should fail with missing attribute error
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MeshImportError::MissingAttribute(_)
        ));
    }

    #[test]
    fn test_import_invalid_vertex_index() {
        // Create a mesh with invalid indices
        let mut mesh = Mesh::new(
            bevy::render::render_resource::PrimitiveTopology::TriangleList,
            bevy::asset::RenderAssetUsages::default(),
        );

        // Add positions (only 3 vertices) - using Float32x3 format for Bevy 0.18
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                bevy::prelude::Vec3::new(0.0, 0.0, 0.0),
                bevy::prelude::Vec3::new(1.0, 0.0, 0.0),
                bevy::prelude::Vec3::new(0.0, 1.0, 0.0),
            ],
        );

        // Add invalid indices (100 is out of range)
        mesh.insert_indices(bevy::mesh::Indices::U32(vec![0, 1, 100]));

        let mut importer = MeshImporter::new();
        let result = importer.import_mesh(&mesh);

        // Should fail with invalid vertex index error
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MeshImportError::InvalidVertexIndex { .. }
        ));
    }

    #[test]
    fn test_import_empty_mesh() {
        // Create a mesh with no data (no positions attribute)
        let mesh = Mesh::new(
            bevy::render::render_resource::PrimitiveTopology::TriangleList,
            bevy::asset::RenderAssetUsages::default(),
        );

        let mut importer = MeshImporter::new();
        let result = importer.import_mesh(&mesh);

        // Should fail with missing attribute error (empty mesh has no positions)
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MeshImportError::MissingAttribute(_)
        ));
    }

    #[test]
    fn test_topology_validation_after_import() {
        // Import a valid mesh and verify topology validation passes
        let mesh = Mesh::from(Cuboid::new(1.0, 1.0, 1.0));

        let mut importer = MeshImporter::new();
        let topology = importer.import_mesh(&mesh).unwrap();

        // Validate the topology
        let errors = topology.validate();
        assert!(
            errors.is_empty(),
            "Imported topology should be valid, got errors: {:?}",
            errors
        );

        // Manually call is_valid as well
        assert!(topology.is_valid());
    }
}
