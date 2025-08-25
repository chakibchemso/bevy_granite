use std::borrow::Cow;

use crate::interface::{
    shared::widgets::combobox::component_selector_combo, tabs::EntityEditorTabData,
};
use bevy_egui::egui;
use bevy_granite_core::ReflectedComponent;

// the registered holds the actual registered and runtime editable components
#[derive(Default, PartialEq, Clone)]
pub struct EntityRegisteredData {
    pub components: Vec<ReflectedComponent>,
    pub registered_data_changed: bool,
    pub registered_add_request: Option<String>,
    pub registered_remove_request: Option<String>,
}

impl EntityRegisteredData {
    pub fn clear(&mut self) {
        self.components.clear();
        self.registered_data_changed = false;
        self.registered_add_request = None;
        self.registered_remove_request = None;
    }
}

pub fn entity_component_widget(ui: &mut egui::Ui, data: &mut EntityEditorTabData) {
    let large_spacing = crate::UI_CONFIG.large_spacing;
    // --------------------------------------------------------------------
    // COMPONENTS
    // --------------------------------------------------------------------
    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.add_space(large_spacing);
            ui.vertical(|ui| {
                display_entity_components(ui, data);
            });
            ui.add_space(large_spacing);
        });
    });
}

fn display_entity_components(ui: &mut egui::Ui, data: &mut EntityEditorTabData) {
    let large_spacing = crate::UI_CONFIG.large_spacing;
    let registered_type_names = data.registered_type_names.clone();
    let entity_registered_requested = &mut data.registered_data.registered_add_request;
    let entity_component_changed = &mut data.registered_data.registered_data_changed;
    let entity_component_remove = &mut data.registered_data.registered_remove_request;
    let search_filter = &mut data.component_search_filter;
    let Some(ref component_editor) = data.component_editor else {
        ui.label("Component editor not initialized");
        return;
    };
    let type_registry = component_editor.type_registry.read();
    ui.add_space(large_spacing);
    display_add_registered_component(
        ui,
        entity_component_changed,
        entity_registered_requested,
        registered_type_names,
        &data.registered_data.components,
        search_filter,
    );

    for (index, component) in data.registered_data.components.iter_mut().enumerate() {
        let friendly_name = component
            .type_name
            .split("::")
            .last()
            .unwrap_or(&component.type_name)
            .to_string();

        let mut is_open = false;
        ui.horizontal(|ui| {
            ui.set_width(ui.available_width() - large_spacing);
            // Header
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                let header_response = egui::CollapsingHeader::new(&friendly_name)
                    .show_background(false)
                    .show(ui, |_ui| {});
                is_open = header_response.openness > 0.0;
            });

            // Spacer + Delete button
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(large_spacing);
                if ui.small_button("Delete").clicked() {
                    *entity_component_remove = Some(component.type_name.to_string());
                    *entity_component_changed = true;
                }
            });
        });

        if is_open {
            ui.indent(index, |ui| {
                let original_spacing = ui.spacing().item_spacing;
                ui.spacing_mut().item_spacing = egui::vec2(10.0, 2.0);
                if bevy_inspector_egui::reflect_inspector::ui_for_value(
                    component.reflected_data.as_mut(),
                    ui,
                    &type_registry,
                ) {
                    *entity_component_changed = true;
                }

                ui.spacing_mut().item_spacing = original_spacing;
            });
        }
    }
}

fn display_add_registered_component(
    ui: &mut egui::Ui,
    component_changed: &mut bool,
    registered_add_request: &mut Option<String>,
    registered_type_names: Vec<Cow<'static, str>>,
    existing_components: &[ReflectedComponent],
    search_filter: &mut String,
) {
    let large_spacing = crate::UI_CONFIG.large_spacing;
    // Create a set of existing component type names for fast lookup
    let existing_type_names: std::collections::HashSet<Cow<'static, str>> = existing_components
        .iter()
        .map(|comp| comp.type_name.clone())
        .collect();

    // Filter out components that are already on the entity
    let available_components: Vec<_> = registered_type_names
        .iter()
        .filter(|name| !existing_type_names.contains(name.as_ref()))
        .cloned()
        .collect();

    ui.horizontal(|ui| {
        ui.set_width(ui.available_width() - 8.);
        component_selector_combo(
            ui,
            search_filter,
            available_components,
            existing_components,
            component_changed,
            registered_add_request,
        );
    });

    ui.add_space(large_spacing);
}
