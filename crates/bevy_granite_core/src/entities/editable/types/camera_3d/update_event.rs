use super::{UserUpdatedCamera3DEvent, VolumetricFog};
use crate::{
    entities::editable::RequestEntityUpdateFromClass, Camera3D, GraniteTypes, IdentityData,
};
use bevy::{
    ecs::{
        entity::Entity,
        event::{EventReader},
        system::{Commands, Query},
    },
    pbr::VolumetricFogSettings,
    render::camera::Camera,
};
use bevy_granite_logging::{log, LogCategory, LogLevel, LogType};

impl Camera3D {
    /// Request an entity update with this data
    pub fn push_to_entity(&self, entity: Entity, request_update: &mut RequestEntityUpdateFromClass) {
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Entity,
            "Requesting camera entity update"
        );

        request_update.camera_3d.send(UserUpdatedCamera3DEvent {
            entity: entity,
            data: self.clone(),
        });
    }
}


/// Actually update the specific entity with the class data
/// In the future im sure we will have FOV and what not
pub fn update_camera_3d_system(
    mut reader: EventReader<UserUpdatedCamera3DEvent>,
    mut query: Query<(Entity, &mut Camera, &mut IdentityData)>,
    mut commands: Commands,
) {
    for UserUpdatedCamera3DEvent {
        entity: requested_entity,
        data: new,
    } in reader.read()
    {
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Entity,
            "Heard camera3d update event: {}",
            requested_entity
        );
        if let Ok((entity, mut camera, mut identity_data)) = query.get_mut(*requested_entity) {
            if new.is_active {
                camera.is_active = true;
            } else {
                camera.is_active = false;
            }

            if new.has_volumetric_fog {
                let found_fog = new.volumetric_fog_settings.clone();
                if let Some(new_fog) = found_fog {
                    commands.entity(entity).remove::<VolumetricFogSettings>();
                    commands.entity(entity).insert(VolumetricFogSettings {
                        fog_color: new_fog.fog_color,
                        absorption: new_fog.absorption,
                        ambient_color: new_fog.ambient_color,
                        ambient_intensity: new_fog.ambient_intensity,
                        step_count: new_fog.step_count,
                        light_intensity: new_fog.light_intensity,
                        light_tint: new_fog.light_tint,
                        density: new_fog.density,
                        max_depth: new_fog.max_depth,
                        scattering: new_fog.scattering,
                        scattering_asymmetry: new_fog.scattering_asymmetry,
                    });
                } else {
                    let default_fog = VolumetricFog::default();
                    commands.entity(entity).insert(VolumetricFogSettings {
                        fog_color: default_fog.fog_color,
                        absorption: default_fog.absorption,
                        ambient_color: default_fog.ambient_color,
                        ambient_intensity: default_fog.ambient_intensity,
                        step_count: default_fog.step_count,
                        light_intensity: default_fog.light_intensity,
                        light_tint: default_fog.light_tint,
                        density: default_fog.density,
                        max_depth: default_fog.max_depth,
                        scattering: default_fog.scattering,
                        scattering_asymmetry: default_fog.scattering_asymmetry,
                    });
                }
            } else {
                commands.entity(entity).remove::<VolumetricFogSettings>();
            }

            // Update the IdentityData to match new changes
            if let GraniteTypes::Camera3D(ref mut camera_data) = identity_data.class {
                camera_data.is_active = new.is_active;
                camera_data.has_volumetric_fog = new.has_volumetric_fog;

                if new.has_volumetric_fog {
                    // Ensure volumetric_fog_settings is populated
                    if camera_data.volumetric_fog_settings.is_none() {
                        camera_data.volumetric_fog_settings = Some(VolumetricFog::default());
                    }
                } else {
                    camera_data.volumetric_fog_settings = None;
                }
            }
        } else {
            log!(
                LogType::Editor,
                LogLevel::Error,
                LogCategory::Entity,
                "Could not find camera on: {}",
                requested_entity
            );
        }
    }
}
