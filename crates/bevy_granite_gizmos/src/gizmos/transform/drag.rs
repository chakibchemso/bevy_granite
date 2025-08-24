use std::f32::consts::E;

use super::{draw_axis_line, TransformGizmo};
use crate::{
    gizmos::{
        GizmoMesh, GizmoOf, GizmoRoot, GizmoSnap, GizmoType, SelectedGizmo, TransformDraggingEvent,
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
    asset::Assets,
    ecs::{
        component::Component,
        event::Event,
        observer::Trigger,
        query::{self, Changed},
        system::{Commands, Local, Single},
    },
    gizmos::{config::GizmoLineConfig, retained::Gizmo, GizmoAsset},
    input::{keyboard::KeyCode, mouse::MouseButton, ButtonInput},
    math::{curve::cores::even_interp, Vec2},
    picking::{
        events::{Drag, DragEntry, Pointer, Pressed, Released},
        hover::PickingInteraction,
    },
    prelude::{
        ChildOf, Children, Entity, EventReader, EventWriter, Gizmos, GlobalTransform, Name,
        ParamSet, Query, Res, ResMut, Transform, Vec3, With, Without,
    },
    window::Window,
    winit::cursor,
};
use bevy_egui::egui::Button;
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

pub fn drag_transform_gizmo(
    event: Trigger<Pointer<Drag>>,
    targets: Query<&GizmoOf>,
    camera_query: Query<(&GlobalTransform, &bevy::render::camera::Camera), With<GizmoCamera>>,
    mut objects: Query<&mut Transform, Without<GizmoCamera>>,
    gizmo_snap: Res<GizmoSnap>,
    gizmo_data: Query<&GizmoAxis>,
    input: Res<ButtonInput<KeyCode>>,
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

pub fn draw_axis_lines(
    mut event: Trigger<Pointer<Pressed>>,
    gizmo_data: Query<(&GizmoRoot, &GizmoAxis, &GizmoOf), With<TransformGizmo>>,
    mut bevy_gizmo: ResMut<Assets<GizmoAsset>>,
    mut commands: Commands,
    origin: Query<&Transform>,
) {
    let Ok((parent, axis, root)) = gizmo_data.get(event.target) else {
        return;
    };
    event.propagate(false);
    let Ok(origin) = origin.get(root.get()) else {
        log! {
            LogType::Editor,
            LogLevel::Error,
            LogCategory::Input,
            "Gizmo origin transform not found for entity {:?}",
            root.0
        };
        return;
    };
    let mut asset = GizmoAsset::new();
    asset.line(
        origin.translation + axis.to_vec3() * 1000.,
        origin.translation + axis.to_vec3() * -1000.,
        axis.color(),
    );

    commands.spawn((
        *axis,
        GizmoOf(root.0),
        Gizmo {
            handle: bevy_gizmo.add(asset),
            ..Default::default()
        },
        AxisLine,
    ));
}

pub fn cleanup_axis_line(
    mut commands: Commands,
    query: Query<Entity, With<AxisLine>>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if input.just_released(MouseButton::Left) {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
pub struct AxisLine;
