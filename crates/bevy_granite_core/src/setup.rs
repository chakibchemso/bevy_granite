use std::borrow::Cow;

use crate::entities::{is_bridge_component_check, ComponentEditor};
use bevy::{
    ecs::reflect::AppTypeRegistry,
    prelude::{ReflectComponent, Res, ResMut, Resource},
    reflect::TypeRegistry,
};
use bevy_granite_logging::{log, LogCategory, LogLevel, LogType};

#[derive(Resource, Default)]
pub struct RegisteredTypeNames {
    pub names: Vec<Cow<'static, str>>,
}

pub fn gather_registered_types(
    type_registry: Res<AppTypeRegistry>,
    mut registered_names: ResMut<RegisteredTypeNames>,
) {
    log!(
        LogType::Game,
        LogLevel::Info,
        LogCategory::System,
        "Gathering serializable component names"
    );
    registered_names.names = get_bridge_reflect_component_names(&type_registry.read());
    registered_names
        .names
        .append(&mut get_bevy_reflect_component_names(&type_registry.read()));
    log!(
        LogType::Game,
        LogLevel::Info,
        LogCategory::System,
        "{:?}",
        registered_names.names
    );
}

fn get_bridge_reflect_component_names(type_registry: &TypeRegistry) -> Vec<Cow<'static, str>> {
    type_registry
        .iter()
        .filter(|registration| {
            registration.data::<ReflectComponent>().is_some() && {
                is_bridge_component_check(registration)
            }
        })
        .map(|registration| registration.type_info().type_path().into())
        .collect()
}

pub fn setup_component_editor(
    mut component_editor: ResMut<ComponentEditor>,
    app_type_registry: Res<AppTypeRegistry>,
) {
    component_editor.type_registry = app_type_registry.clone();
    log!(
        LogType::Game,
        LogLevel::Info,
        LogCategory::System,
        "Gave registry to component editor"
    );
}

/// this will return all the bevy componets that should be accessible in the editor
/// this is a big hack for now because I just need one componet and cant be fucked making a proper dynamic implementation
/// this whole approch will need to be redone in the future - Use Cow<'static str> FFS
fn get_bevy_reflect_component_names(type_registry: &TypeRegistry) -> Vec<Cow<'static, str>> {
    vec![type_registry
        .get(std::any::TypeId::of::<
            bevy::core_pipeline::tonemapping::Tonemapping,
        >())
        .expect("Tonemapping to be registered")
        .type_info()
        .type_path()
        .into()]
}
