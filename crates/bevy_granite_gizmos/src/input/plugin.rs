use super::{watch_gizmo_change, DragState};
use crate::{is_gizmos_active, GizmoVisibilityState};
use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::IntoScheduleConfigs,
};

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // Resources
            //
            .insert_resource(GizmoVisibilityState::default())
            .insert_resource(DragState::default())
            //
            // Schedule system
            //
            .add_systems(Update, (watch_gizmo_change,).run_if(is_gizmos_active));
    }
}
