use crate::editor_state::EditorState;
use bevy::{
    gizmos::gizmos::Gizmos,
    math::Vec3,
    prelude::{Color, Query, Res, With},
    transform::components::GlobalTransform,
};
use bevy_granite_core::UICamera;

fn draw_infinite_grid(
    mut gizmos: Gizmos,
    camera_transform: &GlobalTransform,
    max_distance: f32,
    color: [f32; 4],
    size: f32,
) {
    let cell_size = size;
    let camera_pos = camera_transform.translation();
    let camera_x = camera_pos.x;
    let camera_z = camera_pos.z;

    let start_x = camera_x - max_distance;
    let end_x = camera_x + max_distance;
    let start_z = camera_z - max_distance;
    let end_z = camera_z + max_distance;

    let first_x = (start_x / cell_size).floor() * cell_size;
    let first_z = (start_z / cell_size).floor() * cell_size;

    let mut x = first_x;
    while x <= end_x {
        if (camera_pos - Vec3::new(x, 0.0, camera_z)).length() <= max_distance {
            gizmos.line(
                Vec3::new(x, 0.0, start_z),
                Vec3::new(x, 0.0, end_z),
                Color::srgba(color[0], color[1], color[2], color[3]),
            );
        }
        x += cell_size;
    }

    let mut z = first_z;
    while z <= end_z {
        if (camera_pos - Vec3::new(camera_x, 0.0, z)).length() <= max_distance {
            gizmos.line(
                Vec3::new(start_x, 0.0, z),
                Vec3::new(end_x, 0.0, z),
                Color::srgba(color[0], color[1], color[2], color[3]),
            );
        }
        z += cell_size;
    }
}

pub fn update_grid_system(
    gizmos: Gizmos,
    camera_query: Query<&GlobalTransform, With<UICamera>>,
    editor_state: Res<EditorState>,
) {
    if !editor_state.active {
        return;
    }
    if let Ok(camera_transform) = camera_query.single() {
        if editor_state.config.viewport.grid {
            let max_distance = editor_state.config.viewport.grid_distance;
            let color = editor_state.config.viewport.grid_color;
            let size = editor_state.config.viewport.grid_size;
            draw_infinite_grid(gizmos, camera_transform, max_distance, color, size);
        }
    }
}
