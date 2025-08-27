use super::AvailableEditableMaterials;
use crate::EditableMaterial;
use bevy::{
    app::{App, Plugin, PreStartup},
    asset::{AssetServer, Assets, Handle},
    ecs::system::{Res, ResMut},
    pbr::StandardMaterial,
};

fn preload_fallback_material(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut available_materials: ResMut<AvailableEditableMaterials>,
    asset_server: Res<AssetServer>,
) {
    // Special None material
    let mut none_material = EditableMaterial::default();
    none_material.set_to_empty();
    none_material.update_name("None".to_string());

    // Fallback editable default white
    let mut white_editable = EditableMaterial::get_new_unnamed_base_color();
    let white_handle: Handle<StandardMaterial> = materials.add(StandardMaterial::default());
    white_editable.set_handle(Some(white_handle));
    white_editable.update_name("default".to_string());
    white_editable.update_path("materials/internal/default.mat".to_string());
    white_editable.material_exists_and_load(
        &mut available_materials,
        &mut materials,
        &asset_server,
        "",
        "",
    );

    if let Some(ref mut materials) = available_materials.materials {
        materials.insert(0, none_material);
        materials.insert(1, white_editable);
    } else {
        available_materials.materials = Some(vec![none_material, white_editable]);
    }
}

pub struct AssetPlugin;
impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // Resources
            //
            .insert_resource(AvailableEditableMaterials::default())
            //
            // Schedule system
            //
            .add_systems(PreStartup, preload_fallback_material);
    }
}
