use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct VisualizationConfig {
    pub selection_enabled: bool,
    pub selection_active_color: [f32; 3],
    pub selection_color: [f32; 3],
    pub selection_corner_length: f32,
    pub selection_bounds_offset: f32,
    pub selection_line_thickness: f32,

    pub debug_enabled: bool,
    pub debug_selected_only: bool,
    pub debug_relationship_lines: bool,
    pub debug_color: [f32; 3],
    pub debug_line_thickness: f32,

    pub icons_enabled: bool,
    pub icon_size: f32,
    pub icon_distance_scaling: bool,
    pub icon_max_distance: f32,
    pub icon_color: [f32; 3],
    pub icon_show_selected: bool,
    pub icon_show_active: bool,
}

impl Default for VisualizationConfig {
    fn default() -> Self {
        Self {
            selection_enabled: true,
            selection_active_color: [0.0, 0.03, 1.],
            selection_color: [0.3, 0.3, 0.3],
            selection_corner_length: 0.35,
            selection_bounds_offset: 0.10,
            selection_line_thickness: 3.0,
            debug_enabled: true,
            debug_selected_only: true,
            debug_relationship_lines: true,
            debug_color: [0.8, 1.0, 0.0],
            debug_line_thickness: 0.75,
            icons_enabled: true,
            icon_size: 0.2,
            icon_distance_scaling: false,
            icon_max_distance: 100.,
            icon_color: [1.0, 0.0, 0.0],
            icon_show_active: false,
            icon_show_selected: true,
        }
    }
}
