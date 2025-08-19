use crate::{editor_state::EditorState, interface::UserRequestGraniteTypeViaPopup};
use bevy::{
    asset::{AssetServer, Assets},
    ecs::{
        event::{EventReader, EventWriter},
        system::{Commands, Res, ResMut},
    },
    math::Vec3,
    pbr::StandardMaterial,
    prelude::Resource,
    render::mesh::Mesh,
    transform::components::Transform,
};
use bevy_granite_core::{
    entities::{GraniteType, SpawnSource},
    shared::asset_file_browser_multiple,
    AvailableEditableMaterials, GraniteTypes, PromptData, PromptImportSettings,
};
use bevy_granite_gizmos::{RequestDeselectAllEntitiesEvent, RequestSelectEntityEvent};
use bevy_granite_logging::{log, LogCategory, LogLevel, LogType};
use std::collections::VecDeque;

#[derive(Resource, Default)]
pub struct EntitySpawnQueue {
    pub pending: VecDeque<PendingEntitySpawn>,
    pub current_batch_size: usize,
}

#[derive(Clone, PartialEq)]
pub struct PendingEntitySpawn {
    pub class: GraniteTypes,
    pub file: Option<String>,
    pub transform: Transform,
    pub source: String,
    pub batch_size: usize,
}

// Popup to queues entity spawns. Handles single and multiple
pub fn new_entity_via_popup_system(
    mut entity_add_reader: EventReader<UserRequestGraniteTypeViaPopup>,
    mut deselect_writer: EventWriter<RequestDeselectAllEntitiesEvent>,
    mut spawn_queue: ResMut<EntitySpawnQueue>,
    editor_state: Res<EditorState>,
) {
    if let Some(UserRequestGraniteTypeViaPopup { class }) = entity_add_reader.read().next() {
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Entity,
            "User wants to add via popup: {:?}",
            class
        );

        let mut transform = Transform::default();
        transform.translation = Vec3::ZERO;

        let source = editor_state
            .current_file
            .clone()
            .unwrap_or_else(|| "user".to_string());

        if class.needs_prompt() {
            let (base_dir, filter) = class.get_prompt_config();
            if let Some(files) = asset_file_browser_multiple(base_dir, filter) {
                let batch_size = files.len();
                if batch_size > 1 {
                    deselect_writer.send(RequestDeselectAllEntitiesEvent);
                }
                spawn_queue.current_batch_size = batch_size;

                // Queue each file as a separate spawn
                for file in files {
                    spawn_queue.pending.push_back(PendingEntitySpawn {
                        class: class.clone(),
                        file: Some(file),
                        transform,
                        source: source.clone(),
                        batch_size,
                    });
                }
            }
        } else {
            let batch_size = 1;
            spawn_queue.current_batch_size = batch_size;

            // Queue single spawn without file
            spawn_queue.pending.push_back(PendingEntitySpawn {
                class: class.clone(),
                file: None,
                transform,
                source: source.clone(),
                batch_size,
            });
        }

        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Entity,
            "Queued {} entities for spawning",
            spawn_queue.pending.len()
        );
    }
}

pub fn process_entity_spawn_queue_system(
    mut spawn_queue: ResMut<EntitySpawnQueue>,
    mut select_new_entity_writer: EventWriter<RequestSelectEntityEvent>,
    mut commands: Commands,
    available_materials: ResMut<AvailableEditableMaterials>,
    standard_materials: ResMut<Assets<StandardMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    prompt_settings: Res<PromptImportSettings>,
) {
    if let Some(mut pending) = spawn_queue.pending.pop_front() {
        let prompt_data = PromptData {
            file: pending.file,
            import_settings: prompt_settings.clone(),
        };

        let entity = pending.class.spawn_from_new_identity(
            &mut commands,
            pending.transform,
            standard_materials,
            meshes,
            available_materials,
            asset_server,
            Some(prompt_data),
        );

        // Tag entity with spawn source
        commands
            .entity(entity)
            .insert(SpawnSource(pending.source.clone()));

        let additive = pending.batch_size > 1;
        let remaining = spawn_queue.pending.len();

        select_new_entity_writer.send(RequestSelectEntityEvent { entity, additive });

        log!(
            LogType::Editor,
            LogLevel::OK,
            LogCategory::Entity,
            "Spawned entity: '{:?}' from queue, {} remaining",
            entity,
            remaining
        );
    }
}
