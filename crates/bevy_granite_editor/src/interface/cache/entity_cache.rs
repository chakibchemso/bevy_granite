use crate::interface::tabs::entity_editor::widgets::EntityRegisteredData;
use bevy::{
    pbr::{MeshMaterial3d, StandardMaterial},
    prelude::{Entity, Resource},
};
use bevy_granite_core::{IdentityData, TransformData};
use bevy_granite_gizmos::DragState;

#[derive(Resource, Default, Clone)]
pub struct EntityUIDataCache {
    pub last_entity: Option<Entity>,
    pub data: EntityData,
    pub dirty: DirtyCacheFlags,
}

#[derive(Clone, Default, PartialEq)]
pub struct EntityData {
    pub entity: Option<Entity>,
    pub material_handle: Option<MeshMaterial3d<StandardMaterial>>,
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
