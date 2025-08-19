use crate::GraniteType;
use super::Camera3D;
use bevy_egui::egui;

impl Camera3D {
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
        let mut fog_enabled = &mut data.has_volumetric_fog;
        ui.vertical(|ui| {
            egui::Grid::new("camera_settings_grid")
                .num_columns(2)
                .spacing([large_spacing, large_spacing])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Is active:");
                    changed |= ui.checkbox(&mut data.is_active, "").changed();
                    ui.end_row();
                    ui.label("Volumetric Fog:");
                    changed |= ui.checkbox(&mut fog_enabled, "").changed();
                    ui.end_row();
                });
            ui.add_space(large_spacing);
            if *fog_enabled {
                ui.collapsing("Volumetric Fog", |ui| {
                    egui::Grid::new("volumetric_fog_grid")
                        .num_columns(2)
                        .spacing([large_spacing, large_spacing])
                        .striped(true)
                        .show(ui, |ui| {
                            let found_fog = &mut data.volumetric_fog_settings;

                            if let Some(fog_settings) = found_fog {
                                ui.label("Fog Color:");
                                let mut fog_color_array = [
                                    (fog_settings.fog_color.to_srgba().red * 255.0) as u8,
                                    (fog_settings.fog_color.to_srgba().green * 255.0) as u8,
                                    (fog_settings.fog_color.to_srgba().blue * 255.0) as u8,
                                ];
                                if ui.color_edit_button_srgb(&mut fog_color_array).changed() {
                                    fog_settings.fog_color = bevy::prelude::Color::srgb(
                                        fog_color_array[0] as f32 / 255.0,
                                        fog_color_array[1] as f32 / 255.0,
                                        fog_color_array[2] as f32 / 255.0,
                                    );
                                    changed = true;
                                }
                                ui.end_row();

                                ui.label("Ambient Color:");
                                let mut ambient_color_array = [
                                    (fog_settings.ambient_color.to_srgba().red * 255.0) as u8,
                                    (fog_settings.ambient_color.to_srgba().green * 255.0) as u8,
                                    (fog_settings.ambient_color.to_srgba().blue * 255.0) as u8,
                                ];
                                if ui
                                    .color_edit_button_srgb(&mut ambient_color_array)
                                    .changed()
                                {
                                    fog_settings.ambient_color = bevy::prelude::Color::srgb(
                                        ambient_color_array[0] as f32 / 255.0,
                                        ambient_color_array[1] as f32 / 255.0,
                                        ambient_color_array[2] as f32 / 255.0,
                                    );
                                    changed = true;
                                }
                                ui.end_row();

                                ui.label("Ambient Intensity:");
                                changed |= ui
                                    .add(
                                        egui::DragValue::new(&mut fog_settings.ambient_intensity)
                                            .range(0.0..=10.0)
                                            .speed(0.01),
                                    )
                                    .changed();
                                ui.end_row();

                                ui.label("Step Count:");
                                changed |= ui
                                    .add(
                                        egui::DragValue::new(&mut fog_settings.step_count)
                                            .range(1..=256)
                                            .speed(1),
                                    )
                                    .changed();
                                ui.end_row();

                                ui.label("Max Depth:");
                                changed |= ui
                                    .add(
                                        egui::DragValue::new(&mut fog_settings.max_depth)
                                            .range(0.1..=1000.0)
                                            .speed(1.0),
                                    )
                                    .changed();
                                ui.end_row();

                                ui.label("Absorption:");
                                changed |= ui
                                    .add(
                                        egui::DragValue::new(&mut fog_settings.absorption)
                                            .range(0.0..=1.0)
                                            .speed(0.001),
                                    )
                                    .changed();
                                ui.end_row();

                                ui.label("Scattering:");
                                changed |= ui
                                    .add(
                                        egui::DragValue::new(&mut fog_settings.scattering)
                                            .range(0.0..=1.0)
                                            .speed(0.001),
                                    )
                                    .changed();
                                ui.end_row();

                                ui.label("Density:");
                                changed |= ui
                                    .add(
                                        egui::DragValue::new(&mut fog_settings.density)
                                            .range(0.0..=1.0)
                                            .speed(0.001),
                                    )
                                    .changed();
                                ui.end_row();

                                ui.label("Scattering Asymmetry:");
                                changed |= ui
                                    .add(
                                        egui::DragValue::new(
                                            &mut fog_settings.scattering_asymmetry,
                                        )
                                        .range(-1.0..=1.0)
                                        .speed(0.01),
                                    )
                                    .changed();
                                ui.end_row();

                                ui.label("Light Tint:");
                                let mut light_tint_array = [
                                    (fog_settings.light_tint.to_srgba().red * 255.0) as u8,
                                    (fog_settings.light_tint.to_srgba().green * 255.0) as u8,
                                    (fog_settings.light_tint.to_srgba().blue * 255.0) as u8,
                                ];
                                if ui.color_edit_button_srgb(&mut light_tint_array).changed() {
                                    fog_settings.light_tint = bevy::prelude::Color::srgb(
                                        light_tint_array[0] as f32 / 255.0,
                                        light_tint_array[1] as f32 / 255.0,
                                        light_tint_array[2] as f32 / 255.0,
                                    );
                                    changed = true;
                                }
                                ui.end_row();

                                ui.label("Light Intensity:");
                                changed |= ui
                                    .add(
                                        egui::DragValue::new(&mut fog_settings.light_intensity)
                                            .range(0.0..=10.0)
                                            .speed(0.01),
                                    )
                                    .changed();
                                ui.end_row();
                            };
                        });
                });
            };
        });
        changed
    }
}
