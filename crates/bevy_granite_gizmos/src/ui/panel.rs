use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

use crate::gizmos::{GizmoSnap, GizmoType, SelectedGizmo};

pub fn editor_gizmos_ui(
    mut contexts: EguiContexts,
    mut selected_option: ResMut<SelectedGizmo>,
    mut gizmo_snap: ResMut<GizmoSnap>,
) {
    egui::Window::new("Gizmos")
        .resizable(false)
        .title_bar(false)
        .default_pos(egui::pos2(20.0, 90.0))
        .show(
            contexts.ctx_mut().expect("there to alway be a contex"),
            |ui| {
                ui.vertical(|ui| {
                    ui.set_max_width(100.);
                    ui.radio_value(&mut selected_option.value, GizmoType::Pointer, "Pointer");
                    ui.separator();
                    ui.radio_value(&mut selected_option.value, GizmoType::Transform, "Move");
                    ui.radio_value(&mut selected_option.value, GizmoType::Rotate, "Rotate");

                    if matches!(selected_option.value, GizmoType::Transform) {
                        ui.add_space(10.0);
                        ui.label("Snap:");
                        ui.add(
                            egui::DragValue::new(&mut gizmo_snap.transform_value)
                                .speed(1.)
                                .range(0.0..=360.0),
                        );
                    }
                    if matches!(selected_option.value, GizmoType::Rotate) {
                        ui.add_space(10.0);
                        ui.label("Snap°:");
                        ui.add(
                            egui::DragValue::new(&mut gizmo_snap.rotate_value)
                                .speed(1.)
                                .range(0.0..=360.0),
                        );
                    }
                });
            },
        );
}
