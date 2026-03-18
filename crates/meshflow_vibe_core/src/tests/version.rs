/// Tests for version handling and compatibility checking.
///
/// These tests cover:
/// - Version parsing and serialization
/// - Version comparison operations
/// - Version compatibility checking
use crate::shared::version::{is_scene_version_compatible, Version, VersionError};

// ============================================================================
// Version Parsing Tests
// ============================================================================

#[test]
fn test_version_from_str_valid() {
    let v1: Result<Version, _> = "0.1.4".parse();
    let v2: Result<Version, _> = "0.1.5".parse();

    assert!(v1.is_ok());
    assert_eq!(v1.unwrap(), Version::V0_1_4);

    assert!(v2.is_ok());
    assert_eq!(v2.unwrap(), Version::V0_1_5);
}

#[test]
fn test_version_from_str_invalid() {
    let v: Result<Version, _> = "1.0.0".parse();

    assert!(v.is_err());
    match v.unwrap_err() {
        VersionError::InvalidVersion(s) => assert_eq!(s, "1.0.0"),
    }
}

// ============================================================================
// Version Display Tests
// ============================================================================

#[test]
fn test_version_display_v0_1_4() {
    let v = Version::V0_1_4;
    assert_eq!(v.to_string(), "0.1.4");
}

#[test]
fn test_version_display_v0_1_5() {
    let v = Version::V0_1_5;
    assert_eq!(v.to_string(), "0.1.5");
}

// ============================================================================
// Version Component Accessor Tests
// ============================================================================

#[test]
fn test_version_major_v0_1_4() {
    assert_eq!(Version::V0_1_4.major(), 0);
}

#[test]
fn test_version_minor_v0_1_4() {
    assert_eq!(Version::V0_1_4.minor(), 1);
}

#[test]
fn test_version_patch_v0_1_4() {
    assert_eq!(Version::V0_1_4.patch(), 4);
}

#[test]
fn test_version_major_v0_1_5() {
    assert_eq!(Version::V0_1_5.major(), 0);
}

#[test]
fn test_version_minor_v0_1_5() {
    assert_eq!(Version::V0_1_5.minor(), 1);
}

#[test]
fn test_version_patch_v0_1_5() {
    assert_eq!(Version::V0_1_5.patch(), 5);
}

#[test]
fn test_version_suffix_none() {
    assert!(Version::V0_1_4.suffix().is_none());
    assert!(Version::V0_1_5.suffix().is_none());
}

#[test]
fn test_version_is_pre_release() {
    assert!(!Version::V0_1_4.is_pre_release());
    assert!(Version::V0_1_5.is_pre_release());
}

// ============================================================================
// Version Constant Tests
// ============================================================================

#[test]
fn test_version_constants() {
    assert_eq!(Version::CURRENT_VERSION, Version::V0_1_4);
    assert_eq!(Version::MINIMUM_SUPPORTED_VERSION, Version::V0_1_4);
    assert_eq!(Version::PRE_RELEASE_VERSION, Version::V0_1_5);
}

// ============================================================================
// Version Serialization Tests
// ============================================================================

#[test]
fn test_version_serialize_v0_1_4() {
    let v = Version::V0_1_4;
    let serialized = serde_json::to_string(&v).unwrap();
    assert_eq!(serialized, "\"0.1.4\"");
}

#[test]
fn test_version_serialize_v0_1_5() {
    let v = Version::V0_1_5;
    let serialized = serde_json::to_string(&v).unwrap();
    assert_eq!(serialized, "\"0.1.5\"");
}

#[test]
fn test_version_deserialize_v0_1_4() {
    let json = "\"0.1.4\"";
    let deserialized: Version = serde_json::from_str(json).unwrap();
    assert_eq!(deserialized, Version::V0_1_4);
}

#[test]
fn test_version_deserialize_v0_1_5() {
    let json = "\"0.1.5\"";
    let deserialized: Version = serde_json::from_str(json).unwrap();
    assert_eq!(deserialized, Version::V0_1_5);
}

#[test]
fn test_version_deserialize_invalid() {
    let json = "\"1.0.0\"";
    let result: Result<Version, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

// ============================================================================
// Version Comparison Tests
// ============================================================================

#[test]
fn test_version_eq() {
    assert_eq!(Version::V0_1_4, Version::V0_1_4);
    assert_ne!(Version::V0_1_4, Version::V0_1_5);
}

#[test]
fn test_version_partial_cmp_equal() {
    assert_eq!(
        Version::V0_1_4.partial_cmp(&Version::V0_1_4),
        Some(std::cmp::Ordering::Equal)
    );
}

#[test]
fn test_version_partial_cmp_less() {
    assert_eq!(
        Version::V0_1_4.partial_cmp(&Version::V0_1_5),
        Some(std::cmp::Ordering::Less)
    );
}

#[test]
fn test_version_partial_cmp_greater() {
    assert_eq!(
        Version::V0_1_5.partial_cmp(&Version::V0_1_4),
        Some(std::cmp::Ordering::Greater)
    );
}

#[test]
fn test_version_ord_less() {
    assert!(Version::V0_1_4 < Version::V0_1_5);
}

#[test]
fn test_version_ord_greater() {
    assert!(Version::V0_1_5 > Version::V0_1_4);
}

#[test]
fn test_version_ord_min() {
    assert_eq!(Version::V0_1_4.min(Version::V0_1_5), Version::V0_1_4);
    assert_eq!(Version::V0_1_5.min(Version::V0_1_4), Version::V0_1_4);
}

#[test]
fn test_version_ord_max() {
    assert_eq!(Version::V0_1_4.max(Version::V0_1_5), Version::V0_1_5);
    assert_eq!(Version::V0_1_5.max(Version::V0_1_4), Version::V0_1_5);
}

// ============================================================================
// Version Compatibility Tests
// ============================================================================

#[test]
fn test_version_compatible_exact_match() {
    // Current version is V0_1_4, so V0_1_4 should be compatible
    assert!(is_scene_version_compatible(Version::V0_1_4));
}

#[test]
fn test_version_compatible_newer_pre_release() {
    // V0_1_5 is newer but should still be compatible
    assert!(is_scene_version_compatible(Version::V0_1_5));
}

// ============================================================================
// Version Clone and Copy Tests
// ============================================================================

#[test]
fn test_version_clone() {
    let v1 = Version::V0_1_4;
    let v2 = v1.clone();
    assert_eq!(v1, v2);
}

#[test]
fn test_version_copy() {
    let v1 = Version::V0_1_4;
    let v2 = v1; // Copy due to Copy trait
    assert_eq!(v1, v2);
}

// ============================================================================
// Version Debug Tests
// ============================================================================

#[test]
fn test_version_debug_v0_1_4() {
    let v = Version::V0_1_4;
    let debug_output = format!("{:?}", v);
    assert!(debug_output.contains("V0_1_4"));
}

#[test]
fn test_version_debug_v0_1_5() {
    let v = Version::V0_1_5;
    let debug_output = format!("{:?}", v);
    assert!(debug_output.contains("V0_1_5"));
}
