use bevy::ecs::{
    component::{Component, HookContext},
    event::Events,
    world::DeferredWorld,
};

pub mod duplicate;
pub mod events;
pub mod manager;
pub mod plugin;
pub mod ray;

/// Just the active selection marker
#[derive(Component)]
#[component(on_add = ActiveSelection::on_add, on_remove = ActiveSelection::on_remove)]
pub struct ActiveSelection;

impl ActiveSelection {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        world
            .resource_mut::<Events<SpawnGizmoEvent>>()
            .send(SpawnGizmoEvent(ctx.entity));
    }
    fn on_remove(mut world: DeferredWorld, ctx: HookContext) {
        world
            .commands()
            .entity(ctx.entity)
            .despawn_related::<crate::gizmos::Gizmos>();
    }
}

/// ALL selection including the active selection marker
#[derive(Component)]
pub struct Selected;

pub use duplicate::{duplicate_all_selection_system, duplicate_entity_system};
pub use events::{
    RequestDeselectAllEntitiesEvent, RequestDeselectEntityEvent, RequestDuplicateAllSelectionEvent,
    RequestDuplicateEntityEvent, RequestSelectEntityEvent, RequestSelectEntityRangeEvent,
};
pub use manager::{
    apply_pending_parents, deselect_all_entities, deselect_all_entities_watcher,
    deselect_entity_watcher, handle_entity_selection, select_entity_range_watcher,
    select_entity_watcher,
};
pub use plugin::SelectionPlugin;
pub use ray::{RaycastCursorLast, RaycastCursorPos};

use crate::gizmos::{DespawnGizmoEvent, SpawnGizmoEvent};
