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
#[require(Selected)]
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
#[derive(Component, Default)]
pub struct Selected;

pub use duplicate::{duplicate_all_selection_system, duplicate_entity_system};
pub use events::{EntityEvent, RequestDuplicateAllSelectionEvent, RequestDuplicateEntityEvent};
pub use manager::{apply_pending_parents, handle_picking_selection, select_entity};
pub use plugin::SelectionPlugin;
pub use ray::{RaycastCursorLast, RaycastCursorPos};

use crate::gizmos::SpawnGizmoEvent;
