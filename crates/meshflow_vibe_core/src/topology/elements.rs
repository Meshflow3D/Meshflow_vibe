use super::ids::{EdgeId, FaceId, LoopId, VertId};
use bevy::prelude::Vec3;
use serde::{Deserialize, Serialize};

/// A vertex in the editable topology.
///
/// Vertices store 3D positions independently of Bevy render meshes.
/// This is the source of truth for vertex positions during editing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vert {
    /// Unique identifier for this vertex
    pub id: VertId,
    /// 3D position of the vertex
    pub position: Vec3,
}

impl Vert {
    pub fn new(id: VertId, position: Vec3) -> Self {
        Self { id, position }
    }

    pub fn dummy() -> Self {
        Self {
            id: VertId::dummy(),
            position: Vec3::ZERO,
        }
    }
}

/// An edge in the editable topology.
///
/// Edges connect two vertices and can be shared by multiple face loops,
/// enabling representation of non-manifold geometry. In manifold geometry,
/// each edge is shared by exactly two faces. In non-manifold cases (e.g.,
/// a crease edge where 3+ faces meet), an edge can have more than two loop ends.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    /// Unique identifier for this edge
    pub id: EdgeId,
    /// Loop IDs for face corners that use this edge.
    ///
    /// In manifold geometry, this contains 0, 1, or 2 loop IDs.
    /// In non-manifold geometry, this can contain 3+ loop IDs (e.g., a crease
    /// where 3+ faces meet at the same edge).
    ///
    /// The order of loop IDs is not guaranteed to be consistent. Use
    /// `face_loops_for_edge()` on EditableTopology to discover all loops
    /// referencing this edge.
    pub loop_ends: Vec<Option<LoopId>>,
}

impl Edge {
    pub fn new(id: EdgeId) -> Self {
        Self {
            id,
            loop_ends: Vec::new(),
        }
    }

    pub fn dummy() -> Self {
        Self {
            id: EdgeId::dummy(),
            loop_ends: Vec::new(),
        }
    }

    /// Add a loop end to this edge
    pub fn add_loop_end(&mut self, loop_id: LoopId) {
        self.loop_ends.push(Some(loop_id));
    }

    /// Remove a loop end at a specific index if it exists
    pub fn remove_loop_end(&mut self, index: usize) {
        if index < self.loop_ends.len() {
            self.loop_ends[index] = None;
        }
    }

    /// Get the loop end at a specific index
    pub fn loop_end(&self, index: usize) -> Option<LoopId> {
        self.loop_ends.get(index).copied().flatten()
    }

    /// Get all valid loop ends for this edge
    pub fn valid_loop_ends(&self) -> Vec<LoopId> {
        self.loop_ends.iter().filter_map(|&x| x).collect()
    }

    /// Check if this edge has exactly one valid loop end (boundary edge)
    pub fn is_boundary(&self) -> bool {
        self.valid_loop_ends().len() == 1
    }

    /// Check if this edge has exactly two valid loop ends (manifold interior edge)
    pub fn is_manifold_interior(&self) -> bool {
        self.valid_loop_ends().len() == 2
    }

    /// Check if this edge has more than two valid loop ends (non-manifold edge)
    pub fn is_non_manifold(&self) -> bool {
        self.valid_loop_ends().len() > 2
    }

    /// Get the number of faces this edge is shared by
    pub fn face_count(&self) -> usize {
        self.valid_loop_ends().len()
    }

    /// Reserve capacity for loop ends
    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.loop_ends.reserve(capacity);
        self
    }
}

/// A loop (face corner) in the editable topology.
///
/// Loops connect a face to an edge and vertex, forming the per-face-corner
/// element. Each loop belongs to exactly one face and references exactly one edge.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Loop {
    /// Unique identifier for this loop
    pub id: LoopId,
    /// The face this loop belongs to
    pub face_id: FaceId,
    /// The edge this loop references
    pub edge_id: EdgeId,
    /// The vertex this loop references
    pub vert_id: VertId,
    /// Index of this loop within its face (0-based, in winding order)
    pub index_in_face: u32,
}

impl Loop {
    pub fn new(
        id: LoopId,
        face_id: FaceId,
        edge_id: EdgeId,
        vert_id: VertId,
        index_in_face: u32,
    ) -> Self {
        Self {
            id,
            face_id,
            edge_id,
            vert_id,
            index_in_face,
        }
    }

    pub fn dummy() -> Self {
        Self {
            id: LoopId::dummy(),
            face_id: FaceId::dummy(),
            edge_id: EdgeId::dummy(),
            vert_id: VertId::dummy(),
            index_in_face: u32::MAX,
        }
    }
}

/// A face in the editable topology.
///
/// Faces are polygonal regions bounded by an ordered sequence of loops.
/// The loop order defines the face winding (typically counter-clockwise).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Face {
    /// Unique identifier for this face
    pub id: FaceId,
    /// Ordered sequence of loop IDs forming the face boundary
    ///
    /// The order defines the face winding. For a triangle, there are 3 loops.
    /// For a quad, there are 4 loops, etc.
    pub loops: Vec<LoopId>,
}

impl Face {
    pub fn new(id: FaceId) -> Self {
        Self {
            id,
            loops: Vec::new(),
        }
    }

    pub fn dummy() -> Self {
        Self {
            id: FaceId::dummy(),
            loops: Vec::new(),
        }
    }

    /// Add a loop to this face
    pub fn add_loop(&mut self, _loop_id: LoopId) {
        self.loops.push(_loop_id);
    }

    /// Get the number of loops (corners) in this face
    pub fn loop_count(&self) -> usize {
        self.loops.len()
    }

    /// Get the loop at a specific index
    pub fn loop_at(&self, index: usize) -> Option<LoopId> {
        self.loops.get(index).copied()
    }

    /// Check if this face is a triangle
    pub fn is_triangle(&self) -> bool {
        self.loops.len() == 3
    }

    /// Check if this face is a quad
    pub fn is_quad(&self) -> bool {
        self.loops.len() == 4
    }
}
