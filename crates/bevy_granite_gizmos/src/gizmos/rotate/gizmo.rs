use bevy::asset::Handle;
use bevy::ecs::hierarchy::ChildOf;
use bevy::pbr::MeshMaterial3d;
use bevy::picking::Pickable;
use bevy::prelude::{AlphaMode, Meshable, Quat, Sphere};
use bevy::prelude::{
    Assets, Children, Color, Commands, Component, Entity, GlobalTransform, Mesh, Name, Query,
    ResMut, Resource, StandardMaterial, Transform, Vec3, Visibility, Without,
};
use bevy::render::mesh::Mesh3d;
use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    render::view::RenderLayers,
};
use bevy_granite_logging::{
    config::{LogCategory, LogLevel, LogType},
    log,
};

use crate::gizmos::GizmoOf;
use crate::{
    gizmos::{GizmoMesh, GizmoParent},
    input::GizmoAxis,
    selection::manager::ParentTo,
};

#[derive(Component)]
pub struct RotateGizmo;

#[derive(Resource, Default, Component)]
pub struct RotateGizmoParent;

#[derive(Resource, Default)]
pub struct PreviousTransformGizmo {
    pub entity: Option<Entity>,
}

const GIZMO_SCALE: f32 = 0.9;
const ROTATE_INNER_RADIUS: f32 = 0.12 * GIZMO_SCALE; // middle sphere of gizmo (free rotate)
const ROTATE_VISUAL_RADIUS: f32 = 0.64 * GIZMO_SCALE; // middle sphere of gizmo (visual)
const RING_MESH_HASH: u128 = 12345678901234567890; // doesnt matter the value

pub fn register_embedded_rotate_gizmo_mesh(mut meshes: ResMut<Assets<Mesh>>) {
    let handle = get_mesh_handle();
    let ring_obj = include_str!("./Ring.obj");
    let ring_mesh =
        bevy_obj::mesh::load_obj_as_mesh(ring_obj.as_bytes(), &bevy_obj::ObjSettings::default())
            .expect("Obj to load");
    meshes.insert(handle.id(), ring_mesh);
}

pub fn get_mesh_handle() -> Handle<Mesh> {
    Handle::<Mesh>::weak_from_u128(RING_MESH_HASH)
}
pub fn spawn_rotate_gizmo(
    parent: Entity,
    query: &mut Query<&GlobalTransform, Without<RotateGizmoParent>>,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let offset = Vec3::new(0., 0., 0.);

    if let Ok(parent_global_transform) = query.get(parent) {
        let gizmo_translation = offset;

        let sphere = Sphere::new(ROTATE_VISUAL_RADIUS).mesh().ico(7).unwrap();
        let sphere_handle = meshes.add(sphere);
        let material = materials.add(StandardMaterial {
            base_color: Color::srgba(0.6, 0.6, 0.6, 0.24),
            unlit: true,
            alpha_mode: AlphaMode::AlphaToCoverage,
            ..Default::default()
        });

        let gizmo_entity = commands
            .spawn((
                Mesh3d(sphere_handle),
                MeshMaterial3d(material.clone()),
                NotShadowCaster,
                NotShadowReceiver,
                Pickable {
                    is_hoverable: true,
                    should_block_lower: false,
                },
                Transform {
                    translation: gizmo_translation,
                    rotation: parent_global_transform
                        .to_scale_rotation_translation()
                        .1
                        .inverse(),
                    ..Default::default()
                },
                Visibility::default(),
                GizmoOf(parent),
                ChildOf(parent),
            ))
            .insert(RenderLayers::layer(14)) // 14 is our UI/Gizmo layer.
            .insert(Name::new("RotateGizmo"))
            .insert(RotateGizmo)
            .insert(RotateGizmoParent)
            .insert(GizmoParent)
            .id();

        // commands.entity(gizmo_entity).insert(ParentTo(parent));

        build_free_sphere(
            parent,
            commands,
            materials,
            gizmo_entity,
            GizmoAxis::All,
            Color::srgba(1., 1., 0.0, 1.),
            meshes,
        );

        build_axis_ring(
            parent,
            commands,
            materials,
            gizmo_entity,
            Vec3::X,
            Color::srgba(1., 0., 0., 1.0),
        );

        build_axis_ring(
            parent,
            commands,
            materials,
            gizmo_entity,
            Vec3::Y,
            Color::srgba(0., 1., 0., 1.),
        );

        build_axis_ring(
            parent,
            commands,
            materials,
            gizmo_entity,
            Vec3::Z,
            Color::srgba(0., 0., 1., 1.),
        );

        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Entity,
            "Rotate Gizmo spawned"
        );
    } else {
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Entity,
            "Failed to spawn rotate Gizmo. Parent Entity {:?} not found or missing Transform.",
            parent
        );
    }
}

pub fn despawn_rotate_gizmo(
    commands: &mut Commands,
    query: &mut Query<(Entity, &RotateGizmo, &Children)>,
) {
    for (entity, _, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Entity,
            "Despawned Rotate Gizmo"
        );
    }
}

fn build_free_sphere(
    target: Entity,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    parent: Entity,
    axis: GizmoAxis,
    color: Color,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let sphere = Sphere::new(ROTATE_INNER_RADIUS).mesh().ico(5).unwrap();
    let sphere_handle = meshes.add(sphere);
    let material = materials.add(StandardMaterial {
        base_color: color,
        unlit: true,
        alpha_mode: AlphaMode::AlphaToCoverage,
        ..Default::default()
    });

    commands
        .spawn((
            Mesh3d(sphere_handle),
            MeshMaterial3d(material.clone()),
            Transform::default(),
            NotShadowCaster,
            NotShadowReceiver,
            Name::new("Gizmo Rotate Sphere"),
            RenderLayers::layer(14), // 14 is our UI/Gizmo layer.
            axis,
            RotateGizmo,
            GizmoMesh,
            ChildOf(parent),
            GizmoOf(target),
        ))
        .observe(super::drag::debug_handle_rotate_dragging::<'a'>);
}

fn build_axis_ring(
    target: Entity,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    parent: Entity,
    axis: Vec3,
    color: Color,
) {
    let gizmo_axis = match axis {
        Vec3::X => GizmoAxis::X,
        Vec3::Y => GizmoAxis::Y,
        Vec3::Z => GizmoAxis::Z,
        _ => GizmoAxis::None,
    };

    // Load the embedded ring mesh
    let ring_mesh = get_mesh_handle();

    let material = materials.add(StandardMaterial {
        base_color: color,
        unlit: true,
        alpha_mode: AlphaMode::AlphaToCoverage,
        ..Default::default()
    });

    let rotation = match axis {
        Vec3::X => Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
        Vec3::Y => Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
        Vec3::Z => Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
        _ => Quat::IDENTITY,
    };

    commands.spawn((
        Mesh3d(ring_mesh),
        MeshMaterial3d(material.clone()),
        Transform {
            scale: Vec3::ONE * GIZMO_SCALE,
            rotation,
            ..Default::default()
        },
        NotShadowCaster,
        NotShadowReceiver,
        Name::new("Gizmo Rotate Ring"),
        RenderLayers::layer(14), // 14 is our UI/Gizmo layer.
        gizmo_axis,
        RotateGizmo,
        GizmoMesh,
        GizmoOf(target),
        ChildOf(parent),
    ));
}
