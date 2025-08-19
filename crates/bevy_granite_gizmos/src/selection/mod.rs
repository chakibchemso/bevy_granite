use bevy::ecs::component::Component;

pub mod duplicate;
pub mod manager;
pub mod plugin;
pub mod ray;
pub mod events;

/// Just the active selection marker
#[derive(Component)]
pub struct ActiveSelection;

/// ALL selection including the active selection marker
#[derive(Component)]
pub struct Selected;

pub use duplicate::{duplicate_all_selection_system, duplicate_entity_system};
pub use manager::{
    active_selected_removed_watcher, active_selected_watcher, apply_pending_parents,
    deselect_all_entities_watcher, deselect_entity_watcher, handle_entity_selection,
    select_entity_watcher, deselect_all_entities, select_entity_range_watcher
};
pub use plugin::SelectionPlugin;
pub use ray::{RaycastCursorLast, RaycastCursorPos};
pub use events::{
    RequestDeselectAllEntitiesEvent, RequestDeselectEntityEvent, RequestDuplicateAllSelectionEvent,
    RequestDuplicateEntityEvent, RequestSelectEntityEvent, RequestSelectEntityRangeEvent
};