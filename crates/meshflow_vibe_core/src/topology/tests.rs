use super::*;
use bevy::prelude::Vec3;

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Simple valid topology creation (triangle)
    // ========================================================================

    #[test]
    fn test_create_triangle_topology() {
        let mut topology = EditableTopology::new();

        // Create 3 vertices
        let v0_id = topology.generate_vert_id();
        let v1_id = topology.generate_vert_id();
        let v2_id = topology.generate_vert_id();

        topology.insert_vertex(Vert::new(v0_id, Vec3::new(0.0, 0.0, 0.0)));
        topology.insert_vertex(Vert::new(v1_id, Vec3::new(1.0, 0.0, 0.0)));
        topology.insert_vertex(Vert::new(v2_id, Vec3::new(0.5, 1.0, 0.0)));

        // Create 3 edges
        let e0_id = topology.generate_edge_id();
        let e1_id = topology.generate_edge_id();
        let e2_id = topology.generate_edge_id();

        let e0 = Edge::new(e0_id);
        let e1 = Edge::new(e1_id);
        let e2 = Edge::new(e2_id);

        topology.insert_edge(e0);
        topology.insert_edge(e1);
        topology.insert_edge(e2);

        // Create face first
        let face_id = topology.generate_face_id();

        // Create 3 loops for the triangle face
        let l0_id = topology.generate_loop_id();
        let l1_id = topology.generate_loop_id();
        let l2_id = topology.generate_loop_id();

        let loop0 = Loop::new(l0_id, face_id, e0_id, v0_id, 0);
        let loop1 = Loop::new(l1_id, face_id, e1_id, v1_id, 1);
        let loop2 = Loop::new(l2_id, face_id, e2_id, v2_id, 2);

        topology.loops.insert(l0_id, loop0);
        topology.loops.insert(l1_id, loop1);
        topology.loops.insert(l2_id, loop2);

        // Create face with the 3 loops
        let mut face = Face::new(face_id);
        face.loops = vec![l0_id, l1_id, l2_id];

        topology.insert_face(face);

        // Validate topology
        let errors = topology.validate();
        assert!(
            errors.is_empty(),
            "Triangle topology should be valid, got errors: {:?}",
            errors
        );
        assert!(topology.is_valid());
    }

    #[test]
    fn test_create_quad_topology() {
        let mut topology = EditableTopology::new();

        // Create 4 vertices
        let v0_id = topology.generate_vert_id();
        let v1_id = topology.generate_vert_id();
        let v2_id = topology.generate_vert_id();
        let v3_id = topology.generate_vert_id();

        topology.insert_vertex(Vert::new(v0_id, Vec3::ZERO));
        topology.insert_vertex(Vert::new(v1_id, Vec3::X));
        topology.insert_vertex(Vert::new(v2_id, Vec3::X + Vec3::Y));
        topology.insert_vertex(Vert::new(v3_id, Vec3::Y));

        // Create 4 edges
        let e0_id = topology.generate_edge_id();
        let e1_id = topology.generate_edge_id();
        let e2_id = topology.generate_edge_id();
        let e3_id = topology.generate_edge_id();

        topology.insert_edge(Edge::new(e0_id));
        topology.insert_edge(Edge::new(e1_id));
        topology.insert_edge(Edge::new(e2_id));
        topology.insert_edge(Edge::new(e3_id));

        // Create face first
        let face_id = topology.generate_face_id();

        // Create 4 loops
        let l0_id = topology.generate_loop_id();
        let l1_id = topology.generate_loop_id();
        let l2_id = topology.generate_loop_id();
        let l3_id = topology.generate_loop_id();

        let loop0 = Loop::new(l0_id, face_id, e0_id, v0_id, 0);
        let loop1 = Loop::new(l1_id, face_id, e1_id, v1_id, 1);
        let loop2 = Loop::new(l2_id, face_id, e2_id, v2_id, 2);
        let loop3 = Loop::new(l3_id, face_id, e3_id, v3_id, 3);

        topology.loops.insert(l0_id, loop0);
        topology.loops.insert(l1_id, loop1);
        topology.loops.insert(l2_id, loop2);
        topology.loops.insert(l3_id, loop3);

        // Create face
        let mut face = Face::new(face_id);
        face.loops = vec![l0_id, l1_id, l2_id, l3_id];

        topology.insert_face(face);

        assert!(topology.is_valid());
        assert_eq!(topology.face_count(), 1);
        assert_eq!(topology.vertex_count(), 4);
        assert_eq!(topology.edge_count(), 4);
        assert_eq!(topology.loop_count(), 4);
    }

    // ========================================================================
    // Adjacency queries
    // ========================================================================

    #[test]
    fn test_face_loops_for_edge() {
        let mut topology = EditableTopology::new();

        let v0_id = topology.generate_vert_id();
        let v1_id = topology.generate_vert_id();

        topology.insert_vertex(Vert::new(v0_id, Vec3::ZERO));
        topology.insert_vertex(Vert::new(v1_id, Vec3::X));

        let e0_id = topology.generate_edge_id();
        topology.insert_edge(Edge::new(e0_id));

        let f0_id = topology.generate_face_id();

        let l0_id = topology.generate_loop_id();
        let l1_id = topology.generate_loop_id();

        let loop0 = Loop::new(l0_id, f0_id, e0_id, v0_id, 0);
        let loop1 = Loop::new(l1_id, f0_id, e0_id, v1_id, 1);

        topology.loops.insert(l0_id, loop0);
        topology.loops.insert(l1_id, loop1);

        let mut face = Face::new(f0_id);
        face.loops = vec![l0_id, l1_id];
        topology.insert_face(face);

        let loops = topology.face_loops_for_edge(e0_id);
        assert_eq!(loops.len(), 2);
        assert!(loops.iter().any(|l| l.id == l0_id));
        assert!(loops.iter().any(|l| l.id == l1_id));
    }

    #[test]
    fn test_faces_for_edge() {
        let mut topology = EditableTopology::new();

        let v0_id = topology.generate_vert_id();
        let v1_id = topology.generate_vert_id();

        topology.insert_vertex(Vert::new(v0_id, Vec3::ZERO));
        topology.insert_vertex(Vert::new(v1_id, Vec3::X));

        let e0_id = topology.generate_edge_id();
        topology.insert_edge(Edge::new(e0_id));

        let f0_id = topology.generate_face_id();
        let f1_id = topology.generate_face_id();

        let l0_id = topology.generate_loop_id();
        let l1_id = topology.generate_loop_id();

        let loop0 = Loop::new(l0_id, f0_id, e0_id, v0_id, 0);
        let loop1 = Loop::new(l1_id, f1_id, e0_id, v1_id, 1);

        topology.loops.insert(l0_id, loop0);
        topology.loops.insert(l1_id, loop1);

        let mut face0 = Face::new(f0_id);
        face0.loops = vec![l0_id];
        topology.insert_face(face0);

        let mut face1 = Face::new(f1_id);
        face1.loops = vec![l1_id];
        topology.insert_face(face1);

        let faces = topology.faces_for_edge(e0_id);
        assert_eq!(faces.len(), 2);
    }

    #[test]
    fn test_edges_for_face() {
        let mut topology = EditableTopology::new();

        let v0_id = topology.generate_vert_id();
        let v1_id = topology.generate_vert_id();
        let v2_id = topology.generate_vert_id();

        topology.insert_vertex(Vert::new(v0_id, Vec3::ZERO));
        topology.insert_vertex(Vert::new(v1_id, Vec3::X));
        topology.insert_vertex(Vert::new(v2_id, Vec3::Y));

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

        let edges = topology.edges_for_face(face_id);
        assert_eq!(edges.len(), 3);
    }

    #[test]
    fn test_vertices_for_face() {
        let mut topology = EditableTopology::new();

        let v0_id = topology.generate_vert_id();
        let v1_id = topology.generate_vert_id();
        let v2_id = topology.generate_vert_id();

        let verts = vec![
            Vert::new(v0_id, Vec3::ZERO),
            Vert::new(v1_id, Vec3::X),
            Vert::new(v2_id, Vec3::Y),
        ];

        for vert in verts {
            topology.insert_vertex(vert);
        }

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

        let vertices = topology.vertices_for_face(face_id);
        assert_eq!(vertices.len(), 3);
    }

    #[test]
    fn test_vertex_at_loop() {
        let mut topology = EditableTopology::new();

        let v0_id = topology.generate_vert_id();
        topology.insert_vertex(Vert::new(v0_id, Vec3::ZERO));

        let e0_id = topology.generate_edge_id();
        topology.insert_edge(Edge::new(e0_id));

        let face_id = topology.generate_face_id();

        let l0_id = topology.generate_loop_id();
        let loop0 = Loop::new(l0_id, face_id, e0_id, v0_id, 0);

        topology.loops.insert(l0_id, loop0);

        let mut face = Face::new(face_id);
        face.loops = vec![l0_id];

        topology.insert_face(face);

        let vertex = topology.vertex_at_loop(face_id, 0);
        assert!(vertex.is_some());
        assert_eq!(vertex.unwrap().id, v0_id);
    }

    #[test]
    fn test_edge_at_loop() {
        let mut topology = EditableTopology::new();

        let v0_id = topology.generate_vert_id();
        topology.insert_vertex(Vert::new(v0_id, Vec3::ZERO));

        let e0_id = topology.generate_edge_id();
        topology.insert_edge(Edge::new(e0_id));

        let face_id = topology.generate_face_id();

        let l0_id = topology.generate_loop_id();
        let loop0 = Loop::new(l0_id, face_id, e0_id, v0_id, 0);

        topology.loops.insert(l0_id, loop0);

        let mut face = Face::new(face_id);
        face.loops = vec![l0_id];

        topology.insert_face(face);

        let edge = topology.edge_at_loop(face_id, 0);
        assert!(edge.is_some());
        assert_eq!(edge.unwrap().id, e0_id);
    }

    // ========================================================================
    // Invalid/malformed structure validation failures
    // ========================================================================

    #[test]
    fn test_validation_broken_loop_reference() {
        let mut topology = EditableTopology::new();

        let v0_id = topology.generate_vert_id();
        let e0_id = topology.generate_edge_id();

        topology.insert_vertex(Vert::new(v0_id, Vec3::ZERO));
        topology.insert_edge(Edge::new(e0_id));

        // Create a loop with a face_id that doesn't exist
        let l0_id = topology.generate_loop_id();
        let face_id = topology.generate_face_id();
        let loop0 = Loop::new(l0_id, face_id, e0_id, v0_id, 0);

        topology.loops.insert(l0_id, loop0);

        let errors = topology.validate();
        // Loop references a face_id that doesn't exist
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_validation_loop_references_invalid_vertex() {
        let mut topology = EditableTopology::new();

        let e0_id = topology.generate_edge_id();
        let non_existent_vert_id = VertId::new(uuid::Uuid::new_v4());
        let face_id = topology.generate_face_id();

        topology.insert_edge(Edge::new(e0_id));

        let l0_id = topology.generate_loop_id();
        let loop0 = Loop::new(l0_id, face_id, e0_id, non_existent_vert_id, 0);

        topology.loops.insert(l0_id, loop0);

        let errors = topology.validate();
        assert!(!errors.is_empty());
        assert!(matches!(
            errors[0],
            TopologyValidationError::LoopReferencesInvalidVertex { .. }
        ));
    }

    #[test]
    fn test_validation_loop_references_invalid_edge() {
        let mut topology = EditableTopology::new();

        let v0_id = topology.generate_vert_id();
        let non_existent_edge_id = EdgeId::new(uuid::Uuid::new_v4());
        let face_id = topology.generate_face_id();

        topology.insert_vertex(Vert::new(v0_id, Vec3::ZERO));

        let l0_id = topology.generate_loop_id();
        let loop0 = Loop::new(l0_id, face_id, non_existent_edge_id, v0_id, 0);

        topology.loops.insert(l0_id, loop0);

        let errors = topology.validate();
        assert!(!errors.is_empty());
        assert!(matches!(
            errors[0],
            TopologyValidationError::LoopReferencesInvalidEdge { .. }
        ));
    }

    #[test]
    fn test_validation_face_loop_index_mismatch() {
        let mut topology = EditableTopology::new();

        let v0_id = topology.generate_vert_id();
        let e0_id = topology.generate_edge_id();
        let face_id = topology.generate_face_id();

        topology.insert_vertex(Vert::new(v0_id, Vec3::ZERO));
        topology.insert_edge(Edge::new(e0_id));

        let l0_id = topology.generate_loop_id();
        // Set index_in_face to wrong value (should be 0, but we set it to 5)
        let loop0 = Loop::new(l0_id, face_id, e0_id, v0_id, 5);

        topology.loops.insert(l0_id, loop0);

        let mut face = Face::new(face_id);
        face.loops = vec![l0_id];

        topology.insert_face(face);

        let errors = topology.validate();
        assert!(!errors.is_empty());
        assert!(matches!(
            errors[0],
            TopologyValidationError::FaceLoopIndexMismatch { .. }
        ));
    }

    #[test]
    fn test_validation_edge_loop_end_invalid() {
        let mut topology = EditableTopology::new();

        let v0_id = topology.generate_vert_id();
        let e0_id = topology.generate_edge_id();
        let non_existent_loop_id = LoopId::new(999);
        let face_id = topology.generate_face_id();

        topology.insert_vertex(Vert::new(v0_id, Vec3::ZERO));

        // Create a loop that references the edge
        let l0_id = topology.generate_loop_id();
        let loop0 = Loop::new(l0_id, face_id, e0_id, v0_id, 0);
        topology.loops.insert(l0_id, loop0);

        // Create edge with valid loop ends
        let mut edge = Edge::new(e0_id);
        edge.add_loop_end(l0_id);
        // Add an invalid loop end
        edge.add_loop_end(non_existent_loop_id);
        topology.insert_edge(edge);

        let errors = topology.validate();
        // Edge loop ends are validated - should find the non-existent loop
        assert!(!errors.is_empty());
        assert!(matches!(
            errors
                .iter()
                .find(|e| matches!(e, TopologyValidationError::EdgeLoopEndInvalid { .. })),
            Some(TopologyValidationError::EdgeLoopEndInvalid { .. })
        ));
    }

    #[test]
    fn test_validation_invalid_topology_with_missing_face() {
        let mut topology = EditableTopology::new();

        let v0_id = topology.generate_vert_id();
        let v1_id = topology.generate_vert_id();
        let v2_id = topology.generate_vert_id();

        topology.insert_vertex(Vert::new(v0_id, Vec3::ZERO));
        topology.insert_vertex(Vert::new(v1_id, Vec3::X));
        topology.insert_vertex(Vert::new(v2_id, Vec3::Y));

        let e0_id = topology.generate_edge_id();
        let e1_id = topology.generate_edge_id();
        let e2_id = topology.generate_edge_id();

        topology.insert_edge(Edge::new(e0_id));
        topology.insert_edge(Edge::new(e1_id));
        topology.insert_edge(Edge::new(e2_id));

        // Create a non-existent face_id that will be referenced by loops
        let missing_face_id = FaceId::new(uuid::Uuid::new_v4());

        let l0_id = topology.generate_loop_id();
        let l1_id = topology.generate_loop_id();
        let l2_id = topology.generate_loop_id();

        // Loops reference a face that doesn't exist (not dummy, but real non-existent ID)
        let loop0 = Loop::new(l0_id, missing_face_id, e0_id, v0_id, 0);
        let loop1 = Loop::new(l1_id, missing_face_id, e1_id, v1_id, 1);
        let loop2 = Loop::new(l2_id, missing_face_id, e2_id, v2_id, 2);

        topology.loops.insert(l0_id, loop0);
        topology.loops.insert(l1_id, loop1);
        topology.loops.insert(l2_id, loop2);

        // Don't insert any face with this ID
        // The loops reference missing_face_id but the face doesn't exist in the topology

        let errors = topology.validate();
        // Loops reference invalid face IDs (missing_face_id that doesn't exist)
        assert!(!errors.is_empty());
        assert!(matches!(
            errors[0],
            TopologyValidationError::LoopReferencesInvalidFace { .. }
        ));
    }

    #[test]
    fn test_edge_non_manifold_support() {
        let mut topology = EditableTopology::new();

        let v0_id = topology.generate_vert_id();
        let v1_id = topology.generate_vert_id();

        topology.insert_vertex(Vert::new(v0_id, Vec3::ZERO));
        topology.insert_vertex(Vert::new(v1_id, Vec3::X));

        let e0_id = topology.generate_edge_id();

        // Add 3 loop ends to simulate non-manifold (3 faces meeting at one edge)
        let l0_id = topology.generate_loop_id();
        let l1_id = topology.generate_loop_id();
        let l2_id = topology.generate_loop_id();

        let mut edge = Edge::new(e0_id);
        edge.add_loop_end(l0_id);
        edge.add_loop_end(l1_id);
        edge.add_loop_end(l2_id);

        topology.insert_edge(edge.clone());

        // Verify edge has 3 valid loop ends
        assert_eq!(edge.face_count(), 3);
        assert!(edge.is_non_manifold());
        assert!(!edge.is_manifold_interior());
        assert!(!edge.is_boundary());

        // Verify all loop ends are accessible
        let valid_ends = edge.valid_loop_ends();
        assert_eq!(valid_ends.len(), 3);
        assert!(valid_ends.contains(&l0_id));
        assert!(valid_ends.contains(&l1_id));
        assert!(valid_ends.contains(&l2_id));
    }

    #[test]
    fn test_topology_default() {
        let topology: EditableTopology = Default::default();
        assert_eq!(topology.vertex_count(), 0);
        assert_eq!(topology.edge_count(), 0);
        assert_eq!(topology.face_count(), 0);
        assert_eq!(topology.loop_count(), 0);
        assert!(topology.is_valid());
    }

    #[test]
    fn test_topology_with_capacity() {
        let topology = EditableTopology::with_capacity(10, 20, 5);
        assert_eq!(topology.vertex_count(), 0);
        assert_eq!(topology.edge_count(), 0);
        assert_eq!(topology.face_count(), 0);
        assert!(topology.is_valid());
    }

    #[test]
    fn test_remove_elements() {
        let mut topology = EditableTopology::new();

        let v0_id = topology.generate_vert_id();
        let e0_id = topology.generate_edge_id();
        let f0_id = topology.generate_face_id();

        topology.insert_vertex(Vert::new(v0_id, Vec3::ZERO));
        topology.insert_edge(Edge::new(e0_id));

        let l0_id = topology.generate_loop_id();
        let loop0 = Loop::new(l0_id, f0_id, e0_id, v0_id, 0);
        topology.loops.insert(l0_id, loop0);

        let mut face = Face::new(f0_id);
        face.loops = vec![l0_id];
        topology.insert_face(face);

        // Remove elements
        assert!(topology.remove_vertex(v0_id).is_some());
        assert!(topology.remove_edge(e0_id).is_some());
        assert!(topology.remove_face(f0_id).is_some());

        assert_eq!(topology.vertex_count(), 0);
        assert_eq!(topology.edge_count(), 0);
        assert_eq!(topology.face_count(), 0);
    }

    #[test]
    fn test_id_generation() {
        let mut topology = EditableTopology::new();

        let vert_id = topology.generate_vert_id();
        let edge_id = topology.generate_edge_id();
        let face_id = topology.generate_face_id();
        let loop0_id = topology.generate_loop_id();
        let loop1_id = topology.generate_loop_id();

        assert_ne!(vert_id, VertId::dummy());
        assert_ne!(edge_id, EdgeId::dummy());
        assert_ne!(face_id, FaceId::dummy());
        assert_ne!(loop0_id, LoopId::dummy());
        assert_ne!(loop1_id, LoopId::dummy());

        // Loop IDs should be sequential
        assert_eq!(loop0_id.index(), 0);
        assert_eq!(loop1_id.index(), 1);
    }

    #[test]
    fn test_iteration_helpers() {
        let mut topology = EditableTopology::new();

        let v0_id = topology.generate_vert_id();
        let v1_id = topology.generate_vert_id();
        let e0_id = topology.generate_edge_id();
        let f0_id = topology.generate_face_id();

        topology.insert_vertex(Vert::new(v0_id, Vec3::ZERO));
        topology.insert_vertex(Vert::new(v1_id, Vec3::X));
        topology.insert_edge(Edge::new(e0_id));

        let face = Face::new(f0_id);
        topology.insert_face(face);

        let vertex_ids: Vec<_> = topology.vertex_ids().collect();
        let edge_ids: Vec<_> = topology.edge_ids().collect();
        let face_ids: Vec<_> = topology.face_ids().collect();

        assert_eq!(vertex_ids.len(), 2);
        assert_eq!(edge_ids.len(), 1);
        assert_eq!(face_ids.len(), 1);
    }
}
