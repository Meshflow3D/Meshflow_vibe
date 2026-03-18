pub mod cameras;
pub mod entities;
pub mod lights;
pub mod relationships;
pub mod selection;

use bevy::{gizmos::config::GizmoConfigGroup, reflect::Reflect};

// Renamed from CustomSelectionGizmos to be more descriptive
#[derive(GizmoConfigGroup, Default, Reflect)]
pub struct SelectionRenderer;

// Renamed from CustomDebugGizmos to be more descriptive
#[derive(GizmoConfigGroup, Default, Reflect)]
pub struct DebugRenderer;

// Re-export all debug visualization functions
pub use cameras::*;
pub use entities::*;
pub use lights::*;
pub use relationships::*;
pub use selection::*;
