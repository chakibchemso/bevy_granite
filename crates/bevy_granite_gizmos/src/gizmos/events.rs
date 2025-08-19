use bevy::prelude::{Entity, Event};
use super::GizmoType;

#[derive(Event)]
pub struct RotateInitDragEvent;

#[derive(Event)]
pub struct RotateDraggingEvent;

#[derive(Event)]
pub struct RotateResetDragEvent;

#[derive(Event)]
pub struct TransformInitDragEvent;

#[derive(Event)]
pub struct TransformDraggingEvent;

#[derive(Event)]
pub struct TransformResetDragEvent;

#[derive(Event)]
pub struct SpawnGizmoEvent(pub Entity);

#[derive(Event)]
pub struct DespawnGizmoEvent(pub GizmoType);