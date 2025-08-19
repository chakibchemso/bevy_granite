use bevy::prelude::Reflect;
use serde::{Deserialize, Serialize};
pub mod definition;
pub mod load;

pub use definition::*;
pub use load::*;

// Store the material path, the current material, and the last material
#[derive(Serialize, Deserialize, Reflect, Debug, Clone, PartialEq)]
pub struct MaterialData {
    pub path: String,

    #[serde(skip)]
    #[reflect(skip_serializing)]
    pub current: EditableMaterial,

    #[serde(skip)]
    #[reflect(skip_serializing)]
    pub last: EditableMaterial,
}

impl MaterialData {
    pub fn new(path: String) -> Self {
        Self {
            path,
            current: EditableMaterial::get_new_unnamed_base_color(),
            last: EditableMaterial::get_new_unnamed_base_color(),
        }
    }

    pub fn as_ref(&self) -> RequiredMaterialData {
        RequiredMaterialData {
            current: &self.current,
            last: &self.last,
            path: &self.path,
        }
    }

    pub fn as_mut(&mut self) -> RequiredMaterialDataMut {
        RequiredMaterialDataMut {
            current: &mut self.current,
            last: &mut self.last,
            path: &mut self.path,
        }
    }
}
