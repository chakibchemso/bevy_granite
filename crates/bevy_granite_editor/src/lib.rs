use crate::setup::{editor_info, setup_ui_style};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

pub mod entities;
pub mod editor_state;
pub mod input;
pub mod interface;
pub mod setup;
pub mod utils;
pub mod viewport;

use entities::AssetPlugin;
use editor_state::ConfigPlugin;
use input::InputPlugin;
use interface::InterfacePlugin;
use viewport::ViewportPlugin;

pub use entities::get_entity_bounds_or_fallback;
pub use editor_state::{
    get_interface_config_float, get_interface_config_str, update_editor_config_field, HELP_CONFIG,
    UI_CONFIG,
};
pub use interface::events::{
    RequestCameraEntityFrame, RequestEditorToggle, RequestNewParent, RequestRemoveChildren,
    RequestRemoveParents, RequestToggleCameraSync,
};

pub struct BevyGraniteEditor {
    pub active: bool,
    pub default_world: String,
}

impl Plugin for BevyGraniteEditor {
    fn build(&self, app: &mut App) {
        app
            //
            //Plugins
            //
            .add_plugins(FrameTimeDiagnosticsPlugin) // Bevy internal frame plugin
            //
            // Internal plugins
            .add_plugins(InputPlugin)
            .add_plugins(InterfacePlugin)
            .add_plugins(ViewportPlugin) // Required
            .add_plugins(AssetPlugin) // Required
            .add_plugins(ConfigPlugin {
                editor_active: self.active,
                default_world: self.default_world.clone(),
            }) // Required
            //
            // Startup
            //
            .add_systems(PostStartup, (setup_ui_style, editor_info));
    }
}
