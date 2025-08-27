use std::borrow::Cow;

use super::widgets::{
    EntityGlobalTransformData, EntityIdentityData, EntityRegisteredData, MaterialTab,
};
use bevy::prelude::Entity;
use bevy_granite_core::{AvailableEditableMaterials, ComponentEditor, NewEditableMaterial};

#[derive(PartialEq, Clone)]
pub struct EntityEditorTabData {
    pub user_edited_data: bool,
    pub active_entity: Option<Entity>,
    pub last_selected_entity: Option<Entity>, // Track last selected entity to detect changes
    pub identity_data: EntityIdentityData,
    pub global_transform_data: EntityGlobalTransformData,
    pub registered_data: EntityRegisteredData,
    pub component_editor: Option<ComponentEditor>,
    pub registered_type_names: Vec<Cow<'static, str>>, // Parity with the PostStartup bevy resource
    pub material_builder_open: bool,
    pub material_to_build: NewEditableMaterial,
    pub surface_collapsed_state: bool,
    pub settings_collapsed_state: bool,
    pub material_tab: MaterialTab,
    pub material_search_filter: String,
    pub component_search_filter: String,
    pub available_materials: AvailableEditableMaterials,
    pub material_delete_requested: bool,
    pub init: bool, //FIX:, proper on init not bool
}

impl Default for EntityEditorTabData {
    fn default() -> Self {
        Self {
            user_edited_data: false,
            component_editor: None,
            active_entity: None,
            last_selected_entity: None,
            identity_data: Default::default(),
            global_transform_data: Default::default(),
            registered_data: Default::default(),
            registered_type_names: Vec::new(),
            material_builder_open: false,
            material_to_build: Default::default(),
            surface_collapsed_state: true,
            settings_collapsed_state: false,
            material_tab: Default::default(),
            material_search_filter: String::new(),
            component_search_filter: String::new(),
            available_materials: Default::default(),
            material_delete_requested: false,
            init: false,
        }
    }
}
