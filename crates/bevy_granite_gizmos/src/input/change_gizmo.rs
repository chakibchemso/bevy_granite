use crate::gizmos::{GizmoType, SelectedGizmo};
use bevy::{
    ecs::system::{Res, ResMut},
    input::keyboard::KeyCode,
};
use bevy_granite_core::{InputTypes, UserInput};
use bevy_granite_logging::{
    config::{LogCategory, LogLevel, LogType},
    log,
};

pub fn watch_gizmo_change(user_input: Res<UserInput>, mut selected_gizmo: ResMut<SelectedGizmo>) {
    // By grabbing the list of inputs, we can ensure only that key is pressed
    let allow_transform = user_input.current_button_inputs.len() == 1
        && user_input.current_button_inputs[0] == InputTypes::Button(KeyCode::KeyW)
        && !user_input.mouse_over_egui;

    let allow_rotate = user_input.current_button_inputs.len() == 1
        && user_input.current_button_inputs[0] == InputTypes::Button(KeyCode::KeyE)
        && !user_input.mouse_over_egui;

    let allow_pointer = user_input.current_button_inputs.len() == 1
        && user_input.current_button_inputs[0] == InputTypes::Button(KeyCode::KeyQ)
        && !user_input.mouse_over_egui;

    if allow_transform && !matches!(selected_gizmo.value, GizmoType::Transform) {
        selected_gizmo.value = GizmoType::Transform;
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Input,
            "(shortcut) Toggling gizmo to Transform"
        );
    }

    if allow_rotate && !matches!(selected_gizmo.value, GizmoType::Rotate) {
        selected_gizmo.value = GizmoType::Rotate;
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Input,
            "(shortcut) Toggling gizmo to Rotate"
        );
    }

    if allow_pointer && !matches!(selected_gizmo.value, GizmoType::Pointer) {
        selected_gizmo.value = GizmoType::Pointer;
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Input,
            "(shortcut) Toggling gizmo to Pointer"
        );
    }
}
