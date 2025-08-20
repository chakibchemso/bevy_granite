use super::editor_gizmos_ui;
use crate::is_gizmos_active;
use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::IntoScheduleConfigs,
};

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // Schedule system
            //
            .add_systems(Update, (editor_gizmos_ui).run_if(is_gizmos_active));
    }
}
