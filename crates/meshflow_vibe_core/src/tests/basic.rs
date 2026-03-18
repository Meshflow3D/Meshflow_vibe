/// Basic tests for the test infrastructure.
///
/// These tests verify that the test fixtures work correctly.
use crate::tests::{
    app_should_exit, create_temp_dir, create_temp_file, create_temp_ron_file,
    create_temp_scene_file, create_temp_structure, create_temp_toml_file, create_test_assets,
    run_iterations,
};
use std::collections::HashMap;
use std::fs;

#[test]
fn test_temp_dir_creation() {
    let temp_dir = create_temp_dir();
    assert!(temp_dir.path().exists());
    // Temp dir should auto-clean on drop
}

#[test]
fn test_temp_file_creation() {
    let (_temp_dir, file_path) = create_temp_file("test content");
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "test content");
    // File should exist
    assert!(file_path.exists());
}

#[test]
fn test_temp_structure_creation() {
    let mut structure = HashMap::new();
    structure.insert("dir1/file1.txt", "content1");
    structure.insert("dir1/dir2/file2.txt", "content2");

    let (_temp_dir, base_path) = create_temp_structure(structure);

    assert!(base_path.join("dir1/file1.txt").exists());
    assert!(base_path.join("dir1/dir2/file2.txt").exists());

    let content1 = fs::read_to_string(base_path.join("dir1/file1.txt")).unwrap();
    assert_eq!(content1, "content1");
}

#[test]
fn test_temp_ron_file() {
    let (_temp_dir, ron_path) = create_temp_ron_file(
        r#"
        entity = Entity {
            id: UUID("00000000-0000-0000-0000-000000000000"),
            name: "Test",
        }
    "#,
    );

    assert!(ron_path.exists());
    let content = fs::read_to_string(&ron_path).unwrap();
    assert!(content.contains("entity"));
}

#[test]
fn test_temp_toml_file() {
    let (_temp_dir, toml_path) = create_temp_toml_file(
        r#"
        [section]
        key = "value"
    "#,
    );

    assert!(toml_path.exists());
    let content = fs::read_to_string(&toml_path).unwrap();
    assert!(content.contains("section"));
}

#[test]
fn test_temp_scene_file() {
    let (_temp_dir, scene_path) = create_temp_scene_file(
        r#"
        scene = Scene {
            entities = [
                Entity {
                    id: UUID("00000000-0000-0000-0000-000000000000"),
                }
            ]
        }
    "#,
    );

    assert!(scene_path.exists());
    let content = fs::read_to_string(&scene_path).unwrap();
    assert!(content.contains("scene"));
}

#[test]
fn test_temp_assets() {
    let (_temp_dir, assets_path) = create_test_assets();

    assert!(assets_path.exists());
    assert!(assets_path.is_dir());
    assert!(assets_path.ends_with("assets"));
}

/// Test helper to verify file operations
#[test]
fn test_file_operations_in_temp_dir() {
    let temp_dir = create_temp_dir();
    let test_file = temp_dir.path().join("test.txt");

    // Write content
    fs::write(&test_file, "hello").unwrap();
    assert!(test_file.exists());

    // Read content
    let content = fs::read_to_string(&test_file).unwrap();
    assert_eq!(content, "hello");

    // File should be cleaned up when temp_dir is dropped
}

#[test]
fn test_run_iterations() {
    let mut app = crate::headless_app!();
    run_iterations(&mut app, 5);
    // App should still be valid after iterations
    assert!(!app_should_exit(&app));
}
