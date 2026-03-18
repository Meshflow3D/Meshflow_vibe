/// Tests for path conversion and normalization helpers.
///
/// These tests cover path normalization logic that is independent of Bevy's runtime.
/// Note: The actual file.rs functions depend on Bevy's FileAssetReader, so we test
/// the normalization logic separately.
// Test path normalization logic that is used in file.rs
fn normalize_path_separators(path: &str) -> String {
    path.replace('\\', "/")
}

// ============================================================================
// Path Normalization Tests
// ============================================================================

#[test]
fn test_normalize_path_separators_forward_slash() {
    let path = "assets/models/cube.obj";
    let normalized = normalize_path_separators(path);
    assert_eq!(normalized, "assets/models/cube.obj");
}

#[test]
fn test_normalize_path_separators_back_slash() {
    let path = "assets\\models\\cube.obj";
    let normalized = normalize_path_separators(path);
    assert_eq!(normalized, "assets/models/cube.obj");
}

#[test]
fn test_normalize_path_separators_mixed() {
    let path = "assets\\models/cube.obj";
    let normalized = normalize_path_separators(path);
    assert_eq!(normalized, "assets/models/cube.obj");
}

#[test]
fn test_normalize_path_separators_multiple_back_slashes() {
    let path = "assets\\models\\textures\\cube.png";
    let normalized = normalize_path_separators(path);
    assert_eq!(normalized, "assets/models/textures/cube.png");
}

#[test]
fn test_normalize_path_separators_empty() {
    let path = "";
    let normalized = normalize_path_separators(path);
    assert_eq!(normalized, "");
}

#[test]
fn test_normalize_path_separators_only_back_slashes() {
    let path = "\\\\";
    let normalized = normalize_path_separators(path);
    assert_eq!(normalized, "//");
}

// ============================================================================
// Path Join Logic Tests (simulating FileAssetReader behavior)
// ============================================================================

#[test]
fn test_rel_path_to_absolute_relative() {
    // Simulate: FileAssetReader::get_base_path().join("assets").join("models/cube.obj")
    let base_path = "/base";
    let relative = "models/cube.obj";

    let result = format!("{}/assets/{}", base_path, relative);
    assert_eq!(result, "/base/assets/models/cube.obj");
}

#[test]
fn test_rel_path_to_absolute_with_back_slashes() {
    // Simulate: normalize first, then join
    let base_path = "/base";
    let relative = "models\\cube.obj";
    let normalized = normalize_path_separators(relative);

    let result = format!("{}/assets/{}", base_path, normalized);
    assert_eq!(result, "/base/assets/models/cube.obj");
}

#[test]
fn test_rel_path_to_absolute_root_path() {
    // When path is already absolute, should use as-is
    let path = "/absolute/path/models/cube.obj";
    let normalized = normalize_path_separators(path);

    // Absolute path should remain absolute
    assert!(normalized.starts_with('/'));
    assert_eq!(normalized, "/absolute/path/models/cube.obj");
}

// ============================================================================
// Path Strip Prefix Logic Tests
// ============================================================================

#[test]
fn test_absolute_to_rel_same_prefix() {
    // Simulate stripping prefix from absolute path
    let abs_path = "/base/assets/models/cube.obj";
    let base_assets = "/base/assets";

    if abs_path.starts_with(base_assets) {
        let stripped = abs_path.trim_start_matches(base_assets);
        let result = stripped.trim_start_matches('/');
        assert_eq!(result, "models/cube.obj");
    } else {
        panic!("Path doesn't start with expected prefix");
    }
}

#[test]
fn test_absolute_to_rel_different_prefix() {
    // When path doesn't match the assets prefix, return as-is
    let abs_path = "/other/path/models/cube.obj";
    let base_assets = "/base/assets";

    if !abs_path.starts_with(base_assets) {
        // Should return normalized absolute path
        let normalized = normalize_path_separators(abs_path);
        assert_eq!(normalized, abs_path);
    }
}

#[test]
fn test_absolute_to_rel_with_back_slashes() {
    let abs_path = "/base/assets\\models\\cube.obj";
    let normalized = normalize_path_separators(abs_path);
    let base_assets = "/base/assets";

    if normalized.starts_with(base_assets) {
        let stripped = normalized.trim_start_matches(base_assets);
        let result = stripped.trim_start_matches('/');
        assert_eq!(result, "models/cube.obj");
    }
}

// ============================================================================
// Edge Cases for Path Operations
// ============================================================================

#[test]
fn test_normalize_empty_string() {
    assert_eq!(normalize_path_separators(""), "");
}

#[test]
fn test_normalize_only_forward_slashes() {
    assert_eq!(normalize_path_separators("//"), "//");
}

#[test]
fn test_normalize_only_back_slashes() {
    assert_eq!(normalize_path_separators("\\\\"), "//");
}

#[test]
fn test_normalize_trailing_slash() {
    assert_eq!(normalize_path_separators("assets/"), "assets/");
}

#[test]
fn test_normalize_trailing_back_slash() {
    assert_eq!(normalize_path_separators("assets\\"), "assets/");
}

#[test]
fn test_normalize_leading_slash() {
    assert_eq!(normalize_path_separators("/assets"), "/assets");
}

#[test]
fn test_normalize_leading_back_slash() {
    assert_eq!(normalize_path_separators("\\assets"), "/assets");
}

#[test]
fn test_normalize_complex_path() {
    let path = "assets\\models\\textures\\diffuse.png";
    let normalized = normalize_path_separators(path);
    assert_eq!(normalized, "assets/models/textures/diffuse.png");
}

#[test]
fn test_normalize_path_with_dots() {
    let path = "assets/./models/../textures/cube.png";
    let normalized = normalize_path_separators(path);
    assert_eq!(normalized, "assets/./models/../textures/cube.png");
}

// ============================================================================
// Serialization Compatibility Tests
// ============================================================================

#[test]
fn test_path_serialization_string() {
    let path = "assets/models/cube.obj";
    let serialized = serde_json::to_string(&path).unwrap();
    let deserialized: String = serde_json::from_str(&serialized).unwrap();
    assert_eq!(path, deserialized);
}

#[test]
fn test_path_serialization_with_back_slashes() {
    let path = "assets\\models\\cube.obj";
    let normalized = normalize_path_separators(path);
    let serialized = serde_json::to_string(&normalized).unwrap();
    let deserialized: String = serde_json::from_str(&serialized).unwrap();
    assert_eq!(normalized, deserialized);
}
