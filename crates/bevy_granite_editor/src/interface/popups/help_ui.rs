use crate::{
    editor_state::{update_editor_config_field, EditorState},
    interface::shared::widgets::make_frame_solid_via_context,
    HELP_CONFIG, UI_CONFIG,
};
use bevy::{ecs::system::ResMut, window::Window as BevyWindow};
use bevy_egui::{
    egui::{self, Window},
    EguiContexts,
};

pub fn help_ui(
    contexts: &mut EguiContexts,
    window: &BevyWindow,
    mut editor_state: ResMut<EditorState>,
) -> bool {
    let mut should_close = false;
    let mut start_enabled = editor_state.config.show_help_on_start;

    let window_size = egui::Vec2::new(window.width(), window.height());
    let default_size = egui::Vec2::new(window_size.x * 0.3, window_size.y * 0.8);
    let min_size = egui::Vec2::new(900.0, 700.0);
    let max_scroll_height = window_size.y * 0.6; // Large scroll area

    let spacing = UI_CONFIG.spacing;
    let large_spacing = UI_CONFIG.large_spacing;
    let _response = Window::new("Help")
        .default_size(default_size)
        .min_size(min_size)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .frame(make_frame_solid_via_context(
            egui::Frame::window(&contexts.ctx_mut().expect("Egui context to exist").style()),
            contexts.ctx_mut().expect("Egui context to exist"),
        ))
        .show(contexts.ctx_mut().expect("Egui context to exist"), |ui| {
            ui.vertical(|ui| {
                // Content area with horizontal padding
                ui.horizontal(|ui| {
                    ui.add_space(spacing);
                    ui.vertical(|ui| {
                        egui::ScrollArea::vertical()
                            .min_scrolled_height(window_size.y * 0.6)
                            .max_height(max_scroll_height)
                            .show(ui, |ui| {
                                ui.heading(HELP_CONFIG.header.to_string());
                                ui.add_space(spacing);
                                ui.label(HELP_CONFIG.body.to_string());
                                ui.add_space(spacing);
                                ui.label(HELP_CONFIG.youtube_text.to_string());
                                ui.add_space(spacing);
                                if ui.link(HELP_CONFIG.youtube_link.to_string()).clicked() {
                                    let _ = webbrowser::open(&HELP_CONFIG.youtube_link);
                                }
                            });
                        ui.add_space(spacing);
                        if ui.checkbox(&mut start_enabled, "Show on startup").changed() {
                            if let Err(e) =
                                update_editor_config_field(&mut editor_state, |config| {
                                    config.show_help_on_start = start_enabled;
                                })
                            {
                                eprintln!("Failed to update editor config: {}", e);
                            }
                        }
                    });
                    ui.add_space(spacing);
                });

                ui.add_space(large_spacing);
                if ui
                    .add_sized([ui.available_width(), 0.0], egui::Button::new("Close Help"))
                    .clicked()
                {
                    should_close = true;
                }
            });
        });
    should_close
}
