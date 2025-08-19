use super::{
    active_selected_removed_watcher, active_selected_watcher, apply_pending_parents,
    deselect_all_entities_watcher, deselect_entity_watcher, duplicate_all_selection_system,
    duplicate_entity_system, handle_entity_selection, select_entity_range_watcher,
    select_entity_watcher, RaycastCursorLast, RaycastCursorPos, RequestDeselectAllEntitiesEvent,
    RequestDeselectEntityEvent, RequestDuplicateAllSelectionEvent, RequestDuplicateEntityEvent,
    RequestSelectEntityEvent, RequestSelectEntityRangeEvent,
};
use crate::is_gizmos_active;
use bevy::{
    app::{App, Plugin, PostUpdate, Update},
    ecs::schedule::IntoSystemConfigs,
    math::Vec3,
};

pub struct SelectionPlugin;
impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // Events
            //
            .add_event::<RequestSelectEntityEvent>()
            .add_event::<RequestSelectEntityRangeEvent>()
            .add_event::<RequestDeselectEntityEvent>()
            .add_event::<RequestDuplicateEntityEvent>()
            .add_event::<RequestDuplicateAllSelectionEvent>()
            .add_event::<RequestDeselectAllEntitiesEvent>()
            //
            // Resources
            //
            .insert_resource(RaycastCursorLast {
                position: Vec3::ZERO,
            })
            .insert_resource(RaycastCursorPos {
                position: Vec3::ZERO,
            })
            //
            // Events
            //
            //
            // Schedule system
            //
            .add_systems(
                Update,
                (
                    select_entity_watcher,
                    handle_entity_selection,
                    active_selected_removed_watcher,
                    select_entity_range_watcher,
                    active_selected_watcher.after(active_selected_removed_watcher),
                    deselect_all_entities_watcher,
                    deselect_entity_watcher,
                    duplicate_entity_system.after(handle_entity_selection),
                    duplicate_all_selection_system.after(handle_entity_selection),
                )
                    .run_if(is_gizmos_active),
            )
            .add_systems(PostUpdate, (apply_pending_parents).run_if(is_gizmos_active));
    }
}
