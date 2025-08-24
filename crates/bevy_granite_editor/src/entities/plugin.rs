use super::{
    child_removal_system, new_entity_via_popup_system, parent_from_node_tree_system,
    parent_removal_from_entities_system, parent_removal_system, parent_system,
    process_entity_spawn_queue_system, EntitySpawnQueue,
};
use crate::setup::is_editor_active;
use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::IntoScheduleConfigs,
};

pub struct AssetPlugin;
impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // Resources
            //
            .insert_resource(EntitySpawnQueue::default())
            //
            // Schedule system
            //
            .add_systems(
                Update,
                (
                    new_entity_via_popup_system,
                    process_entity_spawn_queue_system,
                    parent_system,
                    parent_from_node_tree_system,
                    child_removal_system,
                    parent_removal_system,
                    parent_removal_from_entities_system,
                )
                    .run_if(is_editor_active),
            );
    }
}
