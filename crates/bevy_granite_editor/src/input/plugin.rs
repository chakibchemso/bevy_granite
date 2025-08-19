use super::shortcuts_system;
use bevy::app::{App, Plugin, Update};

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // Schedule system
            //
            .add_systems(
                Update,
                (
                    // always runs
                    // allows editor/toggle
                    shortcuts_system,
                ),
            );
    }
}
