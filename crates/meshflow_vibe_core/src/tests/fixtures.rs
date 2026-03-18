/// Fixture utilities for deterministic testing.
///
/// These helpers provide temporary file and asset management for tests
/// that need file I/O without affecting the real filesystem.
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Creates a temporary directory for testing file operations.
///
/// # Returns
///
/// A [`TempDir`] that will be automatically cleaned up when dropped.
///
/// # Example
///
/// ```rust
/// let temp_dir = create_temp_dir();
/// let file_path = temp_dir.path().join("test.txt");
/// fs::write(&file_path, "content").unwrap();
/// // temp_dir is automatically cleaned up when it goes out of scope
/// ```
pub fn create_temp_dir() -> TempDir {
    TempDir::new().expect("Failed to create temporary directory")
}

/// Creates a temporary file with given content.
///
/// # Arguments
///
/// * `content` - Content to write to the file
///
/// # Returns
///
/// A tuple of (`TempDir`, `PathBuf`) where the path points to the created file.
///
/// # Example
///
/// ```rust
/// let (_temp_dir, file_path) = create_temp_file("test content");
/// let content = fs::read_to_string(&file_path).unwrap();
/// assert_eq!(content, "test content");
/// ```
pub fn create_temp_file(content: &str) -> (TempDir, PathBuf) {
    let temp_dir = create_temp_dir();
    let file_path = temp_dir.path().join("test_file.txt");
    fs::write(&file_path, content).expect("Failed to write to temporary file");
    (temp_dir, file_path)
}

/// Creates a temporary directory structure for testing.
///
/// # Arguments
///
/// * `structure` - A map of file paths to their content
///
/// # Returns
///
/// A tuple of (`TempDir`, `PathBuf`) where the path points to the temp directory.
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
///
/// let mut structure = HashMap::new();
/// structure.insert("dir1/file1.txt", "content1");
/// structure.insert("dir1/dir2/file2.txt", "content2");
///
/// let (_temp_dir, base_path) = create_temp_structure(structure);
/// ```
pub fn create_temp_structure(
    structure: std::collections::HashMap<&str, &str>,
) -> (TempDir, PathBuf) {
    let temp_dir = create_temp_dir();
    let base_path = temp_dir.path().to_path_buf();

    for (relative_path, content) in structure {
        let file_path = base_path.join(relative_path);

        // Create parent directories if needed
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create temporary directory structure");
        }

        fs::write(&file_path, content).expect("Failed to write to temporary file");
    }

    (temp_dir, base_path)
}

/// Creates a temporary RON file with given content.
///
/// # Arguments
///
/// * `content` - RON content to write to the file
///
/// # Returns
///
/// A tuple of (`TempDir`, `PathBuf`) where the path points to the created RON file.
///
/// # Example
///
/// ```rust
/// let (_temp_dir, ron_path) = create_temp_ron_file(r#"
///     entity = Entity {
///         id: UUID("00000000-0000-0000-0000-000000000000"),
///         name: "Test",
///     }
/// "#);
/// ```
pub fn create_temp_ron_file(content: &str) -> (TempDir, PathBuf) {
    let temp_dir = create_temp_dir();
    let file_path = temp_dir.path().join("test.ron");
    fs::write(&file_path, content).expect("Failed to write temporary RON file");
    (temp_dir, file_path)
}

/// Creates a temporary TOML file with given content.
///
/// # Arguments
///
/// * `content` - TOML content to write to the file
///
/// # Returns
///
/// A tuple of (`TempDir`, `PathBuf`) where the path points to the created TOML file.
///
/// # Example
///
/// ```rust
/// let (_temp_dir, toml_path) = create_temp_toml_file(r#"
///     [section]
///     key = "value"
/// "#);
/// ```
pub fn create_temp_toml_file(content: &str) -> (TempDir, PathBuf) {
    let temp_dir = create_temp_dir();
    let file_path = temp_dir.path().join("config.toml");
    fs::write(&file_path, content).expect("Failed to write temporary TOML file");
    (temp_dir, file_path)
}

/// Creates a temporary scene file with given content.
///
/// # Arguments
///
/// * `content` - Scene content to write to the file
///
/// # Returns
///
/// A tuple of (`TempDir`, `PathBuf`) where the path points to the created scene file.
///
/// # Example
///
/// ```rust
/// let (_temp_dir, scene_path) = create_temp_scene_file(r#"
///     scene = Scene {
///         entities = [
///             Entity {
///                 id: UUID("00000000-0000-0000-0000-000000000000"),
///             }
///         ]
///     }
/// "#);
/// ```
pub fn create_temp_scene_file(content: &str) -> (TempDir, PathBuf) {
    let temp_dir = create_temp_dir();
    let file_path = temp_dir.path().join("test.scene");
    fs::write(&file_path, content).expect("Failed to write temporary scene file");
    (temp_dir, file_path)
}

/// Helper to get the absolute path of a file in a temp directory.
///
/// # Arguments
///
/// * `temp_dir` - The temporary directory
/// * `filename` - The filename to create
///
/// # Returns
///
/// An absolute path to the file.
pub fn get_temp_file_path(temp_dir: &TempDir, filename: &str) -> PathBuf {
    temp_dir.path().join(filename)
}

/// Helper to check if a file exists in a temporary directory.
///
/// # Arguments
///
/// * `temp_dir` - The temporary directory
/// * `filename` - The filename to check
///
/// # Returns
///
/// `true` if the file exists, `false` otherwise.
pub fn temp_file_exists(temp_dir: &TempDir, filename: &str) -> bool {
    temp_dir.path().join(filename).exists()
}

/// Helper to read a file from a temporary directory.
///
/// # Arguments
///
/// * `temp_dir` - The temporary directory
/// * `filename` - The filename to read
///
/// # Returns
///
/// The content of the file as a String.
pub fn read_temp_file(temp_dir: &TempDir, filename: &str) -> String {
    fs::read_to_string(temp_dir.path().join(filename)).expect("Failed to read temporary file")
}

/// Creates a minimal assets directory structure for testing.
///
/// # Returns
///
/// A tuple of (`TempDir`, `PathBuf`) where the path points to the assets directory.
///
/// # Example
///
/// ```rust
/// let (_temp_dir, assets_path) = create_test_assets();
/// let materials_dir = assets_path.join("materials");
/// fs::create_dir_all(&materials_dir).unwrap();
/// ```
pub fn create_test_assets() -> (TempDir, PathBuf) {
    let temp_dir = create_temp_dir();
    let assets_path = temp_dir.path().join("assets");
    fs::create_dir_all(&assets_path).expect("Failed to create assets directory");
    (temp_dir, assets_path)
}
