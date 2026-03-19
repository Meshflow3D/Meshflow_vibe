use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Stable ID for a vertex in the topology.
///
/// This ID is independent of Bevy entity IDs and remains stable across
/// topology modifications. The source of truth for vertex positions
/// lives separately from render meshes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct VertId(Uuid);

impl VertId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn dummy() -> Self {
        Self(Uuid::nil())
    }

    pub fn uuid(&self) -> Uuid {
        self.0
    }

    pub fn is_dummy(&self) -> bool {
        self.0.is_nil()
    }
}

impl fmt::Display for VertId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}", self.0.simple())
    }
}

/// Stable ID for an edge in the topology.
///
/// Edges represent 1D connections between vertices and can be shared
/// by multiple faces (loops), enabling non-manifold topology support.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct EdgeId(Uuid);

impl EdgeId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn dummy() -> Self {
        Self(Uuid::nil())
    }

    pub fn uuid(&self) -> Uuid {
        self.0
    }

    pub fn is_dummy(&self) -> bool {
        self.0.is_nil()
    }
}

impl fmt::Display for EdgeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "e{}", self.0.simple())
    }
}

/// Stable ID for a face in the topology.
///
/// Faces are polygonal regions bounded by an ordered sequence of loops.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct FaceId(Uuid);

impl FaceId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn dummy() -> Self {
        Self(Uuid::nil())
    }

    pub fn uuid(&self) -> Uuid {
        self.0
    }

    pub fn is_dummy(&self) -> bool {
        self.0.is_nil()
    }
}

impl fmt::Display for FaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "f{}", self.0.simple())
    }
}

/// Stable ID for a loop (face corner).
///
/// Loops connect a face to an edge and vertex, forming the per-face-corner
/// element that enables edge-to-face adjacency tracking. Each loop belongs
/// to exactly one face and references exactly one edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct LoopId(u32);

impl LoopId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn dummy() -> Self {
        Self(u32::MAX)
    }

    pub fn index(&self) -> u32 {
        self.0
    }

    /// Create a new loop ID from an index. Used for iteration and indexing.
    pub fn from_index(idx: u32) -> Self {
        Self(idx)
    }
}

impl fmt::Display for LoopId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "l{}", self.0)
    }
}
