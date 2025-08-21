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
    ecs::{observer::Trigger, query::Changed, world::OnAdd},
    prelude::{
        Added, Component, Entity, EventReader, EventWriter, Name, Query, RemovedComponents, Res,
        ResMut, With,
    },
};
use bevy::{
    ecs::{query::QueryEntityError, system::Commands},
    picking::{
        events::{Click, Pointer},
        hover::PickingInteraction,
    },
};
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

// // System for entity interaction
// pub fn handle_entity_selection(
//     mut select_event_writer: EventWriter<RequestSelectEntityEvent>,
//     mut deselect_event_writer: EventWriter<RequestDeselectAllEntitiesEvent>,
//     active_selection: Query<Entity, With<ActiveSelection>>,
//     user_input: Res<UserInput>,
//     interaction: Query<
//         (
//             Entity,
//             Option<&GizmoMesh>,
//             Option<&IconProxy>,
//             &Name,
//             &PickingInteraction,
//         ),
//         Changed<PickingInteraction>,
//     >,
//     gizmo_filter: Query<(Entity, Option<&GizmoMesh>, Option<&IconProxy>, &Name)>,
//     icon_proxy_query: Query<&IconProxy>,
//     mut raycast_cursor_last_pos: ResMut<RaycastCursorLast>,
//     mut raycast_cursor_pos: ResMut<RaycastCursorPos>,
// ) {
//     if user_input.mouse_left.just_pressed && !user_input.mouse_right.pressed {
//         if user_input.mouse_over_egui {
//             return;
//         }

//         log!(
//             LogType::Editor,
//             LogLevel::Info,
//             LogCategory::Input,
//             "Cursor over UI: {}",
//             user_input.mouse_over_egui
//         );

//         let additive = user_input.shift_left.pressed;

//         let (entity, hit_type) = raycast_at_cursor(interaction);

//         if hit_type == HitType::Gizmo {
//             return;
//         }
//         log(
//             LogType::Editor,
//             LogLevel::Info,
//             LogCategory::Debug,
//             "Its Here".into(),
//         );

//         if let Some(entity) = entity {
//             if hit_type == HitType::Icon || hit_type == HitType::Mesh {
//                 log!(
//                     LogType::Editor,
//                     LogLevel::Info,
//                     LogCategory::Input,
//                     "Ray hit: {} entity",
//                     match hit_type {
//                         HitType::Icon => "Icon",
//                         HitType::Mesh => "Mesh",
//                         _ => "Unknown",
//                     }
//                 );

//                 if let Ok(active_entity) = active_selection.single() {
//                     if active_entity == entity {
//                         log!(
//                             LogType::Editor,
//                             LogLevel::Info,
//                             LogCategory::Input,
//                             "Entity already active, skipping event",
//                         );
//                         return;
//                     }
//                 }

//                 // Check if the hit entity is an icon proxy and redirect to target entity
//                 let target_entity = if let Ok(icon_proxy) = icon_proxy_query.get(entity) {
//                     log!(
//                         LogType::Editor,
//                         LogLevel::Info,
//                         LogCategory::Input,
//                         "Icon proxy hit, redirecting to target entity",
//                     );
//                     icon_proxy.target_entity
//                 } else {
//                     entity
//                 };

//                 select_event_writer.write(RequestSelectEntityEvent {
//                     entity: target_entity,
//                     additive,
//                 });
//                 return;
//             }
//         }

//         if hit_type == HitType::Void || hit_type == HitType::None {
//             log!(
//                 LogType::Editor,
//                 LogLevel::Info,
//                 LogCategory::Input,
//                 "Could not find an entity, deselecting",
//             );

//             deselect_event_writer.write(RequestDeselectAllEntitiesEvent);
//         }
//     }
// }

/// when an entity gets ActiveSelection added to it we check if there is already an entity with ActiveSelection
/// if there is, we remove it for the other entity
pub fn single_active_observer(
    mut add_active: Trigger<OnAdd, ActiveSelection>,
    active_selection: Query<Entity, With<ActiveSelection>>,
    mut commands: Commands,
) {
    add_active.propagate(false);
    if active_selection.single().is_err() {
        for entity in &active_selection {
            log(
                LogType::Editor,
                LogLevel::Info,
                LogCategory::Input,
                format!("Entity {} is no longer active", entity.index()),
            );
            if entity != add_active.target() {
                commands.entity(entity).remove::<ActiveSelection>();
            }
        }
    }
}

pub fn handle_entity_selection(
    mut on_click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    selected: Query<Entity, With<Selected>>,
    active_selection: Query<Entity, With<ActiveSelection>>,
    ignored: Query<(), With<crate::utils::EditorIgnore>>,
) {
    if on_click.button != bevy::picking::pointer::PointerButton::Primary {
        return;
    }
    match ignored.get(on_click.target()) {
        Ok(_) => {
            log!(
                "ignoring EditorIgnore entity: {}",
                on_click.target().index()
            );
            return;
        }
        Err(QueryEntityError::EntityDoesNotExist(_)) => {
            log!("Entity does not exist: {}", on_click.target().index());
            return;
        }
        Err(_) => {}
    }
    if on_click.target().index() == 0 {
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Input,
            "Clicked on window?, ignoring"
        );
        return;
    }
    on_click.propagate(false);
    let entity = on_click.target();
    if active_selection.get(entity).is_ok() {
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Input,
            "Deselecting Entity {}",
            entity.index()
        );
        commands
            .entity(entity)
            .remove::<(ActiveSelection, Selected)>();
    }
    if selected.get(on_click.target()).is_ok() {
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Input,
            "Retargeting Entity {}",
            entity.index()
        );
        commands.entity(entity).insert(ActiveSelection);
    } else {
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Input,
            "Selecting Entity {}",
            entity.index()
        );
        commands.entity(entity).insert((Selected, ActiveSelection));
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
