use super::{
    RequestDeselectAllEntitiesEvent, RequestDeselectEntityEvent, RequestSelectEntityEvent,
};
use crate::{
    gizmos::{DespawnGizmoEvent, GizmoMesh, SelectedGizmo, SpawnGizmoEvent},
    selection::{
        ray::{raycast_at_cursor, HitType, RaycastCursorLast, RaycastCursorPos},
        ActiveSelection, RequestSelectEntityRangeEvent, Selected,
    },
};
use bevy::{
    ecs::query::Changed,
    prelude::{
        Added, Component, Entity, EventReader, EventWriter, Name, Query, RemovedComponents, Res,
        ResMut, With,
    },
};
use bevy::{ecs::system::Commands, picking::hover::PickingInteraction};
use bevy_granite_core::{IconProxy, UserInput};
use bevy_granite_logging::{
    config::{LogCategory, LogLevel, LogType},
    log,
};

pub fn apply_pending_parents(mut commands: Commands, query: Query<(Entity, &ParentTo)>) {
    for (entity, parent_to) in &query {
        if let Ok(mut parent) = commands.get_entity(parent_to.0) {
            parent.add_children(&[entity]);
            commands.entity(entity).remove::<ParentTo>();
        } else {
            log!(
                LogType::Editor,
                LogLevel::Critical,
                LogCategory::Entity,
                "Failed to parent entity {:?} to {:?}",
                entity,
                parent_to.0
            );
        }
    }
}

#[derive(Component)]
pub struct ParentTo(pub Entity);

// Used when Active Selection is no longer available
pub fn active_selected_removed_watcher(
    mut removed_selected: RemovedComponents<ActiveSelection>,
    mut despawn_gizmo_writer: EventWriter<DespawnGizmoEvent>,
    selected_gizmo: Res<SelectedGizmo>,
) {
    for _entity in removed_selected.read() {
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Entity,
            "Lost old active selection"
        );
        despawn_gizmo_writer.send(DespawnGizmoEvent(selected_gizmo.value));
    }
}

//  Used when the Active Selection becomes available
pub fn active_selected_watcher(
    active_selection: Query<Entity, (With<ActiveSelection>, Added<ActiveSelection>)>,
    mut gizmo_spawn_writer: EventWriter<SpawnGizmoEvent>,
) {
    for entity in active_selection.iter() {
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Entity,
            "Active selection found"
        );
        gizmo_spawn_writer.send(SpawnGizmoEvent(entity));
    }
}

// used when an entity is selected
pub fn select_entity_watcher(
    mut commands: Commands,
    selection: Query<Entity, With<Selected>>,
    active_selection: Query<Entity, With<ActiveSelection>>,
    mut select_entity_event: EventReader<RequestSelectEntityEvent>,
) {
    for RequestSelectEntityEvent { entity, additive } in select_entity_event.read() {
        if !additive {
            for selected_entity in selection.iter() {
                commands.entity(selected_entity).remove::<Selected>();
            }
            for active_entity in active_selection.iter() {
                commands.entity(active_entity).remove::<ActiveSelection>();
            }
        }

        if selection.get(*entity).is_ok() && active_selection.get(*entity).is_err() {
            commands.entity(*entity).insert(ActiveSelection);
            log!(
                LogType::Editor,
                LogLevel::Info,
                LogCategory::Entity,
                "Existing selected entity set as active"
            );
        }

        if *additive && active_selection.get(*entity).is_err() {
            for active_entity in active_selection.iter() {
                commands.entity(active_entity).remove::<ActiveSelection>();
            }
            commands.entity(*entity).insert(ActiveSelection);
            log!(
                LogType::Editor,
                LogLevel::Info,
                LogCategory::Entity,
                "New entity selected and set active"
            );
        }

        if selection.get(*entity).is_err() {
            commands
                .entity(*entity)
                .insert(Selected)
                .insert(ActiveSelection);
            log!(
                LogType::Editor,
                LogLevel::Info,
                LogCategory::Entity,
                "New entity selected and set active"
            );
        }
    }
}

// Used when we get a single entity deselected
pub fn deselect_entity_watcher(
    mut commands: Commands,
    selection: Query<Entity, With<Selected>>,
    active_selection: Query<Entity, With<ActiveSelection>>,
    mut events: EventReader<RequestDeselectEntityEvent>,
) {
    for RequestDeselectEntityEvent(entity) in events.read() {
        let was_active = active_selection.get(*entity).is_ok();

        if selection.get(*entity).is_ok() {
            commands.entity(*entity).remove::<Selected>();
            log!(
                LogType::Editor,
                LogLevel::Info,
                LogCategory::Entity,
                "Entity no longer selected!"
            );
        }

        if was_active {
            commands.entity(*entity).remove::<ActiveSelection>();

            if let Some(new_active) = selection.iter().find(|e| *e != *entity) {
                log!(
                    LogType::Editor,
                    LogLevel::Info,
                    LogCategory::Entity,
                    "New entity set as active!"
                );
                commands.entity(new_active).insert(ActiveSelection);
            }
        }
    }
}

pub fn deselect_all_entities_watcher(
    mut commands: Commands,
    selection: Query<Entity, With<Selected>>,
    active_selection: Query<Entity, With<ActiveSelection>>,
    mut events: EventReader<RequestDeselectAllEntitiesEvent>,
) {
    for _event in events.read() {
        deselect_all_entities(&mut commands, &selection, &active_selection);
    }
}

pub fn deselect_all_entities(
    commands: &mut Commands,
    selection: &Query<Entity, With<Selected>>,
    active_selection: &Query<Entity, With<ActiveSelection>>,
) {
    for entity in selection.iter() {
        commands.entity(entity).remove::<Selected>();
    }
    for entity in active_selection.iter() {
        commands.entity(entity).remove::<ActiveSelection>();
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Entity,
            "Deselected all entities!"
        );
    }
}

// System for entity interaction
pub fn handle_entity_selection(
    mut select_event_writer: EventWriter<RequestSelectEntityEvent>,
    mut deselect_event_writer: EventWriter<RequestDeselectAllEntitiesEvent>,
    active_selection: Query<Entity, With<ActiveSelection>>,
    user_input: Res<UserInput>,
    interaction: Query<
        (
            Entity,
            Option<&GizmoMesh>,
            Option<&IconProxy>,
            &Name,
            &PickingInteraction,
        ),
        Changed<PickingInteraction>,
    >,
    gizmo_filter: Query<(Entity, Option<&GizmoMesh>, Option<&IconProxy>, &Name)>,
    icon_proxy_query: Query<&IconProxy>,
    mut raycast_cursor_last_pos: ResMut<RaycastCursorLast>,
    mut raycast_cursor_pos: ResMut<RaycastCursorPos>,
) {
    if user_input.mouse_left.just_pressed && !user_input.mouse_right.pressed {
        if user_input.mouse_over_egui {
            return;
        }

        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Input,
            "Cursor over UI: {}",
            user_input.mouse_over_egui
        );

        let additive = user_input.shift_left.pressed;

        let (entity, hit_type) = raycast_at_cursor(interaction);

        if hit_type == HitType::Gizmo {
            return;
        }

        if let Some(entity) = entity {
            if hit_type == HitType::Icon || hit_type == HitType::Mesh {
                log!(
                    LogType::Editor,
                    LogLevel::Info,
                    LogCategory::Input,
                    "Ray hit: {} entity",
                    match hit_type {
                        HitType::Icon => "Icon",
                        HitType::Mesh => "Mesh",
                        _ => "Unknown",
                    }
                );

                if let Ok(active_entity) = active_selection.single() {
                    if active_entity == entity {
                        log!(
                            LogType::Editor,
                            LogLevel::Info,
                            LogCategory::Input,
                            "Entity already active, skipping event",
                        );
                        return;
                    }
                }

                // Check if the hit entity is an icon proxy and redirect to target entity
                let target_entity = if let Ok(icon_proxy) = icon_proxy_query.get(entity) {
                    log!(
                        LogType::Editor,
                        LogLevel::Info,
                        LogCategory::Input,
                        "Icon proxy hit, redirecting to target entity",
                    );
                    icon_proxy.target_entity
                } else {
                    entity
                };

                select_event_writer.write(RequestSelectEntityEvent {
                    entity: target_entity,
                    additive,
                });
                return;
            }
        }

        if hit_type == HitType::Void || hit_type == HitType::None {
            log!(
                LogType::Editor,
                LogLevel::Info,
                LogCategory::Input,
                "Could not find an entity, deselecting",
            );

            deselect_event_writer.write(RequestDeselectAllEntitiesEvent);
        }
    }
}

// used when a range of entities is selected
pub fn select_entity_range_watcher(
    mut commands: Commands,
    selection: Query<Entity, With<Selected>>,
    active_selection: Query<Entity, With<ActiveSelection>>,
    mut select_entity_range_event: EventReader<RequestSelectEntityRangeEvent>,
) {
    for RequestSelectEntityRangeEvent { entities, additive } in select_entity_range_event.read() {
        if !additive {
            for selected_entity in selection.iter() {
                commands.entity(selected_entity).remove::<Selected>();
            }
            for active_entity in active_selection.iter() {
                commands.entity(active_entity).remove::<ActiveSelection>();
            }
        }

        // Only the last entity in the range will be set as active
        let last_entity = entities.last().copied();
        for entity in entities {
            let is_selected = selection.get(*entity).is_ok();
            if !is_selected {
                commands.entity(*entity).insert(Selected);
            }
            if Some(*entity) == last_entity {
                // Remove ActiveSelection from all others
                for active_entity in active_selection.iter() {
                    commands.entity(active_entity).remove::<ActiveSelection>();
                }
                commands.entity(*entity).insert(ActiveSelection);
                log!(
                    LogType::Editor,
                    LogLevel::Info,
                    LogCategory::Entity,
                    "Entity {:?} set as active in range selection",
                    entity
                );
            }
        }
    }
}
