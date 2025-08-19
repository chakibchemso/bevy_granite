use bevy_egui::egui;

use super::{
    data::EntityEditorTabData,
    widgets::{
        entity_component_widget, entity_identity_widget,
        entity_name_widget, entity_transform_widget,
    },
};

pub fn entity_editor_tab_ui(ui: &mut egui::Ui, data: &mut EntityEditorTabData) {
    entity_name_widget(ui, data);
    entity_transform_widget(ui, data);
    entity_identity_widget(ui, data);
    entity_component_widget(ui, data);
}
