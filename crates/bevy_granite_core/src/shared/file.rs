use std::path::Path;
use bevy::asset::io::file::FileAssetReader;

pub fn rel_asset_to_absolute(rel_string: String) -> String {
    if !Path::new(&rel_string).is_absolute() {
        let abs_path = FileAssetReader::get_base_path()
            .join(format!("assets/{}", rel_string))
            .to_string_lossy()
            .to_string();
        abs_path
    } else {
        rel_string
    }
}

pub fn absolute_asset_to_rel(abs_string: String) -> String {
    let abs_path = Path::new(&abs_string);
    
    if abs_path.is_absolute() {
        let base_assets_path = FileAssetReader::get_base_path().join("assets");
        
        if let Ok(rel_path) = abs_path.strip_prefix(&base_assets_path) {
            rel_path.to_string_lossy().to_string().replace("\\", "/")
        } else {
            abs_string
        }
    } else {
        abs_string
    }
}