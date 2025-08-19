use super::{capture_input_events, update_mouse_pos, CursorWindowPos, UserInput};
use bevy::app::{Plugin, PreUpdate};
use bevy::prelude::{App, Update};

pub struct SharedPlugin;
impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // Resources
            //
            .insert_resource(UserInput::default())
            .insert_resource(CursorWindowPos::default())
            //
            // Schedule systems
            //
            .add_systems(PreUpdate, capture_input_events)
            .add_systems(Update, update_mouse_pos);
    }
}
