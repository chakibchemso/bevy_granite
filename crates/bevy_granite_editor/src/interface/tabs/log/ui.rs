use bevy_egui::egui::{self, Color32, FontId, Margin};
use bevy_granite_logging::{
    config::{LogCategory, LogLevel, LogType},
    LogEntry, RgbaColor,
};
use egui::Frame;

fn to_color32(color: RgbaColor) -> Color32 {
    Color32::from_rgba_premultiplied(color.0, color.1, color.2, color.3)
}

use egui::{text::LayoutJob, text::TextFormat};

pub fn filter_and_format_logs_rich(
    entries: &[LogEntry],
    filter: &LogFilter,
    search: &str,
    font_id: FontId,
) -> Vec<LayoutJob> {
    entries
        .iter()
        .rev() // Start from the newest
        .filter(|entry| {
            filter.enabled_types.contains(&entry.log_type)
                && filter.enabled_levels.contains(&entry.level)
                && filter.enabled_categories.contains(&entry.category)
                && (search.is_empty()
                    || entry
                        .message
                        .to_lowercase()
                        .contains(&search.to_lowercase()))
        })
        .take(filter.max_display_logs) // Only keep the most recent N entries
        .collect::<Vec<_>>() // Must collect here to reverse again
        .into_iter()
        .rev() // So logs are displayed oldest to newest
        .map(|entry| {
            let mut job = LayoutJob::default();

            if !filter.only_message {
                job.append(
                    &entry.timestamp.to_string(),
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: Color32::GRAY,
                        ..Default::default()
                    },
                );

                job.append(
                    &format!("{:?} ", entry.log_type),
                    0.0,
                    TextFormat {
                        font_id: font_id.clone(),
                        color: Color32::GRAY,
                        ..Default::default()
                    },
                );

                if entry.category != LogCategory::Blank {
                    job.append(
                        &format!("[{:?}] ", entry.category),
                        0.0,
                        TextFormat {
                            font_id: font_id.clone(),
                            color: to_color32(entry.category.ui_color()),
                            ..Default::default()
                        },
                    );
                }

                if !matches!(entry.level, LogLevel::Info) {
                    job.append(
                        &format!("({:?}) ", entry.level),
                        0.0,
                        TextFormat {
                            font_id: font_id.clone(),
                            color: to_color32(entry.level.ui_color()),
                            ..Default::default()
                        },
                    );
                }
            }

            job.append(
                &entry.message,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color: to_color32(entry.level.ui_color()),
                    ..Default::default()
                },
            );

            job
        })
        .collect()
}

// TODO:
// add this filter to editor settings for persistence

#[derive(PartialEq, Eq, Debug, Default, Hash, Clone)]
pub struct LogFilter {
    only_message: bool,
    enabled_types: Vec<LogType>,
    enabled_levels: Vec<LogLevel>,
    enabled_categories: Vec<LogCategory>,
    max_display_logs: usize,
}

#[derive(PartialEq, Clone)]
pub struct LogTabData {
    pub filter: LogFilter,
    pub formatted_log_cache: Vec<LayoutJob>,
    pub last_log_count: usize,
    pub search_query: String,
}

impl Default for LogTabData {
    fn default() -> Self {
        Self {
            filter: LogFilter {
                only_message: false,
                max_display_logs: 500,
                enabled_types: LogType::all(),
                enabled_levels: LogLevel::minimal(),
                enabled_categories: LogCategory::all(),
            },
            formatted_log_cache: Vec::new(),
            last_log_count: 0,
            search_query: String::new(),
        }
    }
}

pub fn log_tab_ui(ui: &mut egui::Ui, data: &mut LogTabData) {
    let spacing = crate::UI_CONFIG.spacing;
    let small_spacing = crate::UI_CONFIG.small_spacing;
    let large_spacing = crate::UI_CONFIG.large_spacing;
    ui.separator();
    ui.add_space(spacing);

    let available = egui::vec2(ui.available_width(), ui.available_height() - spacing);
    let filter_width = ui.available_width() / 4.;

    ui.allocate_ui_with_layout(
        available,
        egui::Layout::left_to_right(egui::Align::Min),
        |ui| {
            // Left side: Search + Log display
            ui.allocate_ui(egui::vec2(available.x - filter_width, available.y), |ui| {
                ui.vertical(|ui| {
                    // Search bar
                    ui.horizontal(|ui| {
                        ui.add_space(large_spacing);
                        ui.label("Search:");
                        ui.add_space(large_spacing);
                        ui.text_edit_singleline(&mut data.search_query);
                    });
                    ui.add_space(spacing);
                    ui.separator();
                    ui.add_space(spacing);

                    // Log display area
                    let extreme_bg = ui.ctx().style().visuals.extreme_bg_color;
                    Frame::NONE
                        .fill(extreme_bg)
                        .inner_margin(Margin::same(large_spacing as i8))
                        .show(ui, |ui| {
                            egui::ScrollArea::vertical()
                                .stick_to_bottom(true)
                                .show(ui, |ui| {
                                    ui.add_space(large_spacing);
                                    ui.set_min_height(ui.available_height());
                                    ui.set_min_width(ui.available_width());
                                    ui.vertical(|ui| {
                                        for job in &data.formatted_log_cache {
                                            ui.label(job.clone());
                                        }
                                    });
                                });
                        });
                });
            });

            ui.separator();
            ui.add_space(spacing);

            // Filter panel (fixed width, full height)
            ui.allocate_ui(egui::vec2(filter_width - 30., available.y), |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.group(|ui| {
                        filter_multi_select(
                            ui,
                            "Type",
                            &mut data.filter.enabled_types,
                            LogType::all(),
                        );
                        filter_multi_select(
                            ui,
                            "Level",
                            &mut data.filter.enabled_levels,
                            LogLevel::all(),
                        );
                        filter_multi_select(
                            ui,
                            "Category",
                            &mut data.filter.enabled_categories,
                            LogCategory::all(),
                        );

                        ui.separator();
                        ui.add_space(spacing);

                        ui.horizontal(|ui| {
                            if ui.button("All").clicked() {
                                data.filter.enabled_types = LogType::all();
                                data.filter.enabled_levels = LogLevel::all();
                                data.filter.enabled_categories = LogCategory::all();
                            }
                            ui.separator();
                            if ui.button("None").clicked() {
                                data.filter.enabled_types.clear();
                                data.filter.enabled_levels.clear();
                                data.filter.enabled_categories.clear();
                            }
                            ui.add_space(small_spacing);
                            if ui.button("Minimal").clicked() {
                                data.filter.enabled_types = LogType::all();
                                data.filter.enabled_levels = LogLevel::minimal();
                                data.filter.enabled_categories = LogCategory::all();
                            }
                            ui.add_space(small_spacing);
                            if ui.button("Info").clicked() {
                                data.filter.enabled_types = LogType::all();
                                data.filter.enabled_levels = LogLevel::info();
                                data.filter.enabled_categories = LogCategory::all();
                            }
                            ui.add_space(small_spacing);
                            if ui.button("Errors").clicked() {
                                data.filter.enabled_types = LogType::all();
                                data.filter.enabled_levels = LogLevel::errors();
                                data.filter.enabled_categories = LogCategory::all();
                            }
                        });
                    });

                    ui.add_space(spacing);
                    ui.group(|ui| {
                        ui.set_min_width(ui.available_width());
                        ui.add_space(spacing);

                        ui.horizontal(|ui| {
                            ui.add_space(spacing);
                            ui.checkbox(&mut data.filter.only_message, "Hide metadata")
                        });

                        ui.add_space(spacing);

                        ui.horizontal(|ui| {
                            ui.add(
                                egui::DragValue::new(&mut data.filter.max_display_logs)
                                    .range(50..=7_500)
                                    .speed(50)
                                    .suffix(" log lines"),
                            );
                        });
                    });
                });
            });
        },
    );
}

fn filter_multi_select<T: std::fmt::Debug + Eq + Clone>(
    ui: &mut egui::Ui,
    label: &str,
    selected: &mut Vec<T>,
    options: impl IntoIterator<Item = T>,
) {
    ui.collapsing(label, |ui| {
        for option in options {
            let mut is_selected = selected.contains(&option);
            if ui
                .checkbox(&mut is_selected, format!("{:?}", option))
                .changed()
            {
                if is_selected {
                    selected.push(option.clone());
                } else {
                    selected.retain(|x| x != &option);
                }
            }
        }
    });
}
