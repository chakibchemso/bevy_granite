use super::UserUpdatedOBJEvent;
use crate::{entities::editable::RequestEntityUpdateFromClass, OBJ};
use bevy::ecs::{entity::Entity, event::EventReader};
use bevy_granite_logging::{log, LogCategory, LogLevel, LogType};

impl OBJ {
    pub fn push_to_entity(
        &self,
        entity: Entity,
        request_update: &mut RequestEntityUpdateFromClass,
    ) {
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Entity,
            "Requesting OBJ entity update"
        );

        request_update.obj.write(UserUpdatedOBJEvent {
            entity,
            data: self.clone(),
        });
    }
}

pub fn update_obj_system(mut reader: EventReader<UserUpdatedOBJEvent>) {
    for UserUpdatedOBJEvent {
        entity: requested_entity,
        data: _new,
    } in reader.read()
    {
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Entity,
            "Heard obj update event: {}",
            requested_entity
        );
        // Nothing to update yet
        // If we impl OBJ editable data, well need to push it here
    }
}
