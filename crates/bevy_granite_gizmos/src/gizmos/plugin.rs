// use super::register_embedded_rotate_gizmo_mesh;
use super::{
    // gizmo_changed_watcher,
    // gizmo_events,
    // handle_init_rotate_drag, handle_init_transform_drag,
    // handle_rotate_dragging, handle_rotate_input, handle_rotate_reset, handle_transform_dragging,
    // handle_transform_input, handle_transform_reset,
    scale_gizmo_by_camera_distance_system,
    DespawnGizmoEvent,
    GizmoSnap,
    GizmoType,
    LastSelectedGizmo,
    // PreviousTransformGizmo,
    RotateDraggingEvent,
    RotateInitDragEvent,
    RotateResetDragEvent,
    SelectedGizmo,
    SpawnGizmoEvent,
    TransformDraggingEvent,
    TransformInitDragEvent,
    TransformResetDragEvent,
};
use crate::is_gizmos_active;
use bevy::{
    app::{App, Plugin, PostUpdate, Startup, Update},
    ecs::schedule::IntoScheduleConfigs,
};

pub struct GizmoPlugin;
impl Plugin for GizmoPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // Resources
            //
            // .insert_resource(PreviousTransformGizmo::default())
            .insert_resource(LastSelectedGizmo {
                value: GizmoType::default(),
            })
            .insert_resource(SelectedGizmo {
                value: GizmoType::Pointer,
                speed_scale: 2.0,
                distance_scale: 1.0,
            })
            .insert_resource(GizmoSnap {
                transform_value: 0.,
                rotate_value: 0.,
            })
            //
            // Events
            //
            .add_event::<RotateInitDragEvent>()
            .add_event::<RotateDraggingEvent>()
            .add_event::<RotateResetDragEvent>()
            .add_event::<TransformInitDragEvent>()
            .add_event::<TransformDraggingEvent>()
            .add_event::<TransformResetDragEvent>()
            .add_event::<SpawnGizmoEvent>()
            .add_event::<DespawnGizmoEvent>()
            //
            // Schedule system
            //
            // .add_systems(Startup, register_embedded_rotate_gizmo_mesh)
            // .add_systems(
            //     Update,
            //     (gizmo_changed_watcher, gizmo_events).run_if(is_gizmos_active),
            // )
            // .add_systems(
            //     Update,
            //     (
            //         // Starting, dragging and resetting of gizmos
            //         // Transform gizmo
            //         handle_transform_input,
            //         handle_init_transform_drag.after(handle_transform_input),
            //         handle_transform_dragging.after(handle_init_transform_drag),
            //         handle_transform_reset.after(handle_transform_dragging),
            //         // Rotate gizmo
            //         handle_rotate_input,
            //         handle_init_rotate_drag.after(handle_rotate_input),
            //         handle_rotate_dragging.after(handle_init_rotate_drag),
            //         handle_rotate_reset.after(handle_rotate_dragging),
            //     )
            //         .run_if(is_gizmos_active),
            // )
            .add_systems(
                PostUpdate,
                scale_gizmo_by_camera_distance_system.run_if(is_gizmos_active),
            );
    }
}
