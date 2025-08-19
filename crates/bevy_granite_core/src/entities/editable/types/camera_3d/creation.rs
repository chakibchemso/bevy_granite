use super::Camera3D;
use crate::{
    entities::EntitySaveReadyData, GraniteEditorSerdeEntity, GraniteType, GraniteTypes,
    HasRuntimeData, IdentityData,
};
use bevy::{
    core::Name,
    core_pipeline::core_3d::Camera3dBundle,
    ecs::{bundle::Bundle, entity::Entity, system::Commands},
    pbr::VolumetricFogSettings,
    render::camera::Camera,
    transform::components::Transform,
};
use uuid::Uuid;

impl Camera3D {
    /// Extract needed info to spawn this entity via save data
    pub fn spawn_from_save_data(
        save_data: &EntitySaveReadyData,
        commands: &mut Commands,
    ) -> Entity {
        let identity = &save_data.identity;
        let save_transform = &save_data.transform;

        Self::spawn_from_identity(commands, identity, save_transform.to_bevy())
    }

    /// Take the name and class from identity to spawn
    pub fn spawn_from_identity(
        commands: &mut Commands,
        identity: &IdentityData,
        transform: Transform,
    ) -> Entity {
        let class = Self::extract_class(&identity);

        class.spawn(identity, commands, transform)
    }

    /// Generally to be used from UI popups as it gives default name
    pub fn spawn_from_new_identity(&self, commands: &mut Commands, transform: Transform) -> Entity {
        let identity = IdentityData {
            name: self.type_name(),
            uuid: Uuid::new_v4(),
            class: GraniteTypes::Camera3D(self.clone()),
        };
        self.spawn(&identity, commands, transform)
    }

    /// Private core logic
    fn spawn(
        &self,
        identity: &IdentityData,
        commands: &mut Commands,
        transform: Transform,
    ) -> Entity {
        let mut entity =
            commands.spawn(Self::get_bundle(self.clone(), identity.clone(), transform));

        if self.has_volumetric_fog {
            if let Some(fog_settings) = &self.volumetric_fog_settings {
                entity.insert(VolumetricFogSettings {
                    fog_color: fog_settings.fog_color,
                    absorption: fog_settings.absorption,
                    ambient_color: fog_settings.ambient_color,
                    ambient_intensity: fog_settings.ambient_intensity,
                    step_count: fog_settings.step_count,
                    light_intensity: fog_settings.light_intensity,
                    light_tint: fog_settings.light_tint,
                    density: fog_settings.density,
                    max_depth: fog_settings.max_depth,
                    scattering: fog_settings.scattering,
                    scattering_asymmetry: fog_settings.scattering_asymmetry,
                });
            } else {
                entity.insert(VolumetricFogSettings::default());
            }
        }
        entity.id()
    }

    /// Build a bundle that is ready to spawn from a Camera3D
    fn get_bundle(
        camera_3d: Camera3D,
        identity: IdentityData,
        transform: Transform,
    ) -> impl Bundle {
        (
            Camera3dBundle {
                camera: Camera {
                    is_active: camera_3d.is_active,
                    ..Default::default()
                },
                transform,
                ..Default::default()
            },
            Name::new(identity.name.clone()),
            GraniteEditorSerdeEntity,
            HasRuntimeData,
            IdentityData {
                name: identity.name.clone(),
                uuid: identity.uuid.clone(),
                class: identity.class.clone(),
            },
        )
    }

    fn extract_class(identity: &IdentityData) -> Camera3D {
        match &identity.class {
            GraniteTypes::Camera3D(camera_data) => camera_data.clone(),
            _ => panic!("Expected Camera3D class data, got different type from save data"),
        }
    }
}
