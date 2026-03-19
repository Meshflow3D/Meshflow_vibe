//! Plugin for edit mode session management.

use bevy::{
    app::{App, Plugin, Update},
    ecs::message::MessageReader,
    prelude::{Commands, Entity, IntoScheduleConfigs, Query, Res, ResMut, With},
};

use super::{
    EditSession, EnterEditMode, ExitEditMode, InEditMode, ModeSelectionState, OnEnterEditMode,
    OnExitEditMode, OnSubSelectionModeSwitched, SubSelectionMode, SwitchSubSelectionMode,
};
use crate::entities::{EditableTopologyRegistry, TopologyOwner};

/// Plugin for edit mode session management.
///
/// This plugin provides:
/// - Edit mode state management via `EditSession` resource
/// - Eligibility checking against topology ownership contract
/// - Automatic cleanup of edit-only state on mode exit
pub struct EditModePlugin;

impl Plugin for EditModePlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // Resources
            //
            .insert_resource(EditSession::default())
            .insert_resource(SubSelectionMode::default())
            //
            // Events
            //
            .add_message::<EnterEditMode>()
            .add_message::<ExitEditMode>()
            .add_message::<OnEnterEditMode>()
            .add_message::<OnExitEditMode>()
            .add_message::<SwitchSubSelectionMode>()
            .add_message::<OnSubSelectionModeSwitched>()
            //
            // Schedule systems
            //
            .add_systems(Update, handle_enter_edit_mode)
            .add_systems(Update, handle_exit_edit_mode)
            .add_systems(
                Update,
                handle_sub_selection_mode_switch.run_if(in_edit_mode),
            )
            .add_systems(
                Update,
                handle_sub_selection_mode_cleanup.run_if(in_edit_mode),
            )
            //
            // Systems
            //
            .add_systems(Update, cleanup_stale_edit_session.run_if(in_edit_mode));
    }
}

/// System to handle entering edit mode.
///
/// Checks eligibility against the topology ownership contract:
/// - Entity must have `TopologyOwner` component
/// - Topology ID must be valid (not dummy)
/// - Topology must exist in `EditableTopologyRegistry`
///
/// If eligible, adds `InEditMode` marker component to trigger cleanup callbacks.
/// If a session is already active, exits it properly first.
fn handle_enter_edit_mode(
    mut commands: Commands,
    mut edit_session: ResMut<EditSession>,
    topology_registry: Res<EditableTopologyRegistry>,
    owner_query: Query<&TopologyOwner>,
    mut enter_events: MessageReader<EnterEditMode>,
    mut on_enter: bevy::ecs::message::MessageWriter<OnEnterEditMode>,
    mut on_exit: bevy::ecs::message::MessageWriter<OnExitEditMode>,
) {
    for event in enter_events.read() {
        let entity = event.entity;

        // Check if a session is already active
        if edit_session.is_active() {
            if let Some(active_entity) = edit_session.active_entity() {
                on_exit.write(OnExitEditMode(active_entity));
                commands.entity(active_entity).remove::<InEditMode>();
            }
            edit_session.exit();
        }

        // Check eligibility
        let Ok(owner) = owner_query.get(entity) else {
            continue; // Entity doesn't have TopologyOwner
        };

        // Verify topology ID matches
        if owner.topology_id != event.topology_id {
            continue;
        }

        // Verify topology exists in registry
        if !topology_registry.contains(event.topology_id) {
            continue;
        }

        // Eligible - enter edit mode
        edit_session.enter(entity, event.topology_id);
        commands.entity(entity).insert(InEditMode);
        on_enter.write(OnEnterEditMode(entity));
    }
}

/// System to handle exiting edit mode.
///
/// Can be triggered by:
/// - Explicit `ExitEditMode` event
/// - Despawn of the in-edit-mode entity (via InEditMode::on_remove)
fn handle_exit_edit_mode(
    mut commands: Commands,
    mut edit_session: ResMut<EditSession>,
    mut exit_events: MessageReader<ExitEditMode>,
    mut on_exit: bevy::ecs::message::MessageWriter<OnExitEditMode>,
) {
    for _event in exit_events.read() {
        if let Some(edit_entity) = edit_session.active_entity() {
            on_exit.write(OnExitEditMode(edit_entity));
            commands.entity(edit_entity).remove::<InEditMode>();
        }
        edit_session.exit();
    }
}

/// Condition to check if currently in edit mode.
pub fn in_edit_mode(edit_session: Res<EditSession>) -> bool {
    edit_session.is_active()
}

/// System to handle sub-selection mode switching.
///
/// Clears incompatible selections and visuals when switching modes.
/// Fires `OnSubSelectionModeSwitched` observer event for cleanup.
fn handle_sub_selection_mode_switch(
    _commands: Commands,
    edit_session: Res<EditSession>,
    mut sub_selection: ResMut<SubSelectionMode>,
    mut switch_events: bevy::ecs::message::MessageReader<SwitchSubSelectionMode>,
    mut on_switch: bevy::ecs::message::MessageWriter<super::OnSubSelectionModeSwitched>,
) {
    for event in switch_events.read() {
        let entity = edit_session
            .active_entity()
            .expect("Mode switch should only happen in edit mode");
        let old_mode = sub_selection.switch(event.mode);

        if old_mode != event.mode {
            on_switch.write(super::OnSubSelectionModeSwitched {
                entity,
                old_mode,
                new_mode: event.mode,
            });
        }
    }
}

/// System to handle cleanup when sub-selection mode is switched.
///
/// Removes `ModeSelectionState` components when mode changes:
/// - Cleans up stale selections and visuals
/// - Ensures mode-specific resources are properly released
fn handle_sub_selection_mode_cleanup(
    mut commands: Commands,
    query: Query<(Entity, &ModeSelectionState), With<InEditMode>>,
    _sub_selection: ResMut<SubSelectionMode>,
    mut switch_events: bevy::ecs::message::MessageReader<SwitchSubSelectionMode>,
) {
    for event in switch_events.read() {
        // Clean up stale ModeSelectionState components from old mode
        for (entity, mode_state) in query.iter() {
            if mode_state.mode != event.mode {
                commands.entity(entity).remove::<ModeSelectionState>();
            }
        }
    }
}

/// Detect and clear stale edit sessions when the active entity is despawned.
///
/// This system runs every frame while in edit mode and checks if the active
/// entity still exists in the ECS. If the entity was despawned, the session
/// is cleared to prevent stale state.
fn cleanup_stale_edit_session(
    mut edit_session: ResMut<EditSession>,
    mut on_exit: bevy::ecs::message::MessageWriter<OnExitEditMode>,
    active_query: Query<(), With<InEditMode>>,
) {
    // Try to get the active entity and check if it still has InEditMode
    if let Some(active_entity) = edit_session.active_entity() {
        // If we can't get a component from the active entity, it's despawned
        if active_query.get(active_entity).is_err() {
            on_exit.write(OnExitEditMode(active_entity));
            edit_session.exit();
        }
    }
}
