use bevy::prelude::Resource;
use bevy_egui::egui;
use egui_dock::{DockState, NodeIndex, TabViewer};
use serde::{Deserialize, Serialize};

use crate::interface::{tabs::{debug_tab_ui, log_tab_ui}, DebugTabData, LogTabData};

#[derive(Resource, Clone)]
pub struct BottomDockState {
    pub dock_state: DockState<BottomTab>,
}

impl Default for BottomDockState {
    fn default() -> Self {
        let log_tab = BottomTab::Log {
            data: LogTabData::default(),
        };
        let debug_tab = BottomTab::Debug {
            data: DebugTabData::default(),
        };

        let mut dock_state = DockState::new(vec![debug_tab]);

        let surface = dock_state.main_surface_mut();

        let [_debug_node, _log_node] = surface.split_right(NodeIndex::root(), 0.1, vec![log_tab]);

        Self { dock_state }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum BottomTab {
    Log {
        #[serde(skip)]
        data: LogTabData,
    },
    Debug {
        #[serde(skip)]
        data: DebugTabData,
    },
}

#[derive(Resource)]
pub struct BottomTabViewer;

impl TabViewer for BottomTabViewer {
    type Tab = BottomTab;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            BottomTab::Log { data, .. } => log_tab_ui(ui, data),
            BottomTab::Debug { data, .. } => debug_tab_ui(ui, data),
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            BottomTab::Log { .. } => "Log".into(),
            BottomTab::Debug { .. } => "Debug".into(),
        }
    }
}
