use super::filter_and_format_logs_rich;
use crate::interface::panels::bottom_panel::{BottomDockState, BottomTab};

use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};
use bevy_granite_logging::LOG_BUFFER;

pub fn update_log_tab_system(mut bottom_dock: ResMut<BottomDockState>, mut contexts: EguiContexts) {
    let log_entries = LOG_BUFFER.lock().unwrap().clone();
    let ctx = contexts.ctx_mut().expect("Egui context to exist");
    let style = (*ctx.style()).clone();

    let default_font_id = egui::FontId::default();
    let monospace_font_id = style
        .text_styles
        .get(&egui::TextStyle::Monospace)
        .unwrap_or(&default_font_id)
        .clone();

    for (_, tab) in bottom_dock.dock_state.iter_all_tabs_mut() {
        if let BottomTab::Log { ref mut data, .. } = tab {
            data.formatted_log_cache = filter_and_format_logs_rich(
                &log_entries,
                &data.filter,
                &data.search_query,
                monospace_font_id.clone(),
            );
            data.last_log_count = log_entries.len();
        }
    }
}
