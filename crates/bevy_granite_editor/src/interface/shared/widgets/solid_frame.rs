use bevy_egui::egui::{self, Color32};


pub fn make_frame_solid(mut frame: egui::Frame, ui: &egui::Ui) -> egui::Frame {
    if frame.fill.a() < 255 {
        let window_color = ui.ctx().style().visuals.widgets.hovered.bg_fill;
        frame.fill = Color32::from_rgb(window_color.r(), window_color.g(), window_color.b());
    }
    frame
}

pub fn make_frame_solid_via_context(mut frame: egui::Frame, ctx: &egui::Context) -> egui::Frame {
    if frame.fill.a() < 255 {
        let window_color = ctx.style().visuals.widgets.hovered.bg_fill;
        frame.fill = Color32::from_rgb(window_color.r(), window_color.g(), window_color.b());
    }
    frame
}