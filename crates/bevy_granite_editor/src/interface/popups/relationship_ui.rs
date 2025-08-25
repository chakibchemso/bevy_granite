use crate::{
    interface::{
        events::{RequestNewParent, RequestRemoveChildren, RequestRemoveParents},
        shared::widgets::make_frame_solid_via_context,
        EditorEvents,
    },
    UI_CONFIG,
};
use bevy::prelude::Vec2;
use bevy_egui::{
    egui::{self, Window},
    EguiContexts,
};

pub fn relationship_ui(
    contexts: &mut EguiContexts,
    position: Vec2,
    mut events: EditorEvents,
) -> bool {
    let spacing = UI_CONFIG.spacing;
    let mut should_close = false;
    let _response = Window::new("Relationships")
        .resizable(false)
        .title_bar(false)
        .fixed_pos([position.x, position.y])
        // call this to ensure the window is not transparent when theme transparency is selected
        .frame(make_frame_solid_via_context(
            egui::Frame::window(&contexts.ctx_mut().expect("Egui context to exist").style()),
            contexts.ctx_mut().expect("Egui context to exist"),
        ))
        .show(contexts.ctx_mut().expect("Egui context to exist"), |ui| {
            ui.vertical(|ui| {
                ui.set_max_width(250.);
                ui.label("Relationship:");
                ui.add_space(spacing);

                if ui.button("Set as Parent").clicked() {
                    events.parent.write(RequestNewParent);
                    should_close = true;
                }

                if ui.button("Remove Parent").clicked() {
                    events.remove_parent.write(RequestRemoveParents);
                    should_close = true;
                }
                if ui.button("Remove Children").clicked() {
                    events.remove_children.write(RequestRemoveChildren);
                    should_close = true;
                }

                ui.add_space(spacing);
            });
        });

    let ctx = contexts.ctx_mut().expect("Egui context to exist");
    if ctx.input(|i| i.pointer.any_click()) && !ctx.is_pointer_over_area() {
        should_close = true;
    }
    should_close
}
