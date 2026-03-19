use super::{
    cache::update_entity_cache_system,
    events::{
        MaterialDeleteEvent, MaterialHandleUpdateEvent, PopupMenuRequestedEvent,
        RequestCameraEntityFrame, RequestEditorToggle, RequestNewParent, RequestRemoveChildren,
        RequestRemoveParents, RequestToggleCameraSync, RequestViewportCameraOverride,
        SetActiveWorld, UserRequestGraniteTypeViaPopup, UserUpdatedComponentsEvent,
        UserUpdatedIdentityEvent, UserUpdatedTransformEvent,
    },
    layout::dock_ui_system,
    popups::{handle_popup_requests_system, show_active_popups_system},
    tabs::{
        handle_material_deletion_system, send_queued_events_system, update_debug_tab_ui_system,
        update_editor_settings_tab_system, update_entity_editor_tab_system,
        update_entity_with_new_components_system, update_entity_with_new_identity_system,
        update_entity_with_new_transform_system, update_log_tab_system,
        update_material_handle_system, update_node_tree_tabs_system, RequestReparentEntityEvent,
    },
    BottomDockState, EntityUIDataCache, PopupState, RequestEnterEditMode, RequestExitEditMode,
    SideDockState,
};
use crate::{interface::RequestRemoveParentsFromEntities, setup::is_editor_active};
use bevy::ecs::message::MessageReader;
use bevy::{
    app::Update,
    ecs::schedule::IntoScheduleConfigs,
    prelude::{App, Handle, Mesh, Plugin, StandardMaterial},
};
use bevy_egui::EguiPrimaryContextPass;

pub struct InterfacePlugin;
impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // Interface events
            //
            .add_message::<MaterialHandleUpdateEvent>()
            .add_message::<MaterialDeleteEvent>()
            .add_message::<UserUpdatedComponentsEvent>()
            .add_message::<UserUpdatedTransformEvent>()
            .add_message::<UserUpdatedIdentityEvent>()
            .add_message::<UserRequestGraniteTypeViaPopup>()
            .add_message::<PopupMenuRequestedEvent>()
            .add_message::<RequestEditorToggle>()
            .add_message::<RequestCameraEntityFrame>()
            .add_message::<RequestToggleCameraSync>()
            .add_message::<RequestNewParent>()
            .add_message::<RequestRemoveChildren>()
            .add_message::<RequestRemoveParents>()
            .add_message::<SetActiveWorld>()
            .add_message::<RequestViewportCameraOverride>()
            // need to rework
            .add_message::<RequestReparentEntityEvent>()
            .add_message::<RequestRemoveParentsFromEntities>()
            // Edit mode events
            .add_message::<RequestEnterEditMode>()
            .add_message::<RequestExitEditMode>()
            //
            // Conversion systems for RequestEnterEditMode -> EnterEditMode
            //
            .add_systems(
                Update,
                convert_request_enter_edit_mode.run_if(is_editor_active),
            )
            .add_systems(
                Update,
                convert_request_exit_edit_mode.run_if(is_editor_active),
            )
            //
            // Register types
            // If you want to duplicate bevy data you must register the type
            //
            .register_type::<Handle<Mesh>>()
            .register_type::<Handle<StandardMaterial>>()
            //
            // Resources
            //
            .insert_resource(EntityUIDataCache::default())
            .insert_resource(PopupState::default())
            .insert_resource(SideDockState::default())
            .insert_resource(BottomDockState::default())
            //
            // Schedule systems
            //
            .add_systems(
                Update,
                (
                    //
                    // Handle UI requests to update entities
                    //
                    update_entity_with_new_components_system,
                    update_entity_with_new_transform_system,
                    update_entity_with_new_identity_system,
                    //
                    // Actual entity updates from UI changes
                    //
                    update_entity_cache_system,
                    update_material_handle_system,
                    handle_material_deletion_system,
                    //
                    // Layout and Popups
                    //
                    handle_popup_requests_system,
                    //
                    // Interface tabs UI
                    //
                    update_node_tree_tabs_system,
                    update_entity_editor_tab_system,
                    update_editor_settings_tab_system,
                    update_log_tab_system,
                    update_debug_tab_ui_system,
                    update_node_tree_tabs_system,
                )
                    .chain()
                    .run_if(is_editor_active),
            )
            .add_systems(
                EguiPrimaryContextPass,
                (show_active_popups_system, dock_ui_system).run_if(is_editor_active),
            )
            .add_systems(Update, send_queued_events_system.run_if(is_editor_active));
    }
}

fn convert_request_enter_edit_mode(
    mut editor_events: crate::interface::events::EditorEvents,
    mut request_events: MessageReader<RequestEnterEditMode>,
) {
    for event in request_events.read() {
        editor_events.enter_edit_mode.write(
            meshflow_vibe_core::entities::edit_mode::EnterEditMode {
                entity: event.entity,
                topology_id: event.topology_id,
            },
        );
    }
}

fn convert_request_exit_edit_mode(
    mut editor_events: crate::interface::events::EditorEvents,
    mut request_events: MessageReader<RequestExitEditMode>,
) {
    for _event in request_events.read() {
        editor_events
            .exit_edit_mode
            .write(meshflow_vibe_core::entities::edit_mode::ExitEditMode);
    }
}
