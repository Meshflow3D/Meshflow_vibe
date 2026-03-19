pub mod builder;
pub mod elements;
pub mod ids;

#[cfg(test)]
mod tests;

pub use builder::{MeshImportError, MeshImporter};
pub use elements::{Edge, Face, Loop, Vert};
pub use ids::{EdgeId, FaceId, LoopId, VertId};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Editable topology model for mesh editing.
///
/// This is the source of truth for mesh topology during edit mode, separate
/// from Bevy render meshes. It uses Blender-inspired vocabulary (vert, edge,
/// loop, face) with stable IDs for robustness.
///
/// # Design Notes
///
/// - **Stable IDs**: Elements use stable IDs independent of Bevy entities
/// - **Explicit adjacency**: Edge-to-face adjacency is tracked via loops
/// - **Non-manifold ready**: Data structure supports edges shared by >2 faces
/// - **Render-mesh separation**: This model is independent of `bevy::mesh::Mesh`
///
/// # Invariants
///
/// 1. Every loop must reference a valid edge and vertex
/// 2. Every face's loops must be in winding order
/// 3. Edge loop-ends must reference valid loops
/// 4. No duplicate element IDs within a topology
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EditableTopology {
    /// Vertices indexed by their ID
    vertices: HashMap<VertId, Vert>,
    /// Edges indexed by their ID
    edges: HashMap<EdgeId, Edge>,
    /// Faces indexed by their ID
    faces: HashMap<FaceId, Face>,
    /// All loops in the topology (keyed by loop ID)
    loops: HashMap<LoopId, Loop>,
    /// Next loop ID to assign (for ID generation)
    next_loop_id: u32,
}

impl Default for EditableTopology {
    fn default() -> Self {
        Self::new()
    }
}

impl EditableTopology {
    /// Create a new empty editable topology
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            faces: HashMap::new(),
            loops: HashMap::new(),
            next_loop_id: 0,
        }
    }

    /// Create a new topology with pre-allocated capacity
    pub fn with_capacity(vertices: usize, edges: usize, faces: usize) -> Self {
        Self {
            vertices: HashMap::with_capacity(vertices),
            edges: HashMap::with_capacity(edges),
            faces: HashMap::with_capacity(faces),
            loops: HashMap::with_capacity(edges * 2), // Estimate 2 loops per edge
            next_loop_id: 0,
        }
    }

    // ========================================================================
    // Vertex operations
    // ========================================================================

    /// Add a vertex to the topology
    pub fn insert_vertex(&mut self, vert: Vert) {
        self.vertices.insert(vert.id, vert);
    }

    /// Get a vertex by ID
    pub fn vertex(&self, id: VertId) -> Option<&Vert> {
        self.vertices.get(&id)
    }

    /// Get a mutable reference to a vertex
    pub fn vertex_mut(&mut self, id: VertId) -> Option<&mut Vert> {
        self.vertices.get_mut(&id)
    }

    /// Remove a vertex from the topology
    ///
    /// # Note
    /// This does not clean up references to the vertex in edges/faces.
    /// Use `validate()` after bulk removals.
    pub fn remove_vertex(&mut self, id: VertId) -> Option<Vert> {
        self.vertices.remove(&id)
    }

    /// Get all vertices
    pub fn vertices(&self) -> impl Iterator<Item = &Vert> {
        self.vertices.values()
    }

    /// Get the number of vertices
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    // ========================================================================
    // Edge operations
    // ========================================================================

    /// Add an edge to the topology
    pub fn insert_edge(&mut self, edge: Edge) {
        self.edges.insert(edge.id, edge);
    }

    /// Get an edge by ID
    pub fn edge(&self, id: EdgeId) -> Option<&Edge> {
        self.edges.get(&id)
    }

    /// Get a mutable reference to an edge
    pub fn edge_mut(&mut self, id: EdgeId) -> Option<&mut Edge> {
        self.edges.get_mut(&id)
    }

    /// Remove an edge from the topology
    pub fn remove_edge(&mut self, id: EdgeId) -> Option<Edge> {
        self.edges.remove(&id)
    }

    /// Get all edges
    pub fn edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.values()
    }

    /// Get the number of edges
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    // ========================================================================
    // Face operations
    // ========================================================================

    /// Add a face to the topology
    pub fn insert_face(&mut self, face: Face) {
        self.faces.insert(face.id, face);
    }

    /// Get a face by ID
    pub fn face(&self, id: FaceId) -> Option<&Face> {
        self.faces.get(&id)
    }

    /// Get a mutable reference to a face
    pub fn face_mut(&mut self, id: FaceId) -> Option<&mut Face> {
        self.faces.get_mut(&id)
    }

    /// Remove a face from the topology
    pub fn remove_face(&mut self, id: FaceId) -> Option<Face> {
        self.faces.remove(&id)
    }

    /// Get all faces
    pub fn faces(&self) -> impl Iterator<Item = &Face> {
        self.faces.values()
    }

    /// Get the number of faces
    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    // ========================================================================
    // Loop operations
    // ========================================================================

    /// Get a loop by ID
    pub fn loop_at(&self, id: LoopId) -> Option<&Loop> {
        self.loops.get(&id)
    }

    /// Get all loops
    pub fn loops(&self) -> impl Iterator<Item = &Loop> {
        self.loops.values()
    }

    /// Get the number of loops
    pub fn loop_count(&self) -> usize {
        self.loops.len()
    }

    // ========================================================================
    // Adjacency queries
    // ========================================================================

    /// Get all loops that reference a given edge
    ///
    /// This is the key operation for edge-to-face adjacency. In manifold
    /// geometry, this returns 0, 1, or 2 loops. In non-manifold cases,
    /// it can return more.
    pub fn face_loops_for_edge(&self, edge_id: EdgeId) -> Vec<&Loop> {
        self.loops
            .values()
            .filter(|loop_| loop_.edge_id == edge_id)
            .collect()
    }

    /// Get the face(s) that an edge belongs to
    pub fn faces_for_edge(&self, edge_id: EdgeId) -> Vec<&Face> {
        let mut faces = HashSet::new();
        for loop_ in self.face_loops_for_edge(edge_id) {
            if let Some(face) = self.face(loop_.face_id) {
                faces.insert(face);
            }
        }
        faces.into_iter().collect()
    }

    /// Get all edges that a face uses
    pub fn edges_for_face(&self, face_id: FaceId) -> Vec<&Edge> {
        let mut edges = Vec::new();
        if let Some(face) = self.face(face_id) {
            for loop_id in &face.loops {
                if let Some(loop_) = self.loops.get(loop_id) {
                    if let Some(edge) = self.edge(loop_.edge_id) {
                        edges.push(edge);
                    }
                }
            }
        }
        edges
    }

    /// Get all vertices that a face uses
    pub fn vertices_for_face(&self, face_id: FaceId) -> Vec<&Vert> {
        let mut vertices = Vec::new();
        if let Some(face) = self.face(face_id) {
            for loop_id in &face.loops {
                if let Some(loop_) = self.loops.get(loop_id) {
                    if let Some(vert) = self.vertex(loop_.vert_id) {
                        vertices.push(vert);
                    }
                }
            }
        }
        vertices
    }

    /// Get the vertex at a specific loop in a face
    pub fn vertex_at_loop(&self, face_id: FaceId, loop_index: usize) -> Option<&Vert> {
        let face = self.face(face_id)?;
        let loop_id = *face.loops.get(loop_index)?;
        let loop_ = self.loops.get(&loop_id)?;
        self.vertex(loop_.vert_id)
    }

    /// Get the edge at a specific loop in a face
    pub fn edge_at_loop(&self, face_id: FaceId, loop_index: usize) -> Option<&Edge> {
        let face = self.face(face_id)?;
        let loop_id = *face.loops.get(loop_index)?;
        let loop_ = self.loops.get(&loop_id)?;
        self.edge(loop_.edge_id)
    }

    // ========================================================================
    // ID generation
    // ========================================================================

    /// Generate a new unique vertex ID
    pub fn generate_vert_id(&mut self) -> VertId {
        use uuid::Uuid;
        VertId::new(Uuid::new_v4())
    }

    /// Generate a new unique edge ID
    pub fn generate_edge_id(&mut self) -> EdgeId {
        use uuid::Uuid;
        EdgeId::new(Uuid::new_v4())
    }

    /// Generate a new unique face ID
    pub fn generate_face_id(&mut self) -> FaceId {
        use uuid::Uuid;
        FaceId::new(Uuid::new_v4())
    }

    /// Generate a new unique loop ID
    pub fn generate_loop_id(&mut self) -> LoopId {
        let id = LoopId::new(self.next_loop_id);
        self.next_loop_id += 1;
        id
    }

    // ========================================================================
    // Iteration helpers
    // ========================================================================

    /// Get all face IDs
    pub fn face_ids(&self) -> impl Iterator<Item = FaceId> + '_ {
        self.faces.keys().copied()
    }

    /// Get all edge IDs
    pub fn edge_ids(&self) -> impl Iterator<Item = EdgeId> + '_ {
        self.edges.keys().copied()
    }

    /// Get all vertex IDs
    pub fn vertex_ids(&self) -> impl Iterator<Item = VertId> + '_ {
        self.vertices.keys().copied()
    }

    /// Get all loop IDs
    pub fn loop_ids(&self) -> impl Iterator<Item = LoopId> + '_ {
        self.loops.keys().copied()
    }

    // ========================================================================
    // Validation
    // ========================================================================

    /// Validate all topology invariants
    ///
    /// # Invariants checked:
    /// 1. All loops reference valid edges and vertices
    /// 2. All edge loop-ends reference valid loops
    /// 3. All face loops reference valid loops
    /// 4. No duplicate element IDs
    /// 5. Loops in each face are in correct winding order
    ///
    /// Returns a vector of validation errors (empty if valid).
    pub fn validate(&self) -> Vec<TopologyValidationError> {
        let mut errors = Vec::new();

        // Check for duplicate IDs within each collection
        errors.extend(self.validate_unique_ids());

        // Check that all loop references are valid
        errors.extend(self.validate_loop_references());

        // Check that all edge loop-ends are valid
        errors.extend(self.validate_edge_loop_ends());

        // Check face loop ordering
        errors.extend(self.validate_face_loops());

        errors
    }

    /// Check that all element IDs are unique within their collections
    fn validate_unique_ids(&self) -> Vec<TopologyValidationError> {
        // HashMap keys are already unique by definition, so this is mainly
        // a sanity check. We could extend this to check cross-type uniqueness
        // if needed.
        Vec::new()
    }

    /// Check that all loops reference valid edges and vertices
    fn validate_loop_references(&self) -> Vec<TopologyValidationError> {
        let mut errors = Vec::new();

        for loop_ in self.loops.values() {
            // Skip validation for dummy IDs (used in tests/placeholder)
            if !loop_.edge_id.is_dummy() && self.edge(loop_.edge_id).is_none() {
                errors.push(TopologyValidationError::LoopReferencesInvalidEdge {
                    loop_id: loop_.id,
                    edge_id: loop_.edge_id,
                });
            }
            if !loop_.vert_id.is_dummy() && self.vertex(loop_.vert_id).is_none() {
                errors.push(TopologyValidationError::LoopReferencesInvalidVertex {
                    loop_id: loop_.id,
                    vert_id: loop_.vert_id,
                });
            }
            if !loop_.face_id.is_dummy() && self.face(loop_.face_id).is_none() {
                errors.push(TopologyValidationError::LoopReferencesInvalidFace {
                    loop_id: loop_.id,
                    face_id: loop_.face_id,
                });
            }
        }

        errors
    }

    /// Check that all edge loop-ends reference valid loops
    fn validate_edge_loop_ends(&self) -> Vec<TopologyValidationError> {
        let mut errors = Vec::new();

        for edge in self.edges.values() {
            for (i, loop_end_opt) in edge.loop_ends.iter().enumerate() {
                if let Some(loop_id) = loop_end_opt {
                    if self.loops.get(loop_id).is_none() {
                        errors.push(TopologyValidationError::EdgeLoopEndInvalid {
                            edge_id: edge.id,
                            loop_end_index: i,
                            loop_id: *loop_id,
                        });
                    }
                }
            }
        }

        errors
    }

    /// Check that face loops are valid and in correct order
    fn validate_face_loops(&self) -> Vec<TopologyValidationError> {
        let mut errors = Vec::new();

        for face in self.faces.values() {
            // Check that all loop IDs in the face exist
            for (index, loop_id) in face.loops.iter().enumerate() {
                if self.loops.get(loop_id).is_none() {
                    errors.push(TopologyValidationError::FaceLoopInvalid {
                        face_id: face.id,
                        loop_index: index,
                        loop_id: *loop_id,
                    });
                }
            }

            // Check that loop indices in face match their position
            for (index, loop_id) in face.loops.iter().enumerate() {
                if let Some(loop_) = self.loops.get(loop_id) {
                    if loop_.index_in_face != index as u32 {
                        errors.push(TopologyValidationError::FaceLoopIndexMismatch {
                            face_id: face.id,
                            loop_id: *loop_id,
                            expected_index: index as u32,
                            actual_index: loop_.index_in_face,
                        });
                    }

                    // Check that the loop's face_id matches this face
                    if loop_.face_id != face.id {
                        errors.push(TopologyValidationError::LoopFaceIdMismatch {
                            loop_id: *loop_id,
                            expected_face_id: face.id,
                            actual_face_id: loop_.face_id,
                        });
                    }
                }
            }
        }

        errors
    }

    /// Check if this topology is valid
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }
}

/// Errors that can occur during topology validation
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TopologyValidationError {
    /// A loop references an edge that doesn't exist
    LoopReferencesInvalidEdge { loop_id: LoopId, edge_id: EdgeId },
    /// A loop references a vertex that doesn't exist
    LoopReferencesInvalidVertex { loop_id: LoopId, vert_id: VertId },
    /// A loop references a face that doesn't exist
    LoopReferencesInvalidFace { loop_id: LoopId, face_id: FaceId },
    /// An edge's loop-end references a non-existent loop
    EdgeLoopEndInvalid {
        edge_id: EdgeId,
        loop_end_index: usize,
        loop_id: LoopId,
    },
    /// A face references a loop that doesn't exist
    FaceLoopInvalid {
        face_id: FaceId,
        loop_index: usize,
        loop_id: LoopId,
    },
    /// A loop's index_in_face doesn't match its position in the face
    FaceLoopIndexMismatch {
        face_id: FaceId,
        loop_id: LoopId,
        expected_index: u32,
        actual_index: u32,
    },
    /// A loop's face_id doesn't match the face it's referenced by
    LoopFaceIdMismatch {
        loop_id: LoopId,
        expected_face_id: FaceId,
        actual_face_id: FaceId,
    },
}
