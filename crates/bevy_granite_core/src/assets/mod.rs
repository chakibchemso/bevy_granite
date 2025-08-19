pub mod materials;
pub mod plugin;

pub use materials::{
    get_material_from_path, load_texture_with_repeat, material_from_path_into_scene,
    materials_from_folder_into_scene, AvailableEditableMaterials, EditableMaterial,
    EditableMaterialError, EditableMaterialField, MaterialData,
    NewEditableMaterial, RequiredMaterialData, RequiredMaterialDataMut, StandardMaterialDef,
};
pub use plugin::AssetPlugin;
