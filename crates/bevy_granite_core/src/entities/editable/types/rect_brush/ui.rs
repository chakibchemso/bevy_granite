use super::RectBrush;
use bevy::math::{Vec2, Vec3};
use bevy_egui::egui;

impl RectBrush {
    /// Function to edit self's data via UI side panel
    /// We have a sister system that pushes changes to world entity - can be found inside 'update_event.rs'
    /// When true, sends an update to propagate these vars to the world's entity
    pub fn edit_via_ui(&mut self, ui: &mut egui::Ui, spacing: (f32, f32, f32)) -> bool {
        let rect_data = self;
        let normal_spacing = spacing.2;
        let small_spacing = spacing.0;
        let large_spacing = spacing.1;
        let mut changed = false;

        let style = ui.ctx().style().clone();
        let default_font_id = egui::FontId::default();
        let font_id = style
            .text_styles
            .get(&egui::TextStyle::Button)
            .unwrap_or(&default_font_id);
        let btn_height = font_id.size + style.spacing.button_padding.y * 2.0;
        let drag_size = [60., btn_height];

        ui.label(egui::RichText::new("Rectangle Brush Data").italics());
        ui.add_space(large_spacing);

        // Size
        ui.vertical(|ui| {
            ui.label("Brush Size:");
            ui.add_space(normal_spacing);

            ui.horizontal(|ui| {
                egui::Grid::new("size_grid")
                    .num_columns(4)
                    .spacing([1.0, 0.0])
                    .show(ui, |ui| {
                        changed |= ui
                            .add_sized(
                                drag_size,
                                egui::DragValue::new(&mut rect_data.size.x)
                                    .speed(0.1)
                                    .fixed_decimals(2),
                            )
                            .changed();
                        changed |= ui
                            .add_sized(
                                drag_size,
                                egui::DragValue::new(&mut rect_data.size.y)
                                    .speed(0.1)
                                    .fixed_decimals(2),
                            )
                            .changed();
                        changed |= ui
                            .add_sized(
                                drag_size,
                                egui::DragValue::new(&mut rect_data.size.z)
                                    .speed(0.1)
                                    .fixed_decimals(2),
                            )
                            .changed();
                        if ui
                            .add_sized(drag_size, egui::Button::new("Reset"))
                            .clicked()
                        {
                            rect_data.size = Vec3::ONE;
                            changed = true;
                        }
                    });
            });
        });

        ui.add_space(large_spacing);

        // UV Scale
        ui.vertical(|ui| {
            ui.label("UV:");
            ui.add_space(small_spacing);
            ui.horizontal(|ui| {
                egui::Grid::new("uv_grid")
                    .num_columns(3)
                    .spacing([1.0, 0.0])
                    .show(ui, |ui| {
                        changed |= ui
                            .add_sized(
                                drag_size,
                                egui::DragValue::new(&mut rect_data.uv_scale.x)
                                    .speed(0.01)
                                    .fixed_decimals(2),
                            )
                            .changed();
                        changed |= ui
                            .add_sized(
                                drag_size,
                                egui::DragValue::new(&mut rect_data.uv_scale.y)
                                    .speed(0.01)
                                    .fixed_decimals(2),
                            )
                            .changed();
                        if ui
                            .add_sized(drag_size, egui::Button::new("Reset"))
                            .clicked()
                        {
                            rect_data.uv_scale = Vec2::ONE;
                            changed = true;
                        }
                    });
            });
        });

        changed
    }
}
