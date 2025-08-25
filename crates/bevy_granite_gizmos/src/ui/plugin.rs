use super::editor_gizmos_ui;
use crate::is_gizmos_active;
use bevy::{
    app::{App, Plugin},
    ecs::schedule::IntoScheduleConfigs,
};
use bevy_egui::EguiPrimaryContextPass;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // Schedule system
            //
            .add_systems(
                EguiPrimaryContextPass,
                (editor_gizmos_ui).run_if(is_gizmos_active),
            );
    }
}
