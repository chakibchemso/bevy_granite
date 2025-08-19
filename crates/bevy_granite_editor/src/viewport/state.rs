use crate::viewport::VisualizationConfig;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ViewportState {
    pub visualizers: VisualizationConfig,
    pub grid: bool,
    pub grid_distance: f32,
    pub grid_color: [f32; 4],
    pub grid_size: f32,

    #[serde(skip)]
    pub changed: bool,
}
impl Default for ViewportState {
    fn default() -> Self {
        ViewportState {
            grid: true,
            grid_distance: 100.,
            grid_color: [0.124, 0.124, 0.124, 1.0],
            grid_size: 1.,
            visualizers: VisualizationConfig::default(),
            changed: true,
        }
    }
}
