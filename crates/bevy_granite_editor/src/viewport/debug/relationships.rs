use super::DebugRenderer;
use crate::editor_state::EditorState;
use bevy::{
    color::Color,
    ecs::{entity::Entity, system::Query},
    gizmos::gizmos::Gizmos,
    prelude::{ChildOf, Children},
    prelude::{Res, With, Without},
    transform::components::GlobalTransform,
};
use bevy_granite_core::IconProxy;
use bevy_granite_gizmos::{ActiveSelection, GizmoMesh};

pub fn relationship_line_system(
    mut gizmos: Gizmos<DebugRenderer>,
    child_query: Query<
        (Entity, &GlobalTransform, &ChildOf),
        (Without<GizmoMesh>, Without<IconProxy>),
    >,
    active_query: Query<Entity, With<ActiveSelection>>,
    transform_query: Query<&GlobalTransform>,
    parents_query: Query<&Children>,
    editor_state: Res<EditorState>,
) {
    if !editor_state.active {
        return;
    }
    let config = editor_state.config.viewport.visualizers;
    if !config.debug_enabled || !config.debug_relationship_lines {
        return;
    }

    let tip_length = 0.2;
    let active_entities: Vec<Entity> = active_query.iter().collect();

    if !config.debug_selected_only {
        for (_entity, transform, child_of) in child_query.iter() {
            if let Ok(parent_transform) = transform_query.get(child_of.parent()) {
                let child_pos = transform.translation();
                let parent_pos = parent_transform.translation();
                let color = Color::srgb_from_array(config.debug_color);

                gizmos
                    .arrow(child_pos, parent_pos, color)
                    .with_tip_length(tip_length);
            }
        }
        return;
    }

    if active_entities.is_empty() {
        return;
    }

    let mut connected_entities = std::collections::HashSet::new();
    for &selected_entity in &active_entities {
        let mut current = selected_entity;
        connected_entities.insert(current);

        while let Ok((_, _, child_of)) = child_query.get(current) {
            let parent_entity = child_of.parent();
            connected_entities.insert(parent_entity);
            current = parent_entity;
        }

        add_descendants(selected_entity, &parents_query, &mut connected_entities);
    }

    for (entity, transform, child_of) in child_query.iter() {
        let parent_entity = child_of.parent();
        if connected_entities.contains(&entity) || connected_entities.contains(&parent_entity) {
            if let Ok(parent_transform) = transform_query.get(parent_entity) {
                let child_pos = transform.translation();
                let parent_pos = parent_transform.translation();
                let color = Color::srgb_from_array(config.debug_color);

                gizmos
                    .arrow(child_pos, parent_pos, color)
                    .with_tip_length(tip_length);
            }
        }
    }
}

fn add_descendants(
    entity: Entity,
    children_query: &Query<&Children>,
    connected_entities: &mut std::collections::HashSet<Entity>,
) {
    if let Ok(children) = children_query.get(entity) {
        for &child in children.iter() {
            connected_entities.insert(child);
            add_descendants(child, children_query, connected_entities);
        }
    }
}
