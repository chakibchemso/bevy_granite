use bevy_egui::egui;
use bevy_granite_core::{entities::GraniteType, GraniteTypes, EditableMaterial, NewEditableMaterial, ClassCategory};
use bevy_granite_logging::{
    config::{LogCategory, LogLevel, LogType},
    log,
};

use crate::interface::tabs::{
    entity_editor::widgets::material_editor::{
        display_material_creation, display_material_edit, display_material_selector_field,
        display_material_settings,
    },
    EntityEditorTabData,
};

#[derive(Default, PartialEq, Clone)]
pub struct EntityIdentityData {
    pub name: String,
    pub name_changed: bool,
    pub class_data: GraniteTypes,
    pub class_data_changed: bool,
}

pub fn entity_identity_widget(ui: &mut egui::Ui, data: &mut EntityEditorTabData) {
    let large_spacing = crate::UI_CONFIG.large_spacing;
    // --------------------------------------------------------------------
    // CLASS IDENTITY
    // --------------------------------------------------------------------

    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.add_space(large_spacing);
        ui.horizontal(|ui| {
            ui.add_space(large_spacing);
            display_class_specific_data(ui, data);
            ui.add_space(large_spacing);
        });
        ui.add_space(large_spacing);
    });
}

fn display_class_specific_data(ui: &mut egui::Ui, tab_data: &mut EntityEditorTabData) {
    // Display the class-specific data based on the class type
    let spacing = (
        crate::UI_CONFIG.small_spacing,
        crate::UI_CONFIG.large_spacing,
        crate::UI_CONFIG.spacing,
    );

    ui.vertical(|ui| {
        ui.add_space(spacing.1);

        let mut class_data = tab_data.identity_data.class_data.clone();
        let mut changed = class_data.edit_via_ui(ui, spacing);

        // Display material editor
        if matches!(
            class_data.category(),
            ClassCategory::Mesh
        ) {
            if let Some(material_data) = class_data.get_mut_material_data() {
                ui.add_space(spacing.1);
                changed |= display_material_data(
                    ui,
                    material_data.current,
                    material_data.last,
                    material_data.path,
                    tab_data,
                );
            }
        }

        if changed {
            tab_data.identity_data.class_data = class_data;
            tab_data.identity_data.class_data_changed = true;
        }
    });
}

fn display_material_data(
    ui: &mut egui::Ui,
    current_material: &mut EditableMaterial,
    last_material: &mut EditableMaterial,
    class_material_path: &mut String,
    tab_data: &mut EntityEditorTabData,
) -> bool {
    let spacing = crate::UI_CONFIG.spacing;
    let large_spacing = crate::UI_CONFIG.large_spacing;
    let material_search_filter = &mut tab_data.material_search_filter;
    let material_to_build = &mut tab_data.material_to_build;
    let material_builder_open = &mut tab_data.material_builder_open;
    let surface_collapsed_state = &mut tab_data.surface_collapsed_state;
    let settings_collapsed_state = &mut tab_data.settings_collapsed_state;
    let available_materials = &mut tab_data.available_materials;

    let mut changed = false;
    let mut edit_changes = false;

    ui.collapsing("Material Properties", |ui| {
        ui.set_width(ui.available_width() - 8.);
        ui.add_space(spacing);

        ui.horizontal(|ui| {
            if *material_builder_open {
                ui.disable();
            }
            ui.set_width(ui.available_width());
            let (material_changed, delete_clicked) = display_material_selector_field(
                ui,
                available_materials,
                material_builder_open,
                material_search_filter,
                class_material_path,
                current_material,
            );
            changed = material_changed;
            
            // Handle delete request
            if delete_clicked && !current_material.is_empty() && current_material.friendly_name != "None" {
                // Set a flag to indicate deletion was requested
                // We'll handle this in the tab update system
                tab_data.material_delete_requested = true;
            }
        });

        ui.add_space(large_spacing);
        ui.vertical(|ui| {
            if *material_builder_open {
                ui.disable();
            }
            let mut surface_open = *surface_collapsed_state;
            let mut settings_open = *settings_collapsed_state;
            if *material_builder_open {
                surface_open = false;
                settings_open = false;
            }

            // Material surface group
            let collapsing_response = egui::CollapsingHeader::new("Surface")
                .open(Some(surface_open))
                .show(ui, |ui| {
                    ui.set_max_width(ui.available_width());
                    edit_changes |= display_material_edit(ui, current_material);
                    if edit_changes {
                        current_material.disk_changes = edit_changes;
                        changed = true;
                    }
                });

            // Material settings group
            let collapsing_settings_response = egui::CollapsingHeader::new("Metadata")
                .open(Some(settings_open))
                .show(ui, |ui| {
                    edit_changes |= display_material_settings(ui, current_material);
                    if edit_changes {
                        current_material.disk_changes = edit_changes;
                        changed = true;
                    }
                });

            if !*material_builder_open {
                *surface_collapsed_state = if collapsing_response.header_response.clicked() {
                    !surface_open
                } else {
                    surface_open
                };

                *settings_collapsed_state =
                    if collapsing_settings_response.header_response.clicked() {
                        !settings_open
                    } else {
                        settings_open
                    };
            }
        });

        if *material_builder_open {
            ui.add_space(large_spacing);
            let (saved, canceled) = display_material_creation(ui, material_to_build);
            if saved {
                let file_rel_path = material_to_build.rel_path.clone()
                    + &material_to_build.friendly_name.to_lowercase()
                    + ".mat";

                *class_material_path = file_rel_path.clone();
                
                // If current material is None, create a new material instead of modifying it
                if current_material.friendly_name == "None" || current_material.is_empty() {
                    *current_material = EditableMaterial::get_new_unnamed_base_color();
                }
                
                current_material.new_material = true;
                current_material.path = file_rel_path.clone();
                current_material.friendly_name = material_to_build.friendly_name.clone();
                *last_material = current_material.clone();

                *material_builder_open = false;
                log!(
                    LogType::Editor,
                    LogLevel::Info,
                    LogCategory::UI,
                    "Save Material selected"
                );
            }
            if canceled {
                *material_builder_open = false;
                *material_to_build = NewEditableMaterial::default();
                current_material.new_material = false;
            }
            if saved {
                changed = true;
            }

            ui.add_space(large_spacing);
        }
    });

    changed
}
