use super::{draw_axis_line, TransformGizmo};
use crate::{
    gizmos::{
        GizmoMesh, GizmoOf, GizmoSnap, GizmoType, SelectedGizmo, TransformDraggingEvent,
        TransformInitDragEvent, TransformResetDragEvent,
    },
    input::{DragState, GizmoAxis},
    selection::{
        ray::{raycast_at_cursor, HitType, RaycastCursorLast, RaycastCursorPos},
        ActiveSelection, RequestDuplicateAllSelectionEvent, Selected,
    },
    GizmoCamera,
};
use bevy::{
    ecs::{
        observer::Trigger,
        query::Changed,
        system::{Local, Single},
    },
    math::Vec2,
    picking::{
        events::{Drag, DragEntry, Pointer},
        hover::PickingInteraction,
    },
    prelude::{
        ChildOf, Children, Entity, EventReader, EventWriter, Gizmos, GlobalTransform, Name,
        ParamSet, Query, Res, ResMut, Transform, Vec3, With, Without,
    },
    window::Window,
    winit::cursor,
};
use bevy_granite_core::{mouse_to_world_delta, CursorWindowPos, IconProxy, UserInput};
use bevy_granite_logging::{
    config::{LogCategory, LogLevel, LogType},
    log,
};

// TODO:
// Watch for left CTRL just pressed, if so, move camera with transform

// FIX:
// Still need to create helper functions
// ------------------------------------------------------------------------
//
type CameraQuery<'w, 's> = Query<'w, 's, &'w Transform, With<GizmoCamera>>;
type ActiveSelectionQuery<'w, 's> = Query<'w, 's, Entity, With<ActiveSelection>>;
type TransformGizmoQuery<'w, 's> =
    Query<'w, 's, (Entity, &'w GizmoAxis, &'w ChildOf), With<TransformGizmo>>;

type NonActiveSelectionQuery<'w, 's> =
    Query<'w, 's, Entity, (With<Selected>, Without<ActiveSelection>)>;
type TransformQuery<'w, 's> =
    Query<'w, 's, (&'w mut Transform, &'w GlobalTransform, Entity), Without<GizmoCamera>>;
type GizmoMeshNameQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        Option<&'w GizmoMesh>,
        Option<&'w IconProxy>,
        &'w Name,
    ),
>;
type ParentQuery<'w, 's> = Query<'w, 's, &'w ChildOf>;
type ChildrenQuery<'w, 's> = Query<'w, 's, &'w Children>;
// ------------------------------------------------------------------------

/// Recursively checks if an entity is a descendant of a parent through its children
fn is_descendant(target: Entity, children: &Children, children_query: &Query<&Children>) -> bool {
    for child in children.iter() {
        if *child == target {
            return true;
        }

        // Recursively check grandchildren
        if let Ok(grandchildren) = children_query.get(*child) {
            if is_descendant(target, grandchildren, children_query) {
                return true;
            }
        }
    }
    false
}

//
// ------------------------------------------------------------------------

pub fn handle_transform_input(
    mut drag_state: ResMut<DragState>,
    selected_option: ResMut<SelectedGizmo>,
    user_input: Res<UserInput>,
    selection_query: ActiveSelectionQuery,
    mut gizmos: Gizmos,
    mut init_drag_event: EventWriter<TransformInitDragEvent>,
    mut dragging_event: EventWriter<TransformDraggingEvent>,
    mut drag_ended_event: EventWriter<TransformResetDragEvent>,
) {
    return;
    if !user_input.mouse_left.any {
        return;
    }

    if !matches!(selected_option.value, GizmoType::Transform) {
        // Gizmo value for Transform
        return;
    }

    let selection_entity = selection_query.get_single().ok();
    if selection_entity.is_none() {
        return;
    }

    // Setup drag
    if user_input.mouse_left.just_pressed && !drag_state.dragging && !user_input.mouse_over_egui {
        init_drag_event.send(TransformInitDragEvent);
    }
    // Dragging
    else if user_input.mouse_left.pressed && drag_state.dragging {
        draw_axis_line(
            &mut gizmos,
            drag_state.locked_axis,
            &Transform::from_translation(drag_state.raycast_position),
        );
        dragging_event.send(TransformDraggingEvent);
    }
    // Reset Drag
    else if user_input.mouse_left.just_released && drag_state.dragging {
        drag_ended_event.send(TransformResetDragEvent);
    }

    if !drag_state.dragging && !drag_state.drag_ended {
        drag_state.drag_ended = true;
        log!(
            LogType::Editor,
            LogLevel::Error,
            LogCategory::Input,
            "Drag was never reset, this should never happen!"
        );
    }
}

pub fn handle_init_transform_drag(
    mut events: EventReader<TransformInitDragEvent>,
    mut drag_state: ResMut<DragState>,
    resources: (
        Res<CursorWindowPos>,
        ResMut<RaycastCursorLast>,
        ResMut<RaycastCursorPos>,
    ),
    mut duplicate_event_writer: EventWriter<RequestDuplicateAllSelectionEvent>,
    user_input: Res<UserInput>,
    interactions: Query<
        (
            Entity,
            Option<&GizmoMesh>,
            Option<&IconProxy>,
            &Name,
            &PickingInteraction,
        ),
        Changed<PickingInteraction>,
    >,
    mut queries: ParamSet<(
        ActiveSelectionQuery,
        TransformGizmoQuery,
        ParentQuery,
        TransformQuery,
        GizmoMeshNameQuery,
    )>,
) {
    return;
    let (cursor_2d, mut raycast_cursor_last_pos, mut raycast_cursor_pos) = resources;
    for TransformInitDragEvent in events.read() {
        let (entity, hit_type) = raycast_at_cursor(interactions);

        if hit_type == HitType::None
            || hit_type == HitType::Icon
            || hit_type == HitType::Mesh
            || entity.is_none()
        {
            return;
        }

        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Blank,
            "------------------------"
        );
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Input,
            "Init transform drag event",
        );

        let selection_query = queries.p0();
        let selection_entity = selection_query.get_single().ok();

        let raycast_target = entity;
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Input,
            "Just pressed 'Left' and not dragging"
        );

        if let Some(entity) = raycast_target {
            let (gizmo_axis, actual_parent) =
                if let Ok((_gizmo_entity, gizmo_axis, gizmo_parent)) = queries.p1().get(entity) {
                    (*gizmo_axis, gizmo_parent.get())
                } else {
                    return;
                };

            {
                let mut query_p3 = queries.p3();
                if let Ok((_parent_transform, parent_global_transform, _)) =
                    query_p3.get_mut(actual_parent)
                {
                    // Use the entity's own global position, not the root parent's
                    drag_state.raycast_position = parent_global_transform.translation();
                    drag_state.initial_cursor_position = cursor_2d.position;
                    drag_state.dragging = true;
                    drag_state.drag_ended = false;
                    drag_state.locked_axis = Some(gizmo_axis);
                }
                log!(
                    LogType::Editor,
                    LogLevel::Info,
                    LogCategory::Entity,
                    "Drag start position: {}",
                    drag_state.raycast_position
                );
            }

            log!(
                LogType::Editor,
                LogLevel::Info,
                LogCategory::Input,
                "Begin dragging on axis: {:?}",
                drag_state.locked_axis
            );

            if user_input.shift_left.pressed {
                if let Some(_entity) = selection_entity {
                    log!(
                        LogType::Editor,
                        LogLevel::Info,
                        LogCategory::Input,
                        "Duplicate entity"
                    );
                    duplicate_event_writer.send(RequestDuplicateAllSelectionEvent);
                }
            }
        } else {
            log!(
                LogType::Editor,
                LogLevel::Warning,
                LogCategory::Input,
                "Trying to transform drag, but couldn't find the raycast entity"
            );
        }
    }
}

pub fn handle_transform_dragging(
    mut events: EventReader<TransformDraggingEvent>,
    resources: (
        Res<CursorWindowPos>,
        Res<GizmoSnap>,
        ResMut<DragState>,
        Res<SelectedGizmo>,
    ),
    mut queries: ParamSet<(
        CameraQuery,
        ActiveSelectionQuery,
        NonActiveSelectionQuery,
        TransformQuery,
        ChildrenQuery,
        ParentQuery,
    )>,
) {
    return;
    let (cursor_2d, gizmo_snap, drag_state, selected_gizmo) = resources;

    for TransformDraggingEvent in events.read() {
        if drag_state.drag_ended {
            return;
        }

        let gizmo_distance_scale = selected_gizmo.speed_scale;

        //log!(LogType::Editor,
        //    LogLevel::Info,
        //    LogCategory::Listener,
        //    "Transform dragging event",
        //);

        let selection_entity = queries.p1().get_single().ok();
        let speed = 9.0 * gizmo_distance_scale;

        if let Ok(camera_transform) = queries.p0().get_single() {
            let plane_normal = match drag_state.locked_axis {
                Some(GizmoAxis::X) => Some(Vec3::X),
                Some(GizmoAxis::Y) => Some(Vec3::Y),
                Some(GizmoAxis::Z) => Some(Vec3::Z),
                _ => None,
            };

            let world_delta = mouse_to_world_delta(
                cursor_2d.position,
                drag_state.initial_cursor_position,
                camera_transform,
                plane_normal,
            ) * speed;

            let target_delta = match drag_state.locked_axis {
                Some(GizmoAxis::X) => Vec3::new(world_delta.x, 0.0, 0.0),
                Some(GizmoAxis::Y) => Vec3::new(0.0, world_delta.y, 0.0),
                Some(GizmoAxis::Z) => Vec3::new(0.0, 0.0, world_delta.z),
                _ => world_delta,
            };

            let snap_value = gizmo_snap.transform_value;
            let snap = |value: f32| {
                if snap_value == 0.0 {
                    value
                } else {
                    (value / snap_value).round() * snap_value
                }
            };

            if let Some(selection_entity) = selection_entity {
                // Collect all selected entities (both active and non-active)
                let mut selected_entities: Vec<Entity> = queries.p2().iter().collect();
                selected_entities.push(selection_entity); // Add the active selection

                // Calculate snapped world position from raycast position and delta
                let snapped_world_position = Vec3::new(
                    snap(drag_state.raycast_position.x + target_delta.x),
                    snap(drag_state.raycast_position.y + target_delta.y),
                    snap(drag_state.raycast_position.z + target_delta.z),
                );

                // Get the world movement from the active selection
                let world_movement = {
                    if let Ok((_selection_transform, selection_global_transform, _)) =
                        queries.p3().get(selection_entity)
                    {
                        let original_global_position = selection_global_transform.translation();
                        snapped_world_position - original_global_position
                    } else {
                        return; // Can't get selection entity data
                    }
                };

                // Find root entities (entities that are not descendants of any other selected entity)
                let children_query = queries.p4();
                let mut root_entities = Vec::new();

                for &entity in &selected_entities {
                    let mut is_root = true;

                    // Check if this entity is a descendant of any other selected entity
                    for &other_entity in &selected_entities {
                        if entity == other_entity {
                            continue;
                        }

                        if let Ok(children) = children_query.get(other_entity) {
                            if is_descendant(entity, children, &children_query) {
                                is_root = false;
                                break;
                            }
                        }
                    }

                    if is_root {
                        root_entities.push(entity);
                    }
                }

                // Apply world movement to all root entities
                // Their descendants will move automatically via hierarchy
                let mut transform_query = queries.p3();
                for entity in root_entities {
                    if let Ok((mut transform, global_transform, _)) =
                        transform_query.get_mut(entity)
                    {
                        // Convert world movement to this entity's local space
                        let global_rotation = global_transform.to_scale_rotation_translation().1;
                        let local_rotation = transform.rotation;
                        let parent_rotation = global_rotation * local_rotation.inverse();
                        let local_delta = parent_rotation.inverse() * world_movement;

                        transform.translation += local_delta;
                    }
                }
            }
        }
    }
}

pub fn handle_transform_reset(
    mut events: EventReader<TransformResetDragEvent>,
    mut drag_state: ResMut<DragState>,
    selection_query: Query<Entity, With<ActiveSelection>>,
    transform_query: Query<(&mut Transform, &GlobalTransform, Entity), Without<GizmoCamera>>,
) {
    for TransformResetDragEvent in events.read() {
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Input,
            "Transform drag reset event",
        );

        drag_state.dragging = false;
        drag_state.locked_axis = None;
        drag_state.drag_ended = true;

        let mut final_position = None;
        if let Some(selection_entity) = selection_query.iter().next() {
            if let Ok((_selection_transform, selection_global_transform, _)) =
                transform_query.get(selection_entity)
            {
                final_position = Some(selection_global_transform.translation());
                log!(
                    LogType::Editor,
                    LogLevel::Info,
                    LogCategory::Entity,
                    "Ended dragging at: {}",
                    selection_global_transform.translation()
                );
            }
        } else {
            log!(
                LogType::Editor,
                LogLevel::Warning,
                LogCategory::Entity,
                "No queries found, will not grab  final position!"
            );
        }

        if let Some(position) = final_position {
            drag_state.raycast_position = position;
        } else {
            log!(
                LogType::Editor,
                LogLevel::Warning,
                LogCategory::Entity,
                "No final position!"
            );
        }

        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Input,
            "Finished dragging"
        );
    }
}

pub fn drag_transform_gizmo(
    event: Trigger<Pointer<Drag>>,
    targets: Query<&GizmoOf>,
    camera_query: Query<(&GlobalTransform, &bevy::render::camera::Camera), With<GizmoCamera>>,
    mut objects: Query<&mut Transform, Without<GizmoCamera>>,
    gizmo_snap: Res<GizmoSnap>,
    gizmo_data: Query<&GizmoAxis>,
) {
    let Ok(axis) = gizmo_data.get(event.target) else {
        log!(
            LogType::Editor,
            LogLevel::Warning,
            LogCategory::Input,
            "Gizmo Axis data not found for Gizmo entity {:?}",
            event.target
        );
        return;
    };

    let Ok((camera_transform, camera)) = camera_query.single() else {
        log! {
            LogType::Editor,
            LogLevel::Error,
            LogCategory::Input,
            "Gizmo camera not found",
        };
        return;
    };

    let Ok(GizmoOf(target)) = targets.get(event.target) else {
        log! {
            LogType::Editor,
            LogLevel::Error,
            LogCategory::Input,
            "Gizmo target not found for entity {:?}",
            event.target
        };
        return;
    };
    let Ok(click_ray) = camera.viewport_to_world(camera_transform, event.pointer_location.position)
    else {
        log! {
            LogType::Editor,
            LogLevel::Error,
            LogCategory::Input,
            "Failed to convert viewport to world coordinates for pointer location: {:?}",
            event.pointer_location.position
        };
        return;
    };

    let Ok(mut target_transform) = objects.get_mut(*target) else {
        log! {
            LogType::Editor,
            LogLevel::Error,
            LogCategory::Input,
            "Gizmo target transform not found for entity {:?}",
            target
        };
        return;
    };

    match axis {
        GizmoAxis::None => {}
        GizmoAxis::X => {
            let Some(click_distance) = click_ray.intersect_plane(
                Vec3::new(0., target_transform.translation.y, 0.),
                bevy::math::primitives::InfinitePlane3d::new(Vec3::Y),
            ) else {
                return;
            };
            let hit = camera_transform.translation() - (click_ray.direction * -click_distance);
            target_transform.translation.x = snap_gizmo(hit.x, gizmo_snap.transform_value);
        }
        GizmoAxis::Y => {
            let mut normal = camera_transform.forward().as_vec3();
            normal.y = 0.0;
            let Some(click_distance) = click_ray.intersect_plane(
                Vec3::new(
                    target_transform.translation.x,
                    0.,
                    target_transform.translation.z,
                ),
                bevy::math::primitives::InfinitePlane3d::new(normal.normalize()),
            ) else {
                return;
            };
            let hit = camera_transform.translation() - (click_ray.direction * -click_distance);
            target_transform.translation.y = snap_gizmo(hit.y, gizmo_snap.transform_value);
        }
        GizmoAxis::Z => {
            let Some(click_distance) = click_ray.intersect_plane(
                Vec3::new(0., target_transform.translation.y, 0.),
                bevy::math::primitives::InfinitePlane3d::new(Vec3::Y),
            ) else {
                return;
            };
            let hit = camera_transform.translation() - (click_ray.direction * -click_distance);
            target_transform.translation.z = snap_gizmo(hit.z, gizmo_snap.transform_value);
        }
        GizmoAxis::All => {
            let Some(click_distance) = click_ray.intersect_plane(
                target_transform.translation,
                bevy::math::primitives::InfinitePlane3d::new(camera_transform.forward()),
            ) else {
                return;
            };
            println!("click_distance: {}", click_ray.direction * click_distance);

            // let hit = camera_transform.translation() + (click_ray.direction * click_distance);
            // target_transform.translation = hit;
        }
    }
}

fn snap_gizmo(value: f32, inc: f32) -> f32 {
    if inc == 0.0 {
        value
    } else {
        (value / inc).round() * inc
    }
}
