use bevy::{prelude::*, reflect::TypeRegistry};
use bevy_granite_core::entities::ExposedToEditor;

pub struct BevyGraniteExposePlugin;

impl Plugin for BevyGraniteExposePlugin {
    fn build(&self, app: &mut App) {
        register_exposed_types(app);
    }
}

fn register_exposed_types(app: &mut App) {
    let registry = app.world_mut().resource::<AppTypeRegistry>();
    let mut registry = registry.write();
    register_bevy_component::<bevy::core_pipeline::tonemapping::Tonemapping>(&mut registry);
}

fn register_bevy_component<T: ExposeToEditor + std::any::Any>(registry: &mut TypeRegistry) {
    if let Some(reg) = registry.get_mut(std::any::TypeId::of::<T>()) {
        reg.insert(ExposedToEditor {
            read_only: T::read_only(),
        });
    };
}

trait ExposeToEditor {
    fn read_only() -> bool;
}

impl ExposeToEditor for bevy::core_pipeline::tonemapping::Tonemapping {
    fn read_only() -> bool {
        false // or true, depending on your requirements
    }
}
