use bevy::{
    ecs::hierarchy::ChildOf,
    pbr::{MeshMaterial3d, NotShadowCaster, NotShadowReceiver},
    prelude::{
        AlphaMode, Assets, Children, Color, Commands, Component, Cone, Cylinder, Entity,
        GlobalTransform, Mesh, Meshable, Name, Quat, Query, ResMut, Resource, Sphere,
        StandardMaterial, Transform, Vec3, Visibility, Without,
    },
    render::{mesh::Mesh3d, view::RenderLayers},
};
use bevy_granite_logging::{
    config::{LogCategory, LogLevel, LogType},
    log,
};

use crate::{
    gizmos::{GizmoMesh, GizmoParent},
    input::GizmoAxis,
    selection::manager::ParentTo,
};

#[derive(Component)]
pub struct TransformGizmo;

#[derive(Resource, Default, Component)]
pub struct TransformGizmoParent;

#[derive(Resource, Default)]
pub struct PreviousTransformGizmo {
    pub entity: Option<Entity>,
}

const GIZMO_SCALE: f32 = 1.35;

const TRANSFORM_INNER_RADIUS: f32 = 0.09 * GIZMO_SCALE; // middle sphere of gizmo
const TRANSFORM_LINE_LENGTH: f32 = 0.6 * GIZMO_SCALE; // length of line
const TRANSFORM_LINE_WIDTH: f32 = 0.04 * GIZMO_SCALE; // width of line
const TRANSFORM_HANDLE_LENGTH: f32 = 0.22 * GIZMO_SCALE; // cone handle length
const TRANSFORM_HANDLE_WIDTH: f32 = 0.09 * GIZMO_SCALE; // cone handle width

pub fn spawn_transform_gizmo(
    parent: Entity,
    query: &mut Query<&GlobalTransform, Without<TransformGizmoParent>>,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let offset = Vec3::new(0., 0., 0.);

    if let Ok(parent_global_transform) = query.get(parent) {
        let gizmo_translation = offset;

        let gizmo_entity = commands
            .spawn((
                Transform {
                    translation: gizmo_translation,
                    rotation: parent_global_transform
                        .to_scale_rotation_translation()
                        .1
                        .inverse(),
                    ..Default::default()
                },
                Visibility::default(),
            ))
            .insert(RenderLayers::layer(14)) // 14 is our UI/Gizmo layer.
            .insert(Name::new("TransformGizmo"))
            .insert(TransformGizmo)
            .insert(TransformGizmoParent)
            .insert(GizmoParent)
            .id();

        commands.entity(gizmo_entity).insert(ParentTo(parent));

        build_gizmo_sphere(
            commands,
            meshes,
            materials,
            gizmo_entity,
            GizmoAxis::All,
            Color::srgba(0.8, 0.8, 0.8, 1.),
        );

        build_axis_cylinder(
            commands,
            meshes,
            materials,
            gizmo_entity,
            Vec3::X,
            Color::srgba(1., 0., 0., 1.),
        );
        build_axis_cylinder(
            commands,
            meshes,
            materials,
            gizmo_entity,
            Vec3::Y,
            Color::srgba(0., 1., 0., 1.),
        );
        build_axis_cylinder(
            commands,
            meshes,
            materials,
            gizmo_entity,
            Vec3::Z,
            Color::srgba(0., 0., 1., 1.),
        );
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Entity,
            "Transform Gizmo spawned"
        );
    } else {
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Entity,
            "Failed to spawn Transform Gizmo. Parent Entity {:?} not found or missing Transform.",
            parent
        );
    }
}

pub fn despawn_transform_gizmo(
    commands: &mut Commands,
    query: &mut Query<(Entity, &TransformGizmo, &Children)>,
) {
    for (entity, _, _) in query.iter() {
        commands.entity(entity).despawn();
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::Entity,
            "Despawned Transform Gizmo"
        );
    }
}

fn build_gizmo_sphere(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    parent: Entity,
    axis: GizmoAxis,
    color: Color,
) {
    let sphere = Sphere::new(TRANSFORM_INNER_RADIUS).mesh().ico(5).unwrap();
    let sphere_handle = meshes.add(sphere);

    let material = materials.add(StandardMaterial {
        base_color: color,
        unlit: true,
        alpha_mode: AlphaMode::AlphaToCoverage,
        ..Default::default()
    });

    commands.entity(parent).with_children(|parent| {
        parent
            .spawn((
                Mesh3d(sphere_handle),
                MeshMaterial3d(material),
                NotShadowCaster,
                NotShadowReceiver,
                Name::from("Gizmo Transform Sphere".to_string()),
                axis,
                TransformGizmo,
                GizmoMesh,
            ))
            .insert(RenderLayers::layer(14)); // 14 is our UI/Gizmo layer.
    });
}

fn build_axis_cylinder(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
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

    let arrow_mesh = meshes.add(Mesh::from(Cylinder {
        radius: TRANSFORM_LINE_WIDTH,
        half_height: TRANSFORM_LINE_LENGTH * 0.5,
    }));

    let cone_mesh = meshes.add(Mesh::from(Cone {
        radius: TRANSFORM_HANDLE_WIDTH,
        height: TRANSFORM_HANDLE_LENGTH,
    }));

    let material = materials.add(StandardMaterial {
        base_color: color,
        unlit: true,
        alpha_mode: AlphaMode::AlphaToCoverage,
        ..Default::default()
    });

    commands
        .spawn((
            Mesh3d(cone_mesh),
            MeshMaterial3d(material.clone()),
            Transform {
                translation: axis * TRANSFORM_LINE_LENGTH,
                rotation: Quat::from_rotation_arc(Vec3::Y, axis),
                ..Default::default()
            },
            NotShadowCaster,
            NotShadowReceiver,
            Name::new("Gizmo Transform Cone"),
        ))
        .insert(RenderLayers::layer(14)) // 14 is our UI/Gizmo layer.
        .insert(gizmo_axis)
        .insert(TransformGizmo)
        .insert(GizmoMesh)
        .insert(ChildOf(parent));

    commands
        .spawn((
            Mesh3d(arrow_mesh),
            MeshMaterial3d(material.clone()),
            Transform {
                translation: axis * (TRANSFORM_LINE_LENGTH * 0.5),
                rotation: Quat::from_rotation_arc(Vec3::Y, axis),
                ..Default::default()
            },
            NotShadowCaster,
            NotShadowReceiver,
            Name::new("Gizmo Transform Arrow"),
        ))
        .insert(RenderLayers::layer(14)) // 14 is our UI/Gizmo layer.
        .insert(gizmo_axis)
        .insert(TransformGizmo)
        .insert(GizmoMesh)
        .insert(ChildOf(parent));
}
