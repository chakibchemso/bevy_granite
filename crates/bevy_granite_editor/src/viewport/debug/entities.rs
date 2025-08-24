use super::DebugRenderer;
use crate::editor_state::EditorState;
use bevy::{
    ecs::{entity::Entity, system::Query},
    gizmos::gizmos::Gizmos,
    math::Vec3,
    prelude::{Res, Transform, With},
    transform::components::GlobalTransform,
};
use bevy_granite_core::{GraniteTypes, IdentityData};
use bevy_granite_gizmos::Selected;

pub fn show_empty_origin_system(
    query: Query<(Entity, &GlobalTransform, &IdentityData)>,
    active_query: Query<Entity, With<Selected>>,
    mut gizmos: Gizmos<DebugRenderer>,
    editor_state: Res<EditorState>,
) {
    if !editor_state.active {
        return;
    }
    let config = editor_state.config.viewport.visualizers;
    if !config.debug_enabled {
        return;
    }
    for (entity, global_transform, identity_data) in query.iter() {
        if config.debug_selected_only {
            match active_query.single() {
                Ok(selected_entity) if selected_entity != entity => continue,
                Err(_) => return,
                _ => {}
            }
        }

        if matches!(identity_data.class, GraniteTypes::Empty(_)) {
            let f_size = 0.7;

            let size = Vec3::new(f_size, f_size, f_size);
            let pos = global_transform.translation();
            let rot = global_transform.to_scale_rotation_translation().1;
            //let color = Color::srgb_from_array(config.debug_color);
            let transform_gizmo = Transform {
                translation: pos,
                rotation: rot,
                scale: size,
            };
            gizmos.axes(transform_gizmo, f_size);
            //gizmos.cuboid(transform_gizmo, color);
        }
    }
}
