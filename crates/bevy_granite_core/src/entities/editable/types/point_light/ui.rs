use crate::GraniteType;

use super::PointLightData;
use bevy_egui::egui;

impl PointLightData {
    /// Function to edit self's data via UI side panel
    /// We have a sister system that pushes changes to world entity - can be found inside 'update_event.rs'
    /// When true, sends an update to propagate these vars to the world's entity
    pub fn edit_via_ui(&mut self, ui: &mut egui::Ui, spacing: (f32, f32, f32)) -> bool {
        let type_name = self.type_name();
        let data = self;
        let large_spacing = spacing.1;
        ui.label(egui::RichText::new(type_name).italics());
        ui.add_space(large_spacing);

        let mut changed = false;
        ui.vertical(|ui| {
            let mut color_array = [
                (data.color.0 * 255.0) as u8,
                (data.color.1 * 255.0) as u8,
                (data.color.2 * 255.0) as u8,
            ];

            egui::Grid::new("point_light_data_grid")
                .num_columns(2)
                .spacing([large_spacing, large_spacing])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Color:");
                    if ui.color_edit_button_srgb(&mut color_array).changed() {
                        data.color = (
                            color_array[0] as f32 / 255.0,
                            color_array[1] as f32 / 255.0,
                            color_array[2] as f32 / 255.0,
                        );
                        changed = true;
                    }
                    ui.end_row();

                    ui.label("Intensity:");
                    changed |= ui
                        .add(
                            egui::DragValue::new(&mut data.intensity)
                                .range(0.0..=4_000_000.0)
                                .speed(2000.0)
                                .suffix(" lm"),
                        )
                        .changed();
                    ui.end_row();

                    ui.label("Range:");
                    changed |= ui
                        .add(
                            egui::DragValue::new(&mut data.range)
                                .range(0.0..=200.0)
                                .speed(0.1),
                        )
                        .changed();
                    ui.end_row();
                    ui.label("Shadows Enabled:");
                    changed |= ui.checkbox(&mut data.shadows_enabled, "").changed();
                    ui.end_row();
                });
        });
        changed
    }
}
