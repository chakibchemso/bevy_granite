use crate::interface::tabs::EntityEditorTabData;
use bevy_egui::egui;
use bevy_granite_logging::{
    config::{LogCategory, LogLevel, LogType},
    log,
};

pub fn entity_name_widget(ui: &mut egui::Ui, data: &mut EntityEditorTabData) {
    let large_spacing = crate::UI_CONFIG.large_spacing;
    // --------------------------------------------------------------------
    // NAME
    // --------------------------------------------------------------------
    let name = &mut data.identity_data.name;

    let style = ui.ctx().style().clone();
    let default_font_id = egui::FontId::default();

    let font_id = style
        .text_styles
        .get(&egui::TextStyle::Button)
        .unwrap_or(&default_font_id);

    let btn_height = font_id.size + style.spacing.button_padding.y * 2.0;

    ui.group(|ui| {
        ui.set_width(ui.available_width());
        ui.add_space(large_spacing);
        ui.horizontal(|ui| {
            ui.add_space(large_spacing);
            egui::Grid::new("name_grid")
                .num_columns(2)
                .spacing([large_spacing, large_spacing])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Name:");
                    let response = ui.add(egui::TextEdit::singleline(name).min_size(egui::Vec2 {
                        x: 60.,
                        y: btn_height,
                    }));
                    if response.changed() {
                        data.identity_data.name_changed = true;
                        log!(
                            LogType::Editor,
                            LogLevel::Info,
                            LogCategory::UI,
                            "User edited name field"
                        );
                    }

                    ui.end_row();
                });
            ui.add_space(large_spacing);
        });
        ui.add_space(large_spacing);
    });
}
