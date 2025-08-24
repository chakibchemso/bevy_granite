use bevy::{
    ecs::{component::Component, resource::Resource},
    prelude::{Deref, DerefMut},
    render::view::RenderLayers,
};

pub mod distance_scaling;
pub mod events;
pub mod manager;
pub mod plugin;
pub mod rotate;
pub mod transform;

#[derive(Clone, Default, Debug, Copy, PartialEq)]
pub enum GizmoType {
    Transform,
    Rotate,
    #[default]
    Pointer,
    None,
}

#[derive(Resource, Deref, DerefMut)]
pub struct SelectedGizmo(GizmoConfig);

#[derive(Component)]
pub struct GizmoConfig {
    pub value: GizmoType,
    pub speed_scale: f32,
    pub distance_scale: f32,
}

#[derive(Resource)]
pub struct LastSelectedGizmo {
    pub value: GizmoType,
}

#[derive(Component)]
pub struct GizmoMesh;

#[derive(Component)]
pub struct GizmoParent;

#[derive(Resource)]
pub struct GizmoSnap {
    pub rotate_value: f32,
    pub transform_value: f32,
}

#[derive(Component, Deref)]
#[relationship(relationship_target = Gizmos)]
#[component(on_add = Self::on_add)]
#[require(bevy_granite_core::EditorIgnore, RenderLayers = RenderLayers::layer(14))]
pub struct GizmoOf(pub bevy::ecs::entity::Entity);

impl GizmoOf {
    fn on_add(mut world: bevy::ecs::world::DeferredWorld, ctx: bevy::ecs::component::HookContext) {
        let mut ignore = world
            .get_mut::<EditorIgnore>(ctx.entity)
            .expect("EditorIgnore is required componet");
        ignore.insert(EditorIgnore::GIZMO | EditorIgnore::PICKING);
    }
}

#[derive(Component)]
#[relationship_target(relationship = GizmoOf)]
pub struct Gizmos(Vec<bevy::ecs::entity::Entity>);

use bevy_granite_core::EditorIgnore;
pub use distance_scaling::scale_gizmo_by_camera_distance_system;
pub use events::{
    DespawnGizmoEvent, RotateDraggingEvent, RotateInitDragEvent, RotateResetDragEvent,
    SpawnGizmoEvent, TransformDraggingEvent, TransformInitDragEvent, TransformResetDragEvent,
};
pub use manager::{gizmo_changed_watcher, gizmo_events};
pub use plugin::GizmoPlugin;
pub use rotate::{
    despawn_rotate_gizmo, handle_init_rotate_drag, handle_rotate_dragging, handle_rotate_input,
    handle_rotate_reset, register_embedded_rotate_gizmo_mesh, spawn_rotate_gizmo, RotateGizmo,
    RotateGizmoParent,
};
pub use transform::{
    draw_axis_line, handle_init_transform_drag, handle_transform_dragging, handle_transform_input,
    handle_transform_reset, spawn_transform_gizmo, PreviousTransformGizmo, TransformGizmo,
    TransformGizmoParent,
};
