use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub enum EntityEvent {
    Select { target: Entity, additive: bool },
    SelectRange { range: Vec<Entity>, additive: bool },
    Deselect { target: Entity },
    DeselectRange { range: Vec<Entity> },
    DeselectAll,
}

#[derive(Event)]
pub struct RequestDuplicateEntityEvent {
    pub entity: Entity,
}

#[derive(Event)]
pub struct RequestDuplicateAllSelectionEvent;
