pub mod cleanup;
pub mod spawning;
pub mod updating;

use bevy::{
    asset::Assets,
    ecs::system::ResMut,
    pbr::PbrBundle,
    prelude::{Bundle, Name},
    render::texture::Image,
};
use bevy_granite_core::{GraniteType, GraniteTypes, IconEntity};
use bevy_granite_logging::{log, LogCategory, LogLevel, LogType};

#[derive(Bundle)]
pub struct IconBundle {
    pub icon_entity: IconEntity,
    pub pbr_bundle: PbrBundle,
    pub name: Name,
}

// Re-export all icon functions
pub use cleanup::*;
pub use spawning::*;
pub use updating::*;

// Register each classes embedded icon
pub fn register_embedded_class_icons(mut images: ResMut<Assets<Image>>) {
    for class in GraniteTypes::all() {
        class.register_embedded_icon(&mut images);
    }
    log!(
        LogType::Game,
        LogLevel::OK,
        LogCategory::Asset,
        "Registered all embedded class icons"
    );
}
