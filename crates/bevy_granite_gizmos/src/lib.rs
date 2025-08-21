use crate::camera::{add_gizmo_camera, watch_for_main_camera_addition, MainCameraAdded};
use bevy::app::{Plugin, Update};
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::picking::mesh_picking::MeshPickingPlugin;
use bevy::prelude::{App, Res, Resource};

// Modules
pub mod camera;
pub mod gizmos;
mod input;
pub mod selection;
mod ui;

pub mod utils {
    /// A marker component that an entity should be ignored by the editor
    /// This will be more powerful then not having Bridge
    /// As this is explicitly added to an entity
    #[derive(bevy::ecs::component::Component, Default)]
    // Im adding this so you cant select the editor camera
    // and to stop a crash because you can select a gizmo that then despawns its self
    pub struct EditorIgnore;
}

// Re-export
pub use camera::GizmoCamera;
pub use gizmos::{
    despawn_rotate_gizmo, despawn_transform_gizmo, GizmoMesh, GizmoParent, GizmoSnap, GizmoType,
    RotateGizmo, SelectedGizmo, TransformGizmo,
};
pub use input::{watch_gizmo_change, DragState, GizmoAxis};
pub use selection::{
    deselect_all_entities, ActiveSelection, RequestDeselectAllEntitiesEvent,
    RequestDeselectEntityEvent, RequestDuplicateAllSelectionEvent, RequestDuplicateEntityEvent,
    RequestSelectEntityEvent, RequestSelectEntityRangeEvent, Selected,
};

// Internal plugins
use gizmos::GizmoPlugin;
use input::InputPlugin;
use selection::SelectionPlugin;
use ui::UIPlugin;

/// Resource to control gizmo visibility
// When editor is toggled off, this will be set to false
// Thats why its a resource instead of an arg for plugin
// Also don't ever need to update this from parent plugin
#[derive(Resource, Clone)]
pub struct GizmoVisibilityState {
    pub active: bool,
}
impl Default for GizmoVisibilityState {
    fn default() -> Self {
        Self { active: true }
    }
}

/// This does NOT sync the gizmo camera to the main camera
/// If you are using this WITHOUT the editor sister plugin,
/// you need to handle the syncing manually
pub struct BevyGraniteGizmos;
impl Plugin for BevyGraniteGizmos {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins(MeshPickingPlugin) // Raycasting plugin
            //
            // internal
            .add_plugins(GizmoPlugin)
            .add_plugins(SelectionPlugin)
            .add_plugins(UIPlugin)
            .add_plugins(InputPlugin) // Optional
            .add_event::<MainCameraAdded>()
            //
            .add_systems(
                Update,
                watch_for_main_camera_addition.run_if(is_gizmos_active),
            )
            .add_systems(Update, add_gizmo_camera.run_if(is_gizmos_active));
    }
}

fn is_gizmos_active(gizmos_state: Res<GizmoVisibilityState>) -> bool {
    gizmos_state.active
}
