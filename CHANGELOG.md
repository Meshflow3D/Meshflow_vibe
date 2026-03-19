# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Wave 1: Edit mode session state foundation - `EditSession` resource, `InEditMode` marker, `EnterEditMode`/`ExitEditMode` messages, `OnEnterEditMode`/`OnExitEditMode` observer events, mode eligibility checks
- Editable topology ownership contract: `TopologyId`, `TopologyOwner`, `EditableTopologyRegistry`
- CI workflow with CHANGELOG.md validation on PRs (macos-latest)
- Release workflow for macOS ARM builds (triggered by GitHub release published, mohit-meshflow only)
- Local build script (scripts/build-release.sh)
- Clippy and format checks in CI
- Gatekeeper-compatible release flow with Developer ID signing and notarization
- Automated notarization ticket stapling for offline validation
- Explicit credential validation before release upload
- Mesh exporter API (`MeshExporter`, `MeshExportError`) for `EditableTopology` -> Bevy `Mesh` conversion
- Round-trip conversion tests for supported manifolds and explicit unsupported-case coverage

### Changed

### Deprecated

### Removed

### Fixed

- Edit mode plugin: Fixed Bevy 0.18 Message API compatibility (Event→Message types, .add_message(), MessageWriter::write(), removed duplicate registration)
- Edit mode tests: Fixed Entity creation for Bevy 0.18 (Entity::from_bits() instead of deprecated Entity::from_raw())
- Automated DMG release workflow: now packages the cube demo (cube, light, camera) instead of the dungeon example
- Fixed test_save_preserve_disk_transform integration test

### Security

## [0.3.1] - 2026-03-18

### Added

### Changed

### Deprecated

### Removed

### Fixed

### Security

[Unreleased]: https://github.com/Meshflow3D/Meshflow_vibe/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/Meshflow3D/Meshflow_vibe/releases/tag/v0.3.1
