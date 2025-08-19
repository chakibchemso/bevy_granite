use crate::GraniteType;

use super::OBJ;
use bevy_egui::egui;

impl OBJ {
    pub fn edit_via_ui(&mut self, ui: &mut egui::Ui, _spacing: (f32, f32, f32)) -> bool {
        //let large_spacing = spacing.1;
        ui.label(egui::RichText::new(self.type_name()).italics());
        //ui.add_space(large_spacing);

        // Any future OBJ data that needs editing goes here

        false
    }
}
