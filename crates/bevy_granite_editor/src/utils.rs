use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::io::{self, Result};
use std::path::Path;

pub fn load_from_toml_file<T: DeserializeOwned>(path: &str) -> Result<T> {
    let content = fs::read_to_string(path)?;
    let data: T =
        toml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(data)
}

pub fn save_to_toml_file<T: Serialize>(value: &T, path: &str) -> Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    let toml_str =
        toml::to_string_pretty(value).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    fs::write(path, toml_str)
}
