use bevy::ecs::{component::Component, entity::Entity};

/// Represents different types of icons used in embedded fetching and debug viewport
#[derive(Debug, Clone, Copy)]
pub enum IconType {
    Empty,
    Camera3D,
    PointLight,
    DirectionalLight,
}

/// Marker component for entities that are icon proxies
/// The entity with this component should redirect selection to the entity in the component
#[derive(Component)]
pub struct IconProxy {
    pub target_entity: Entity,
}

#[derive(Component)]
pub struct IconEntity {
    pub target_entity: Entity,
}
