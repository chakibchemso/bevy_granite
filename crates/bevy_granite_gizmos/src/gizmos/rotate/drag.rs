// Apply the SAME world rotation delta to ROOT ENTITIES ONLY
// Children inherit rotation automatically through hierarchy
use crate::{
    gizmos::{
        GizmoMesh, GizmoOf, GizmoSnap, GizmoType, RotateDraggingEvent, RotateGizmo,
        RotateGizmoParent, RotateInitDragEvent, RotateResetDragEvent, SelectedGizmo,
    },
    input::{DragState, GizmoAxis},
    selection::{
        ray::{raycast_at_cursor, HitType, RaycastCursorLast, RaycastCursorPos},
        ActiveSelection, RequestDuplicateAllSelectionEvent, Selected,
    },
    GizmoCamera,
};
use bevy::{
    ecs::{observer::Trigger, query::Changed, system::Local},
    picking::{
        events::{Drag, Pointer, Pressed},
        hover::PickingInteraction,
    },
    prelude::{
        ChildOf, Children, Entity, EventReader, EventWriter, GlobalTransform, Mut, Name, ParamSet,
        Quat, Query, Res, ResMut, Transform, Vec2, Vec3, Visibility, With, Without,
    },
};
use bevy_granite_core::{CursorWindowPos, IconProxy, UserInput};
use bevy_granite_logging::{
    config::{LogCategory, LogLevel, LogType},
    log,
};

// ------------------------------------------------------------------------
//
type CameraQuery<'w, 's> = Query<'w, 's, &'w Transform, With<GizmoCamera>>;
type ActiveSelectionQuery<'w, 's> = Query<'w, 's, Entity, With<ActiveSelection>>;
type RotateGizmoQuery<'w, 's> =
    Query<'w, 's, (Entity, &'w GizmoAxis, &'w ChildOf), With<RotateGizmo>>;

type RotateGizmoQueryWTransform<'w, 's> =
    Query<'w, 's, (Entity, &'w mut Transform, &'w GlobalTransform), With<RotateGizmoParent>>;
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
//
// ------------------------------------------------------------------------

pub fn handle_rotate_input(
    drag_state: ResMut<DragState>,
    selected_option: ResMut<SelectedGizmo>,
    user_input: Res<UserInput>,
    selection_query: Query<Entity, With<ActiveSelection>>,
    mut init_drag_event: EventWriter<RotateInitDragEvent>,
    mut dragging_event: EventWriter<RotateDraggingEvent>,
    mut drag_ended_event: EventWriter<RotateResetDragEvent>,
) {
    if !user_input.mouse_left.any {
        return;
    }

    if !matches!(selected_option.value, GizmoType::Rotate) {
        // Gizmo value for Rotate
        return;
    }

    if selection_query.single().is_err() {
        return;
    }

    // Setup drag
    if user_input.mouse_left.just_pressed && !drag_state.dragging & !user_input.mouse_over_egui {
        init_drag_event.write(RotateInitDragEvent);
    }
    // Dragging
    else if user_input.mouse_left.pressed && drag_state.dragging {
        dragging_event.write(RotateDraggingEvent);
    }
    // Reset Drag
    else if user_input.mouse_left.just_released && drag_state.dragging {
        drag_ended_event.write(RotateResetDragEvent);
    }
}

pub fn handle_init_rotate_drag(
    mut events: EventReader<RotateInitDragEvent>,
    mut drag_state: ResMut<DragState>,
    resources: (
        Res<CursorWindowPos>,
        ResMut<RaycastCursorLast>,
        ResMut<RaycastCursorPos>,
    ),
    mut duplicate_event_writer: EventWriter<RequestDuplicateAllSelectionEvent>,
    user_input: Res<UserInput>,
    mut gizmo_visibility_query: Query<(&GizmoAxis, Mut<Visibility>)>,
    mut queries: ParamSet<(
        ActiveSelectionQuery,
        RotateGizmoQuery,
        ParentQuery,
        TransformQuery,
        GizmoMeshNameQuery,
        RotateGizmoQueryWTransform,
    )>,
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
) {
    let (cursor_2d, mut raycast_cursor_last_pos, mut raycast_cursor_pos) = resources;

    for _event in events.read() {
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Input,
            "Init rotate drag event",
        );

        // Step 1: Perform Raycast to find the hit entity
        let (entity, hit_type) = raycast_at_cursor(interactions);

        if hit_type == HitType::None
            || hit_type == HitType::Icon
            || hit_type == HitType::Mesh
            || entity.is_none()
        {
            return;
        }

        // Step 2: Get the selected entity
        let selection_query = queries.p0();
        let Ok(_selection_entity) = selection_query.single() else {
            return;
        };

        let Some(raycast_target) = entity else {
            return;
        };

        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Input,
            "Just pressed 'Left' and not dragging"
        );

        // Step 3: Get Gizmo Axis and Parent information
        if let Ok((_gizmo_entity, gizmo_axis, gizmo_parent)) = queries.p1().get(raycast_target) {
            let gizmo_axis = *gizmo_axis;

            let actual_parent = gizmo_parent.parent();

            hide_unselected_axes(gizmo_axis, &mut gizmo_visibility_query);

            let mut query_p3 = queries.p3();
            let Ok((parent_transform, parent_global_transform, _)) =
                query_p3.get_mut(actual_parent)
            else {
                return;
            };

            drag_state.initial_selection_rotation = parent_transform.rotation;
            drag_state.raycast_position = raycast_cursor_pos.position;
            drag_state.initial_cursor_position = cursor_2d.position;
            drag_state.gizmo_position = parent_global_transform.translation();
            drag_state.dragging = true;
            drag_state.locked_axis = Some(gizmo_axis);

            // Compute vector from gizmo to hit point
            let hit_vec = (raycast_cursor_pos.position - drag_state.gizmo_position).normalize();
            drag_state.prev_hit_dir = hit_vec;

            // Get and store initial gizmo rotation
            if let Ok((_, _gizmo_transform, gizmo_world_transform)) = queries.p5().single() {
                let (_, initial_gizmo_rotation, _) =
                    gizmo_world_transform.to_scale_rotation_translation();
                drag_state.initial_gizmo_rotation = initial_gizmo_rotation;
            } else {
                log!(
                    LogType::Editor,
                    LogLevel::Error,
                    LogCategory::Entity,
                    "Couldn't get gizmo transform"
                );
            }

            log!(
                LogType::Editor,
                LogLevel::Info,
                LogCategory::Input,
                "Begin dragging at: {:?}",
                drag_state.locked_axis
            );

            // Step 7: Handle duplication if Shift key is pressed
            if user_input.shift_left.pressed {
                log!(
                    LogType::Editor,
                    LogLevel::Info,
                    LogCategory::Input,
                    "Duplicate entity"
                );
                duplicate_event_writer.write(RequestDuplicateAllSelectionEvent);
            }
        } else {
            return;
        }
    }
}

fn show_unselected_axes(gizmo_query: &mut Query<Mut<Visibility>>) {
    for mut visibility in gizmo_query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

// Function to hide unselected axes
fn hide_unselected_axes(
    selected_axis: GizmoAxis,
    gizmo_query: &mut Query<(&GizmoAxis, Mut<Visibility>)>,
) {
    for (axis, mut visibility) in gizmo_query.iter_mut() {
        *visibility = if *axis == selected_axis {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

pub fn handle_rotate_dragging(
    mut events: EventReader<RotateDraggingEvent>,
    resources: (
        Res<GizmoSnap>,
        ResMut<DragState>,
        Res<CursorWindowPos>,
        Res<SelectedGizmo>,
    ),
    mut queries: ParamSet<(
        CameraQuery,
        ActiveSelectionQuery,
        NonActiveSelectionQuery,
        TransformQuery,
        RotateGizmoQueryWTransform,
        ChildrenQuery,
        ParentQuery, // Add parent query
    )>,
) {
    return;
    let (gizmo_snap, mut drag_state, cursor_2d, selected_gizmo) = resources;
    let gizmo_distance_scale = selected_gizmo.speed_scale;
    let free_rotate_speed = 0.3 * gizmo_distance_scale;
    let locked_rotate_speed = 1.15 * gizmo_distance_scale;

    // let ray_direction = ray.direction;
    // let ray_origin = ray.origin;

    for _ in events.read() {
        let snap_value = gizmo_snap.rotate_value.to_radians();
        let snap = |value: f32| {
            if snap_value == 0.0 {
                value
            } else {
                (value / snap_value).round() * snap_value
            }
        };

        let Some(gizmo_axis) = drag_state.locked_axis else {
            return;
        };

        // Get all selected entities
        let selection_entity = match queries.p1().single() {
            Ok(e) => e,
            Err(_) => return,
        };

        let mut all_selected_entities: Vec<Entity> = queries.p2().iter().collect();
        all_selected_entities.push(selection_entity);

        // Free rotate
        if gizmo_axis == GizmoAxis::All {
            return;
            let cursor_delta_2d = cursor_2d.position - drag_state.initial_cursor_position;
            if cursor_delta_2d == Vec2::ZERO {
                return;
            }
            let delta_pixels = cursor_delta_2d.length();
            let delta_degrees = delta_pixels * 90.0;
            let delta_angle_radians = delta_degrees.to_radians();
            drag_state.accumulated_angle += delta_angle_radians;
            let snapped_angle = snap(drag_state.accumulated_angle);
            let delta_angle = snapped_angle - drag_state.last_snapped;
            if delta_angle.abs() < f32::EPSILON {
                return;
            }
            drag_state.last_snapped = snapped_angle;

            let camera_query = queries.p0();
            if let Ok(camera_transform) = camera_query.single() {
                let camera_right = camera_transform.rotation * Vec3::X;
                let camera_up = camera_transform.rotation * Vec3::Y;
                let yaw = cursor_delta_2d.x * free_rotate_speed;
                let pitch = -cursor_delta_2d.y * free_rotate_speed;
                let rotation_delta = Quat::from_axis_angle(camera_up, yaw)
                    * Quat::from_axis_angle(camera_right, pitch);

                // Apply rotation to each entity independently
                apply_independent_rotation(&mut queries, &all_selected_entities, rotation_delta);

                // Update gizmo
                let final_rotation =
                    if let Ok((transform, _, _)) = queries.p3().get(selection_entity) {
                        transform.rotation
                    } else {
                        Quat::IDENTITY
                    };

                if let Ok((_, mut gizmo_transform, _)) = queries.p4().single_mut() {
                    gizmo_transform.rotation =
                        drag_state.initial_gizmo_rotation * final_rotation.inverse();
                }
            }
            return;
        }

        // Locked rotate
        let axis = match gizmo_axis {
            GizmoAxis::X => Vec3::X,
            GizmoAxis::Y => Vec3::Y,
            GizmoAxis::Z => Vec3::Z,
            _ => return,
        };

        let origin = drag_state.gizmo_position;
        let plane_normal = axis;
        // let ray_dir_dot = ray_direction.dot(plane_normal);
        // if ray_dir_dot.abs() < 1e-6 {
        //     return;
        // }

        // let t = (origin - ray_origin).dot(plane_normal) / ray_dir_dot;
        // let hit_pos = ray_origin + ray_direction * t;
        // let prev_vec = drag_state.prev_hit_dir;
        // let curr_vec = (hit_pos - origin).normalize();
        // let unsigned_angle = prev_vec.angle_between(curr_vec);

        // let direction = prev_vec.cross(curr_vec).dot(axis).signum();

        // let signed_angle = unsigned_angle * direction;
        // let adjusted_angle = signed_angle * locked_rotate_speed;
        // let new_accum = drag_state.accumulated_angle + adjusted_angle;
        // let snapped = snap(new_accum);
        // let delta_angle = snapped - drag_state.last_snapped;

        // if delta_angle.abs() < f32::EPSILON {
        //     return;
        // }

        // drag_state.last_snapped = snapped;
        // drag_state.accumulated_angle = new_accum;
        // drag_state.prev_hit_dir = curr_vec;

        // let rotation_delta = Quat::from_axis_angle(axis, delta_angle);

        // Apply rotation to each entity independently
        // apply_independent_rotation(&mut queries, &all_selected_entities, rotation_delta);

        // Update gizmo
        let final_rotation = if let Ok((transform, _, _)) = queries.p3().get(selection_entity) {
            transform.rotation
        } else {
            Quat::IDENTITY
        };

        if let Ok((_, mut gizmo_transform, _)) = queries.p4().single_mut() {
            gizmo_transform.rotation = drag_state.initial_gizmo_rotation * final_rotation.inverse();
        }
    }
}

pub fn debug_handle_rotate_dragging<const AXIS: char>(
    drag: Trigger<Pointer<Drag>>,
    targets: Query<&GizmoOf>,
    camera_query: Query<&Transform, With<GizmoCamera>>,
    mut objects: Query<&mut Transform, Without<GizmoCamera>>,
    gizmo_snap: Res<GizmoSnap>,
    selected: Res<SelectedGizmo>,
    mut accrued: Local<Vec2>,
) {
    let gizmo_distance_scale = selected.speed_scale;
    let free_rotate_speed = 0.3 * gizmo_distance_scale;

    let gizmo_axis = match AXIS {
        'X' | 'x' => GizmoAxis::X,
        'Y' | 'y' => GizmoAxis::Y,
        'Z' | 'z' => GizmoAxis::Z,
        'A' | 'a' => GizmoAxis::All,
        _ => GizmoAxis::All,
    };
    *accrued += drag.delta * free_rotate_speed;
    if accrued.x.abs() < gizmo_snap.rotate_value && accrued.y.abs() < gizmo_snap.rotate_value {
        return;
    }
    let x_step = snap_roation(accrued.x, gizmo_snap.rotate_value);
    let y_step = snap_roation(accrued.y, gizmo_snap.rotate_value);
    *accrued = Vec2::ZERO;
    let delta_x = x_step.to_radians();
    let delta_y = y_step.to_radians();
    let delta_z = if x_step.abs() > y_step.abs() {
        delta_x
    } else {
        delta_y
    };
    let Ok(target) = targets.get(drag.target) else {
        log(
            LogType::Editor,
            LogLevel::Error,
            LogCategory::Debug,
            format!("Rotaion Gizmo({})'s Target not found", drag.target.index()),
        );
        return;
    };
    match gizmo_axis {
        GizmoAxis::All => {
            if let Ok(camera_transform) = camera_query.single() {
                let camera_right = camera_transform.right().as_vec3();
                let camera_up = camera_transform.up().as_vec3();
                let rotation_delta = Quat::from_axis_angle(camera_up, delta_x)
                    * Quat::from_axis_angle(camera_right, delta_y);

                // Apply rotation to each entity independently
                // apply_independent_rotation(&mut queries, &all_selected_entities, rotation_delta);

                if let Ok(mut transform) = objects.get_mut(**target) {
                    transform.rotate(rotation_delta);
                }
            }
        }
        GizmoAxis::X => {
            if let Ok(mut transform) = objects.get_mut(**target) {
                transform.rotate(Quat::from_axis_angle(Vec3::X, delta_z));
            }
        }
        GizmoAxis::Y => {
            if let Ok(mut transform) = objects.get_mut(**target) {
                transform.rotate(Quat::from_axis_angle(Vec3::Y, delta_z));
            }
        }
        GizmoAxis::Z => {
            if let Ok(mut transform) = objects.get_mut(**target) {
                transform.rotate(Quat::from_axis_angle(Vec3::Z, delta_z));
            }
        }
        _ => {
            log!(
                LogType::Editor,
                LogLevel::Critical,
                LogCategory::Debug,
                "Rotation Gizmo Axis not implemented for axis {:?}",
                gizmo_axis
            )
        }
    }
}

fn snap_roation(value: f32, inc: f32) -> f32 {
    if inc == 0.0 {
        value
    } else {
        (value / inc).round() * inc
    }
}

pub fn test_click_trigger(click: Trigger<Pointer<Pressed>>, query: Query<&Name>) {
    let name = query.get(click.target);
    println!(
        "Click on {:?} Triggered: {}\n, {:?}",
        name,
        click.target.index(),
        click
    );
}

fn apply_independent_rotation(
    queries: &mut ParamSet<(
        CameraQuery,
        ActiveSelectionQuery,
        NonActiveSelectionQuery,
        TransformQuery,
        RotateGizmoQueryWTransform,
        ChildrenQuery,
        ParentQuery,
    )>,
    all_selected_entities: &[Entity],
    rotation_delta: Quat,
) {
    // Phase 1a: Get original global transforms
    let mut original_data = std::collections::HashMap::new();
    {
        let transform_query = queries.p3();
        for &entity in all_selected_entities {
            if let Ok((_, global_transform, _)) = transform_query.get(entity) {
                let (scale, rotation, translation) =
                    global_transform.to_scale_rotation_translation();
                original_data.insert(entity, (scale, rotation, translation));
            }
        }
    }

    // Phase 1b: Get parent relationships
    let mut parent_map = std::collections::HashMap::new();
    {
        let parent_query = queries.p6();
        for &entity in all_selected_entities {
            if let Ok(parent) = parent_query.get(entity) {
                parent_map.insert(entity, parent.parent());
            }
        }
    }

    // Phase 2: Calculate what each entity's final local transform should be
    let mut final_local_transforms = std::collections::HashMap::new();

    for &entity in all_selected_entities {
        if let Some((scale, rotation, translation)) = original_data.get(&entity) {
            // Target: same global position, rotated rotation
            let target_global_rotation = rotation_delta * *rotation;
            let target_global_position = *translation; // STAY PUT!

            if let Some(parent_entity) = parent_map.get(&entity) {
                // Child entity - need parent's current state
                let (parent_rotation, parent_translation) =
                    if all_selected_entities.contains(parent_entity) {
                        // Parent is selected, use its rotated state
                        if let Some((_, parent_orig_rotation, parent_orig_translation)) =
                            original_data.get(parent_entity)
                        {
                            (
                                rotation_delta * *parent_orig_rotation,
                                *parent_orig_translation,
                            )
                        } else {
                            continue; // Skip if can't get parent data
                        }
                    } else {
                        // Parent is NOT selected, get its current transform
                        // We need to get this from a fresh query since it's not in original_data
                        continue; // We'll handle this in a separate phase
                    };

                // Convert child's target global state to local relative to parent's state
                let local_position =
                    parent_rotation.inverse() * (target_global_position - parent_translation);
                let local_rotation = parent_rotation.inverse() * target_global_rotation;

                final_local_transforms.insert(entity, (local_position, local_rotation, *scale));
            } else {
                // Root entity - local = global
                final_local_transforms
                    .insert(entity, (*translation, target_global_rotation, *scale));
            }
        }
    }

    // Phase 2b: Handle children whose parents are NOT selected
    {
        let transform_query = queries.p3();
        for &entity in all_selected_entities {
            if final_local_transforms.contains_key(&entity) {
                continue; // Already handled
            }

            if let Some((scale, rotation, translation)) = original_data.get(&entity) {
                let target_global_rotation = rotation_delta * *rotation;
                let target_global_position = *translation;

                if let Some(parent_entity) = parent_map.get(&entity) {
                    // Get parent's current transform
                    if let Ok((_, parent_global, _)) = transform_query.get(*parent_entity) {
                        let (_, parent_rotation, parent_translation) =
                            parent_global.to_scale_rotation_translation();

                        let local_position = parent_rotation.inverse()
                            * (target_global_position - parent_translation);
                        let local_rotation = parent_rotation.inverse() * target_global_rotation;

                        final_local_transforms
                            .insert(entity, (local_position, local_rotation, *scale));
                    }
                }
            }
        }
    }

    // Phase 3: Apply all transforms simultaneously
    {
        let mut transform_query = queries.p3();
        for &entity in all_selected_entities {
            if let Some((pos, rot, scale)) = final_local_transforms.get(&entity) {
                if let Ok((mut transform, _, _)) = transform_query.get_mut(entity) {
                    transform.translation = *pos;
                    transform.rotation = *rot;
                    transform.scale = *scale;
                }
            }
        }
    }
}

pub fn handle_rotate_reset(
    mut events: EventReader<RotateResetDragEvent>,
    mut drag_state: ResMut<DragState>,
    selection_query: Query<Entity, With<ActiveSelection>>,
    transform_query: Query<(&mut Transform, &GlobalTransform, Entity), Without<GizmoCamera>>,
    mut gizmo_visibility_query: Query<Mut<Visibility>>,
) {
    for RotateResetDragEvent in events.read() {
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Input,
            "Rotation drag reset event",
        );
        let mut final_position = None;
        if let Some(selection_entity) = selection_query.iter().next() {
            if let Ok((_selection_transform, selection_global_transform, _)) =
                transform_query.get(selection_entity)
            {
                final_position = Some(selection_global_transform.translation());
            }
        }
        show_unselected_axes(&mut gizmo_visibility_query);

        drag_state.dragging = false;
        drag_state.locked_axis = None;
        drag_state.drag_ended = true;

        if let Some(position) = final_position {
            drag_state.raycast_position = position;
        }

        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Input,
            "Finish dragging"
        );
    }
}
