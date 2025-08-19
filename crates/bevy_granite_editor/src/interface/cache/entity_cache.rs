use bevy::{
    pbr::StandardMaterial,
    prelude::{Entity, Handle, Resource},
};
use bevy_granite_gizmos::DragState;
use bevy_granite_core::{IdentityData, TransformData};
use crate::interface::tabs::entity_editor::widgets::EntityRegisteredData;


#[derive(Resource, Default, Clone)]
pub struct EntityUIDataCache {
    pub last_entity: Option<Entity>,
    pub data: EntityData,
    pub dirty: DirtyCacheFlags,
}

#[derive(Clone, Default, PartialEq)]
pub struct EntityData {
    pub entity: Option<Entity>,
    pub material_handle: Option<Handle<StandardMaterial>>,
    pub global_transform: TransformData,
    pub identity: IdentityData,
    pub gizmo_drag: DragState,
    pub registered: EntityRegisteredData,
}

#[derive(Default, Clone)]
pub struct DirtyCacheFlags {
    pub entity_dirty: bool,
    pub gizmo_dirty: bool,
    pub material_dirty: bool,
    pub global_transform_dirty: bool,
    pub identity_dirty: bool,
    pub registered_dirty: bool,
}
