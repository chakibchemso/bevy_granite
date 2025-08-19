use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct RequestSelectEntityEvent {
    pub entity: Entity,
    pub additive: bool,
}

#[derive(Event)]
pub struct RequestSelectEntityRangeEvent {
    pub entities: Vec<Entity>,
    pub additive: bool,
}

#[derive(Event)]
pub struct RequestDeselectEntityEvent(pub Entity);

#[derive(Event)]
pub struct RequestDeselectAllEntitiesEvent;

#[derive(Event)]
pub struct RequestDuplicateEntityEvent {
    pub entity: Entity,
}

#[derive(Event)]
pub struct RequestDuplicateAllSelectionEvent;
