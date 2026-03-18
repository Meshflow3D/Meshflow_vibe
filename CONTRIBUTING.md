# Contributing to Meshflow Vibe

Thank you for your interest in contributing to Meshflow Vibe! This guide covers everything you need to know to get started.

## Development Environment Setup

### Prerequisites

- **Rust** (stable): Install via [rustup](https://rustup.rs/)
- **Cargo**: Comes with Rust installation
- **Git**: For version control

### Getting Started

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Meshflow3D/Meshflow_vibe.git
    cd Meshflow_vibe
   ```

2. **Build the project**:
   ```bash
   cargo build --workspace
   ```

3. **Run the editor**:
   ```bash
   cargo run --package meshflow_vibe
   ```

4. **Run the dungeon example**:
   ```bash
   cargo run --example dungeon
   ```

### Project Structure

```
meshflow-vibe/
├── crates/
│   ├── meshflow_vibe_core/     # Core entities and components
│   ├── meshflow_vibe_editor/   # Editor UI and functionality
│   ├── meshflow_vibe_gizmos/   # Gizmo rendering
│   ├── meshflow_vibe_logging/  # Logging utilities
│   ├── meshflow_vibe_macros/   # Custom macros
│   └── meshflow_vibe_expose/   # Expose functionality
├── examples/
│   └── dungeon.rs              # Example scene
└── src/                        # Main application entry point
```

## Code Style Guidelines

### Rust Conventions

We follow standard Rust conventions with a few additional guidelines:

1. **Formatting**: Use `rustfmt` to format your code:
   ```bash
   cargo fmt
   ```

2. **Linting**: Run `clippy` to catch common mistakes:
   ```bash
   cargo clippy --workspace --all-targets
   ```

3. **Type Safety**: Prefer `Option` and `Result` over error handling panics
4. **Documentation**: Document all public APIs with doc comments
5. **Error Handling**: Use descriptive error messages

### Bevy Framework Patterns

- Use `Message` for entity messaging (Bevy 0.18)
- Follow ECS principles for system design
- Register types with `register_type::<T>()` for inspector support
- Use `Plugin` pattern for modular architecture

## Testing

### Running Tests

```bash
# Run all tests
cargo test --workspace --all-targets

# Run documentation tests
cargo test --workspace --doc

# Run tests for a specific crate
cargo test -p meshflow_vibe_core
```

### Writing Tests

- Place unit tests in the same module as the code being tested
- Use integration tests in `tests/` directory for crate-level testing
- Include both success and failure cases
- Test serialization/deserialization for all component types

## Adding New Granite Types

To add a new entity type to the editor, follow our structured approach. See the full documentation in:

- [Adding New Granite Type](crates/meshflow_vibe_core/src/entities/editable/README.md)

### Quick Reference

1. Create a new folder under `types/` with snake_case naming (e.g., `types/camera_2d/`)
2. Implement required files:
   - `mod.rs` - Main struct with `GraniteType` trait
   - `plugin.rs` - Bevy plugin registration
   - `creation.rs` - Entity spawning logic
   - `ui.rs` - UI rendering and editing
   - `update_event.rs` - Update event struct and system
   - `YourType.png` - Icon file (32x32 or 64x64)

3. Register in `types/plugin.rs`
4. Add to `GraniteTypes` enum in `editable/mod.rs`
5. Add update event to `RequestEntityUpdateFromClass`
6. Export in `types/mod.rs`

Refer to existing types (empty, obj, point_light, directional_light, rectangle_brush, camera_3d) as examples.

## PR Submission Process

### Before Submitting

1. **Update documentation**: Update relevant docs if you changed APIs
2. **Run tests**: Ensure all tests pass:
   ```bash
   cargo test --workspace --all-targets
   ```
3. **Format code**: Run `cargo fmt`
4. **Check linting**: Run `cargo clippy --workspace --all-targets`

### Commit Messages

We use conventional commits for clarity:

```
feat: Add new camera type support

- Implement Camera2D struct with GraniteType trait
- Add UI panel for camera properties
- Include icon in assets directory

Closes #123
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Test additions or changes
- `chore`: Maintenance tasks

### Pull Request Template

When creating a PR, include:

1. **Description**: What the PR does and why
2. **Testing**: How you tested the changes
3. **Screenshots**: UI changes should include before/after screenshots
4. **Related Issues**: Link to any related GitHub issues

### CI/CD

All PRs are automatically tested via GitHub Actions. The CI will:
- Run all tests on Ubuntu
- Check for compilation errors
- Validate documentation builds

## Questions and Issues

- **Bug Reports**: Use the issue template provided
- **Feature Requests**: Open an issue with detailed description
- **Discussions**: Use GitHub Discussions for open-ended questions

## Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on what's best for the community

## License

By contributing, you agree that your contributions will be licensed under:
- MIT License (see LICENSE-MIT)
- Apache License, Version 2.0 (see LICENSE-APACHE)
