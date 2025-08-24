use bevy::{
    ecs::{
        query::With,
        system::{Query, ResMut},
    },
    math::Vec3,
    transform::components::{GlobalTransform, Transform},
};

use crate::GizmoCamera;
use super::{GizmoChildren, GizmoType, SelectedGizmo};

const DISTANCE_SCALING_ENABLED: bool = true;

pub fn scale_gizmo_by_camera_distance_system(
    camera_q: Query<&GlobalTransform, With<GizmoCamera>>,
    mut gizmo_q: Query<(&GlobalTransform, &mut Transform), With<GizmoChildren>>,
    mut selected_gizmo: ResMut<SelectedGizmo>,
) {
    if !DISTANCE_SCALING_ENABLED {
        return;
    }

    let Ok(cam_transform) = camera_q.single() else {
        return;
    };

    if matches!(selected_gizmo.value, GizmoType::Pointer) {
        return;
    }

    if gizmo_q.is_empty() {
        return;
    }

    for (gizmo_global_transform, mut gizmo_transform) in gizmo_q.iter_mut() {
        // Calculate what local scale is needed to achieve global scale of 1.0
        let current_global_scale = gizmo_global_transform.to_scale_rotation_translation().0;
        let parent_scale_factor = current_global_scale / gizmo_transform.scale;
        let baseline_local_scale = Vec3::splat(1.0) / parent_scale_factor;

        let distance = cam_transform
            .translation()
            .distance(gizmo_global_transform.translation());

        let base_scale = 0.35; // scale of gizmo's initially
        let distance_scale = 0.08; // factor to scale via distance
        let scale_factor = (distance * distance_scale).clamp(base_scale, 9.0); // 9.0 is max size of gizmo
        let final_scale = (base_scale * scale_factor).clamp(base_scale, f32::INFINITY);

        // Apply the final scale while accounting for parent transforms
        gizmo_transform.scale = baseline_local_scale * final_scale;
        selected_gizmo.distance_scale = final_scale;

        // Transform Gizmo should have higher upper limit on speed
        if matches!(selected_gizmo.value, GizmoType::Transform) {
            if final_scale > base_scale {
                selected_gizmo.speed_scale = final_scale * 3.3; // the further away we are how much faster should entities be moved
            } else {
                selected_gizmo.speed_scale = 1.0
            }
        }
        // Rotate Gizmo should not speed up as quick, and have lower ceiling
        else if matches!(selected_gizmo.value, GizmoType::Rotate) {
            if final_scale > base_scale {
                selected_gizmo.speed_scale = (final_scale * 1.01).clamp(1.0, 1.5);
            } else {
                selected_gizmo.speed_scale = 1.0
            }
        } else {
            selected_gizmo.speed_scale = 1.0;
        }
    }
}
