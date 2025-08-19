use crate::GraniteType;

use super::DirLight;
use bevy_egui::egui;

impl DirLight {
    /// Function to edit self's data via UI side panel
    /// We have a sister system that pushes changes to world entity - can be found inside 'update_event.rs'
    /// When true, sends an update to propagate these vars to the world's entity
    pub fn edit_via_ui(
        &mut self,
        ui: &mut egui::Ui,
        // Small, Large, Normal
        spacing: (f32, f32, f32),
    ) -> bool {
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

            egui::Grid::new("directional_light_data_grid")
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

                    ui.label("Illuminance:");
                    changed |= ui
                        .add(
                            egui::DragValue::new(&mut data.illuminance)
                                .range(0.0..=200_000.0)
                                .speed(2000.0)
                                .suffix(" lm"),
                        )
                        .on_hover_text(
                            "Intensity in lumens. Direct sunlight ~32,000. Office lighting ~400",
                        )
                        .changed();
                    ui.end_row();

                    ui.label("Shadows Enabled:");
                    changed |= ui.checkbox(&mut data.shadows_enabled, "").changed();
                    ui.end_row();

                    ui.label("Volumetric Fog:");
                    changed |= ui
                        .checkbox(&mut data.volumetric, "(Settings on Camera)")
                        .changed();
                    ui.end_row();
                });
        });
        changed
    }
}
