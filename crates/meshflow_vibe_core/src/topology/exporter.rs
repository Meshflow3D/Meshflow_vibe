use super::{EditableTopology, FaceId, LoopId, TopologyValidationError};
use bevy::mesh::{Indices, Mesh};
use bevy::prelude::Vec3;
use bevy::render::render_resource::PrimitiveTopology;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MeshExportError {
    TopologyValidationError(Vec<TopologyValidationError>),
    NonTriangleFace {
        face_id: FaceId,
        loop_count: usize,
    },
    NonManifoldEdge {
        edge_id: super::EdgeId,
        face_count: usize,
    },
    EmptyTopology,
}

impl std::fmt::Display for MeshExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MeshExportError::TopologyValidationError(errors) => {
                write!(f, "Topology validation failed: {} errors", errors.len())
            }
            MeshExportError::NonTriangleFace {
                face_id,
                loop_count,
            } => {
                write!(
                    f,
                    "Face {:?} has {} loops (only triangles with 3 loops are supported for export)",
                    face_id, loop_count
                )
            }
            MeshExportError::NonManifoldEdge {
                edge_id,
                face_count,
            } => {
                write!(
                    f,
                    "Edge {:?} is non-manifold ({} faces, only 2-manifold meshes are supported)",
                    edge_id, face_count
                )
            }
            MeshExportError::EmptyTopology => {
                write!(f, "Topology is empty (no faces to export)")
            }
        }
    }
}

impl std::error::Error for MeshExportError {}

#[derive(Clone, Debug, Default)]
pub struct MeshExporter;

impl MeshExporter {
    pub fn new() -> Self {
        Self
    }

    pub fn export_mesh(&self, topology: &EditableTopology) -> Result<Mesh, MeshExportError> {
        let errors = topology.validate();
        if !errors.is_empty() {
            return Err(MeshExportError::TopologyValidationError(errors));
        }

        if topology.face_count() == 0 {
            return Err(MeshExportError::EmptyTopology);
        }

        let mut positions: Vec<Vec3> = Vec::with_capacity(topology.loop_count());
        let mut indices: Vec<u32> = Vec::with_capacity(topology.loop_count());

        for face_id in topology.face_ids() {
            let face = topology.face(face_id).unwrap();
            let loop_count = face.loop_count();

            if loop_count != 3 {
                return Err(MeshExportError::NonTriangleFace {
                    face_id,
                    loop_count,
                });
            }

            let base_index = positions.len() as u32;

            for loop_id in &face.loops {
                let loop_ = topology.loop_at(*loop_id).unwrap();
                let vert = topology.vertex(loop_.vert_id).unwrap();
                positions.push(vert.position);
            }

            indices.push(base_index);
            indices.push(base_index + 1);
            indices.push(base_index + 2);
        }

        for edge in topology.edges() {
            let face_count = edge.face_count();
            if face_count > 2 {
                return Err(MeshExportError::NonManifoldEdge {
                    edge_id: edge.id,
                    face_count,
                });
            }
        }

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            bevy::asset::RenderAssetUsages::default(),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_indices(Indices::U32(indices));

        Ok(mesh)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::Vec3;

    fn make_triangle_topology() -> EditableTopology {
        let mut topology = EditableTopology::new();

        let v0_id = topology.generate_vert_id();
        let v1_id = topology.generate_vert_id();
        let v2_id = topology.generate_vert_id();

        topology.insert_vertex(Vert::new(v0_id, Vec3::new(0.0, 0.0, 0.0)));
        topology.insert_vertex(Vert::new(v1_id, Vec3::new(1.0, 0.0, 0.0)));
        topology.insert_vertex(Vert::new(v2_id, Vec3::new(0.5, 1.0, 0.0)));

        let e0_id = topology.generate_edge_id();
        let e1_id = topology.generate_edge_id();
        let e2_id = topology.generate_edge_id();

        topology.insert_edge(Edge::new(e0_id));
        topology.insert_edge(Edge::new(e1_id));
        topology.insert_edge(Edge::new(e2_id));

        let face_id = topology.generate_face_id();

        let l0_id = topology.generate_loop_id();
        let l1_id = topology.generate_loop_id();
        let l2_id = topology.generate_loop_id();

        let loop0 = Loop::new(l0_id, face_id, e0_id, v0_id, 0);
        let loop1 = Loop::new(l1_id, face_id, e1_id, v1_id, 1);
        let loop2 = Loop::new(l2_id, face_id, e2_id, v2_id, 2);

        topology.loops.insert(l0_id, loop0);
        topology.loops.insert(l1_id, loop1);
        topology.loops.insert(l2_id, loop2);

        let mut face = Face::new(face_id);
        face.loops = vec![l0_id, l1_id, l2_id];
        topology.insert_face(face);

        if let Some(e) = topology.edge_mut(e0_id) {
            e.add_loop_end(l0_id);
        }
        if let Some(e) = topology.edge_mut(e1_id) {
            e.add_loop_end(l1_id);
        }
        if let Some(e) = topology.edge_mut(e2_id) {
            e.add_loop_end(l2_id);
        }

        topology
    }

    #[test]
    fn test_export_single_triangle() {
        let topology = make_triangle_topology();
        let exporter = MeshExporter::new();

        let result = exporter.export_mesh(&topology);
        assert!(result.is_ok(), "Export should succeed: {:?}", result.err());

        let mesh = result.unwrap();
        assert_eq!(mesh.primitive_topology(), PrimitiveTopology::TriangleList);

        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("Should have positions");
        assert_eq!(positions.len(), 3);

        let mesh_indices = mesh.indices().expect("Should have indices");
        assert_eq!(mesh_indices.len(), 3);
    }

    #[test]
    fn test_export_empty_topology() {
        let topology = EditableTopology::new();
        let exporter = MeshExporter::new();

        let result = exporter.export_mesh(&topology);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MeshExportError::EmptyTopology
        ));
    }

    #[test]
    fn test_export_non_triangle_face() {
        let mut topology = make_triangle_topology();

        let v3_id = topology.generate_vert_id();
        topology.insert_vertex(Vert::new(v3_id, Vec3::X + Vec3::Y));

        let e3_id = topology.generate_edge_id();
        topology.insert_edge(Edge::new(e3_id));

        let face2_id = topology.generate_face_id();

        let l3_id = topology.generate_loop_id();
        let l4_id = topology.generate_loop_id();
        let l5_id = topology.generate_loop_id();
        let l6_id = topology.generate_loop_id();

        let loop3 = Loop::new(l3_id, face2_id, e3_id, v3_id, 0);
        let loop4 = Loop::new(l4_id, face2_id, e3_id, v3_id, 1);
        let loop5 = Loop::new(l5_id, face2_id, e3_id, v3_id, 2);
        let loop6 = Loop::new(l6_id, face2_id, e3_id, v3_id, 3);

        topology.loops.insert(l3_id, loop3);
        topology.loops.insert(l4_id, loop4);
        topology.loops.insert(l5_id, loop5);
        topology.loops.insert(l6_id, loop6);

        let mut face2 = Face::new(face2_id);
        face2.loops = vec![l3_id, l4_id, l5_id, l6_id];
        topology.insert_face(face2);

        let exporter = MeshExporter::new();
        let result = exporter.export_mesh(&topology);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MeshExportError::NonTriangleFace { .. }
        ));
    }

    #[test]
    fn test_export_non_manifold_edge() {
        let mut topology = make_triangle_topology();

        let v3_id = topology.generate_vert_id();
        topology.insert_vertex(Vert::new(v3_id, Vec3::X + Vec3::Y));

        let e3_id = topology.generate_edge_id();
        topology.insert_edge(Edge::new(e3_id));

        let face2_id = topology.generate_face_id();

        let l3_id = topology.generate_loop_id();
        let l4_id = topology.generate_loop_id();
        let l5_id = topology.generate_loop_id();

        let loop3 = Loop::new(l3_id, face2_id, e3_id, v3_id, 0);
        let loop4 = Loop::new(l4_id, face2_id, e3_id, v3_id, 1);
        let loop5 = Loop::new(l5_id, face2_id, e3_id, v3_id, 2);

        topology.loops.insert(l3_id, loop3);
        topology.loops.insert(l4_id, loop4);
        topology.loops.insert(l5_id, loop5);

        let mut face2 = Face::new(face2_id);
        face2.loops = vec![l3_id, l4_id, l5_id];
        topology.insert_face(face2);

        let exporter = MeshExporter::new();
        let result = exporter.export_mesh(&topology);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MeshExportError::NonManifoldEdge { .. }
        ));
    }

    #[test]
    fn test_roundtrip_cube() {
        use bevy::prelude::Cuboid;

        let original = Mesh::from(Cuboid::new(1.0, 1.0, 1.0));

        let importer = super::super::MeshImporter::new();
        let topology = importer.import_mesh(&original).unwrap();

        let exporter = MeshExporter::new();
        let exported = exporter.export_mesh(&topology).unwrap();

        let original_pos = original
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("original should have positions");
        let exported_pos = exported
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("exported should have positions");

        assert_eq!(
            original_pos.len(),
            exported_pos.len(),
            "Vertex count should match after round-trip"
        );

        let original_idx = original.indices().expect("original should have indices");
        let exported_idx = exported.indices().expect("exported should have indices");
        assert_eq!(
            original_idx.len(),
            exported_idx.len(),
            "Index count should match after round-trip"
        );

        assert_eq!(original.primitive_topology(), exported.primitive_topology());
    }
}
