//! Procedural macros for the meshflow_vibe ecosystem.
//!
//! This crate provides three procedural macros for integrating Bevy ECS components with
//! the meshflow_vibe UI system:
//!
//! - [`granite_component`]: Attribute macro for defining Bevy components with automatic reflection
//!   and serialization support, plus automatic registration with the editor.
//! - [`register_editor_components!`]: Macro to register all components marked with
//!   `#[granite_component]` in an Bevy App.
//! - [`ui_callable_events`]: Attribute macro for marking structs as UI-callable event types,
//!   generating the necessary trait implementations for UI interaction.
//!
//! # Example
//!
//! ```rust,ignore
//! use meshflow_vibe_macros::granite_component;
//! use meshflow_vibe_macros::register_editor_components;
//! use meshflow_vibe_macros::ui_callable_events;
//!
//! #[granite_component]
//! pub struct MyComponent {
//!     pub value: f32,
//! }
//!
//! #[ui_callable_events]
//! pub struct MyEvent {
//!     pub data: String,
//! }
//!
//! pub fn setup(app: &mut bevy::app::App) {
//!     app.add_systems(Startup, register_editor_components!);
//! }
//! ```
//!
//! # Attributes for `granite_component`
//!
//! - `default`: Adds `Default` derive (default behavior).
//! - `ui_hidden`: Prevents the component from being registered as a bridge in the editor.
//!
//! # Usage Notes
//!
//! The macros automatically inject necessary imports for Bevy reflection and serialization.
//! These imports are added only once per compilation unit to avoid duplicate import warnings.

use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::quote;
use std::sync::Mutex;
use syn::{parse_macro_input, DeriveInput};

#[cfg(test)]
mod tests;

static REGISTERED_COMPONENTS: Lazy<Mutex<Vec<(String, bool)>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

use std::sync::atomic::{AtomicBool, Ordering};

static IMPORTS_ADDED: AtomicBool = AtomicBool::new(false);

/// Attribute macro for defining Bevy components with automatic reflection and serialization.
///
/// This macro automatically derives the following traits for your struct:
/// - `Reflect`: For runtime type information and reflection
/// - `Serialize`/`Deserialize`: For serialization support
/// - `Debug`: For debugging output
/// - `Clone`: For cloning instances
/// - `Component`: For Bevy ECS compatibility
/// - `PartialEq`: For equality comparison
/// - `Default`: Unless `#[granite_component(ui_hidden)]` is used
///
/// Additionally, it registers the component with meshflow_vibe's reflection system and
/// optionally registers it as a bridge tag for editor integration.
///
/// # Attributes
///
/// - `default` (implicit): Includes `Default` derive. Can be explicitly specified.
/// - `ui_hidden`: Prevents the component from being registered as a bridge in the editor.
///
/// # Example
///
/// ```rust,ignore
/// use meshflow_vibe_macros::granite_component;
///
/// #[granite_component]
/// pub struct Transform {
///     pub position: Vec3,
///     pub rotation: Quat,
/// }
///
/// // With hidden flag
/// #[granite_component(ui_hidden)]
/// pub struct InternalState {
///     pub data: usize,
/// }
/// ```
///
/// # Generated Code
///
/// For a struct `MyComponent`, this macro expands to:
/// ```rust,ignore
/// #[warn(unused_imports)]
/// use bevy::prelude::{Component, ReflectFromReflect, ReflectDefault, ...};
/// #[derive(Reflect, Serialize, Deserialize, Debug, Clone, Component, Default, PartialEq)]
/// #[reflect(Component, Serialize, Deserialize, Default, FromReflect)]
/// pub struct MyComponent { ... }
/// ```
#[proc_macro_attribute]
pub fn granite_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let name_str = name.to_string();
    println!("MACRO: Registering component: {}", name_str);
    let attr_str = attr.to_string();
    let include_default = attr_str.contains("default");
    let is_hidden = attr_str.contains("ui_hidden");
    REGISTERED_COMPONENTS
        .lock()
        .unwrap()
        .push((name_str.clone(), is_hidden));
    let derives = if include_default {
        quote! {
            #[derive(Reflect, Serialize, Deserialize, Debug, Clone, Component, PartialEq)]
        }
    } else {
        quote! {
            #[derive(Reflect, Serialize, Deserialize, Debug, Clone, Component, Default, PartialEq)]
        }
    };
    // Only add imports on the first use
    let needs_imports = !IMPORTS_ADDED.swap(true, Ordering::Relaxed);
    let imports = if needs_imports {
        quote! {
            #[warn(unused_imports)]
            use bevy::prelude::{Component,ReflectFromReflect, ReflectDefault, ReflectDeserialize, ReflectSerialize, ReflectComponent};
            #[warn(unused_imports)]
            use bevy::reflect::{Reflect, FromReflect};
            #[warn(unused_imports)]
            use serde::{Serialize, Deserialize};
        }
    } else {
        quote! {}
    };
    let expanded = quote! {
        #imports
        #derives
        #[reflect(Component, Serialize, Deserialize, Default, FromReflect)]
        #input
    };
    TokenStream::from(expanded)
}

/// Macro to register all components marked with `#[granite_component]` in an Bevy App.
///
/// This macro iterates over all components that have been decorated with
/// `#[granite_component]` during compilation and registers their types with the provided
/// Bevy App. Components marked with `ui_hidden` are registered only for type information,
/// while others are additionally registered with a `BridgeTag` for editor integration.
///
/// # Usage
///
/// Place this macro call in your Bevy App setup, typically in a `Startup` system or
/// directly on the `App` builder:
///
/// ```rust,ignore
/// use meshflow_vibe_macros::register_editor_components;
///
/// fn main() {
///     App::new()
///         .add_systems(Startup, register_editor_components!)
///         .run();
/// }
///
/// // Or with explicit app reference:
/// fn setup(app: &mut App) {
///     app.add_systems(Startup, |app: &mut App| {
///         register_editor_components!(app);
///     });
/// }
/// ```
///
/// # Behavior
///
/// For each `#[granite_component]` struct:
/// - Calls `app.register_type::<ComponentName>()` to enable reflection
/// - If not `ui_hidden`, also calls `app.register_type_data::<ComponentName, BridgeTag>()`
///
/// # Notes
///
/// - If no arguments are provided, the macro uses `app` as the default identifier
/// - The macro must be called after all components are defined
/// - Components are registered at runtime when the macro expands
#[proc_macro]
pub fn register_editor_components(input: TokenStream) -> TokenStream {
    let app_name = if input.is_empty() {
        quote!(app)
    } else {
        let parsed = parse_macro_input!(input as syn::Ident);
        quote!(#parsed)
    };

    let components = REGISTERED_COMPONENTS.lock().unwrap();
    let tokens = components.iter().map(|(name, is_hidden)| {
        let ident = syn::Ident::new(name, proc_macro2::Span::call_site());

        if *is_hidden {
            quote! {
                #app_name.register_type::<#ident>();
            }
        } else {
            quote! {
                #app_name.register_type::<#ident>();
                #app_name.register_type_data::<#ident, meshflow_vibe::prelude::BridgeTag>();
            }
        }
    });

    let expanded = quote! {
        {
            #(#tokens)*
        }
    };
    TokenStream::from(expanded)
}

/// Attribute macro for marking structs as UI-callable event types.
///
/// This macro implements the `UICallableEventMarker` and `UICallableEventProvider` traits
/// for your struct, enabling it to be triggered from the UI. It also generates a `register_ui()`
/// method that registers event senders with the meshflow_vibe system.
///
/// The macro automatically extracts field names and types from the struct to provide
/// type information for the UI system.
///
/// # Requirements
///
/// The decorated struct must:
/// - Be a struct (not an enum or union)
/// - Have named fields (tuple structs are not supported)
/// - Implement `Default` for event creation
///
/// # Example
///
/// ```rust,ignore
/// use meshflow_vibe_macros::ui_callable_events;
///
/// #[ui_callable_events]
/// pub struct RotateEvent {
///     pub angle: f32,
///     pub axis: Vec3,
/// }
///
/// // Usage in your app setup
/// pub fn setup() {
///     RotateEvent::register_ui();
/// }
/// ```
///
/// # Generated Implementations
///
/// For a struct `MyEvent` with fields `(field1: Type1, field2: Type2)`, this macro generates:
/// ```rust,ignore
/// impl meshflow_vibe_core::UICallableEventMarker for MyEvent {}
///
/// impl meshflow_vibe_core::UICallableEventProvider for MyEvent {
///     fn get_event_names() -> &'static [&'static str] {
///         &["field1", "field2"]
///     }
///
///     fn get_struct_name() -> &'static str {
///         "MyEvent"
///     }
/// }
///
/// impl MyEvent {
///     pub fn get_event_types() -> &'static [&'static str] {
///         &["Type1", "Type2"]
///     }
///
///     pub fn register_ui() {
///         // Registers event senders with meshflow_vibe
///     }
/// }
/// ```
///
/// # Notes
///
/// - The `register_ui()` method must be called during app initialization
/// - Event types must implement `Default`
/// - Field names are used as identifiers in the UI system
#[proc_macro_attribute]
pub fn ui_callable_events(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let name_str = name.to_string();

    // Extract field names and types from the struct
    let (field_names, field_types): (Vec<_>, Vec<_>) =
        if let syn::Data::Struct(ref data_struct) = input.data {
            if let syn::Fields::Named(ref fields_named) = data_struct.fields {
                fields_named
                    .named
                    .iter()
                    .map(|field| {
                        let field_name = field.ident.as_ref().unwrap().to_string();
                        let field_type = &field.ty;
                        (field_name, field_type.clone())
                    })
                    .unzip()
            } else {
                (Vec::new(), Vec::new())
            }
        } else {
            (Vec::new(), Vec::new())
        };

    // Generate event sender closures
    let event_senders = field_types.iter().map(|field_type| {
        quote! {
            Box::new(|world: &mut bevy::prelude::World| {
                if let Some(mut events) = world.get_resource_mut::<bevy::ecs::event::Events<#field_type>>() {
                    events.send(#field_type::default());
                }
            }) as Box<dyn Fn(&mut bevy::prelude::World) + Send + Sync>
        }
    });

    let expanded = quote! {
        #input

       impl meshflow_vibe_core::UICallableEventMarker for #name {}

        impl meshflow_vibe_core::UICallableEventProvider for #name {
            fn get_event_names() -> &'static [&'static str] {
                &[#(#field_names),*]
            }

            fn get_struct_name() -> &'static str {
                #name_str
            }
        }

        impl #name {
            pub fn get_event_types() -> &'static [&'static str] {
                &[#(stringify!(#field_types)),*]
            }

            pub fn register_ui() {
                let event_senders = vec![#(#event_senders),*];
                let event_names: &'static [&'static str] = &[#(#field_names),*];

                // Use the registration function - this will be provided by the user's import
                meshflow_vibe::prelude::register_ui_callable_events_with_senders(
                    #name_str,
                    event_names,
                    event_senders,
                );
            }
        }
    };

    TokenStream::from(expanded)
}
