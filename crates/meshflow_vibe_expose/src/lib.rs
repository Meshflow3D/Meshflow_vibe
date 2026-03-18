//! # Meshflow Vibe Expose
//!
//! This crate provides the infrastructure for exposing Bevy components to the editor interface.
//! It registers component types with the type registry and marks them as exposed to the editor,
//! enabling runtime inspection and modification of component state from the editor UI.
//!
//! ## Purpose
//!
//! The expose system allows selected Bevy components to be accessible from the editor,
//! facilitating development workflows where component values can be inspected or adjusted
//! without restarting the application.
//!
//! ## Architecture
//!
//! - [`MeshflowVibeExposePlugin`] - Registers exposed component types during app initialization
//! - [`ExposeToEditor`] - Marker trait for components that should be exposed to the editor
//!
//! ## Usage
//!
//! Add [`MeshflowVibeExposePlugin`] to your Bevy app to enable component exposure:
//!
//! ```ignore
//! App::new()
//!     .add_plugins(MeshflowVibeExposePlugin)
//!     .run();
//! ```
//!
//! Implement [`ExposeToEditor`] for any component you wish to expose, specifying whether
//! it should be treated as read-only or modifiable.

use bevy::{prelude::*, reflect::TypeRegistry};
use meshflow_vibe_core::entities::ExposedToEditor;

/// Plugin that registers components exposed to the editor.
///
/// This plugin initializes the expose system by registering all components that implement
/// the [`ExposeToEditor`] trait with the Bevy type registry. When a component is registered,
/// it receives an [`ExposedToEditor`] marker that signals to the editor system that this
/// component should be visible and accessible in the editor UI.
///
/// ## Placement
///
/// This plugin should be added early in the app initialization, before any systems that
/// depend on exposed components being available.
pub struct MeshflowVibeExposePlugin;

impl Plugin for MeshflowVibeExposePlugin {
    fn build(&self, app: &mut App) {
        register_exposed_types(app);
    }
}

/// Registers all component types that implement [`ExposeToEditor`] with the type registry.
///
/// This function retrieves the application's type registry and iterates through a predefined
/// list of Bevy components, marking each one as exposed to the editor if it exists in the registry.
fn register_exposed_types(app: &mut App) {
    let registry = app.world_mut().resource::<AppTypeRegistry>();
    let mut registry = registry.write();
    register_bevy_component::<bevy::core_pipeline::tonemapping::Tonemapping>(&mut registry);
}

/// Registers a Bevy component with the type registry as exposed to the editor.
///
/// This function checks if the given component type `T` exists in the type registry.
/// If it does, it inserts the [`ExposedToEditor`] marker with the read-only flag
/// determined by the component's implementation of [`ExposeToEditor::read_only`].
///
/// # Type Parameters
///
/// * `T` - The Bevy component type to register. Must implement both [`ExposeToEditor`]
///   and `std::any::Any`.
///
/// # Arguments
///
/// * `registry` - A mutable reference to the type registry where the component will be marked.
fn register_bevy_component<T: ExposeToEditor + std::any::Any>(registry: &mut TypeRegistry) {
    if let Some(reg) = registry.get_mut(std::any::TypeId::of::<T>()) {
        reg.insert(ExposedToEditor {
            read_only: T::read_only(),
        });
    };
}

/// Marker trait for components that should be exposed to the editor.
///
/// Components implementing this trait will be registered with the type system and marked
/// as accessible from the editor interface. This enables runtime inspection and editing
/// of component values without restarting the application.
///
/// # Implementing This Trait
///
/// When implementing `ExposeToEditor` for a component, you must specify whether the
/// component should be treated as read-only in the editor:
///
/// ```
/// use bevy::prelude::*;
///
/// trait ExposeToEditor {
///     fn read_only() -> bool;
/// }
///
/// struct MyComponent {
///     value: f32,
/// }
///
/// impl ExposeToEditor for MyComponent {
///     fn read_only() -> bool {
///         false // Allow editing in editor
///     }
/// }
/// ```
pub trait ExposeToEditor {
    /// Returns whether this component should be read-only in the editor.
    ///
    /// - `true`: The component can be inspected but not modified from the editor UI
    /// - `false`: The component can be both inspected and edited from the editor UI
    fn read_only() -> bool;
}

/// Implementation of [`ExposeToEditor`] for Bevy's
/// [`bevy::core_pipeline::tonemapping::Tonemapping`] component.
///
/// Tonemapping is exposed as editable, allowing developers to adjust tone mapping
/// settings in real-time from the editor to preview different visual effects.
impl ExposeToEditor for bevy::core_pipeline::tonemapping::Tonemapping {
    fn read_only() -> bool {
        false
    }
}
