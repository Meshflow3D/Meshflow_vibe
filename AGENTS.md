# Meshflow Vibe - AI Agent Context

## Project Overview

Meshflow Vibe is an interactive 3D editor for the Bevy game engine (v0.18). It enables users to create, edit, save, and load Bevy scenes with entity components, transforms, and custom data in a visual interface.

**Status**: Early development (v0.3.1) - expect bugs

## Workspace Structure

```
crates/
├── meshflow_vibe_core/    # Core serialization/deserialization with macros and logging
├── meshflow_vibe_editor/  # UI editor functionality
├── meshflow_vibe_gizmos/  # Gizmo components for visual editing
├── meshflow_vibe_logging/ # Logging utilities
└── meshflow_vibe_macros/  # Procedural macros (#[granite_component], #[ui_callable_events])
```

## Key Commands

```bash
# Run all tests (including headless Bevy tests)
cargo test --workspace --all-targets

# Run doctests
cargo test --workspace --doc

# Run examples (e.g., dungeon example)
cargo run --release --example dungeon
```

## Architecture

**Feature Flags** (controlled via Cargo.toml):

- `core` - Serialization with macros and logging (no UI)
- `editor` - Full editor UI (depends on core + gizmos)
- `gizmos` - Gizmo components only (depends on core)

**Default features**: `["core", "editor", "gizmos"]`

**Key Dependencies**:

- Bevy 0.18.1
- bevy-inspector-egui 0.36.0
- bevy_egui 0.39.0
- ron (RON format for scene serialization)

## Important Notes

**File Update Policy**:

- Update this AGENTS.md file with every significant PR
- Update CHANGELOG.md accordingly with version bumps and changes

**Augment Code Intent**:

- This project uses Augment Code Intent intensively for context retrieval and codebase understanding
- Reference `codebase-retrieval` tool for semantic searches

**Context Loading**:

- AGENTS.md is loaded hierarchically from workspace root and all subdirectories
- All specialists (Developer, Implementor, Verifier, etc.) share the same Context Engine access
- Project-specific rules in AGENTS.md are automatically included in every agent's context
- Context is maintained across sessions and workspace boundaries

**README.md Policy**:

- DO NOT modify README.md in this task - it is handled in a separate PR

## Files Created

- `AGENTS.md` (2026-03-18) - AI agent context file with project overview, workspace structure, commands, and architecture notes

## Recent Changes

- **2026-03-18**: Automated DMG release workflow updated to package cube demo (cube, light, camera) instead of dungeon example; release target changed from dungeon to cube demo

## Verification

Run verification commands on completion:

```bash
cargo test --workspace --all-targets
cargo test --workspace --doc
```

## Augment Intent Specialists


### Available Specialists

- **Developer** - General-purpose coding tasks across the codebase
- **Implementor** - Focused implementation work, minimal changes, following existing patterns
- **PR Shepherd** - Guides pull requests to merge-ready state through iterations
- **Verifier** - Thorough verification and review tasks, checking implementation against requirements
- **Coordinator** - Planning, task decomposition, and agent delegation
- **PR Reviewer** - Reviews pull requests for quality, correctness, and adherence to standards
- **UI Designer** - User interface design and UX-focused tasks

### Recommended Workflow

#### Development Workflow

1. Use **Coordinator** for planning and task decomposition
2. Use **Implementor** for code execution and implementation
3. Call **Verifier** agent after each wave of implementation to verify correctness
4. Call **PR Reviewer** before raising a PR to catch issues early
5. Use **PR Shepherd** for shepherding PRs to merge-ready state

#### Best Practices

- Always verify implementation before moving to next feature
- Keep tasks focused and well-scoped
- Document decisions and learnings in task notes
- Run verification commands (tests, typecheck, lint) before completion