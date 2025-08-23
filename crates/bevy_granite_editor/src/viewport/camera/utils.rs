use crate::{editor_state::INPUT_CONFIG, viewport::camera::CameraTarget};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::{
    core_pipeline::core_3d::Camera3d,
    input::mouse::{MouseMotion, MouseWheel},
    prelude::{
        Camera, Commands, EulerRot, EventReader, Local, Name, Quat, Query, Res, ResMut, Time,
        Transform, Vec2, Vec3, With,
    },
    render::view::RenderLayers,
};
use bevy_granite_core::{TreeHiddenEntity, UICamera, UserInput};
use bevy_granite_gizmos::utils::EditorIgnore;

pub fn add_ui_camera(mut commands: Commands) {
    let context = bevy_egui::EguiContext::default();

    let _ui_camera = commands
        .spawn((
            Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            Camera3d::default(),
            Name::new("UI Camera"),
            Tonemapping::None, // need this so bevy rendering doesnt break without tonemapping_luts
            bevy::picking::Pickable {
                // this is so it does not block any other objects
                should_block_lower: false,
                is_hoverable: false,
            },
            EditorIgnore, // This camera should not be selectable in the editor
        ))
        .insert(Camera {
            order: 2,
            ..Default::default()
        })
        .insert(UICamera)
        .insert((bevy_egui::PrimaryEguiContext, context))
        .insert(TreeHiddenEntity)
        .insert(bevy_granite_gizmos::GizmoCamera)
        .insert(RenderLayers::layer(14)) // 14 is our UI/Gizmo layer.
        .id();
}

pub fn rotate_camera_towards(
    camera_transform: &mut Transform,
    target_position: Vec3,
    smooth_factor: f32,
) {
    let desired_rotation = Transform::from_translation(camera_transform.translation)
        .looking_at(target_position, Vec3::Y)
        .rotation;
    camera_transform.rotation = camera_transform
        .rotation
        .slerp(desired_rotation, smooth_factor);
}

// FPS style camera movement
pub fn handle_movement(
    query: &mut Query<&mut Transform, With<UICamera>>,
    user_input: &Res<UserInput>,
    mouse_motion_events: &mut EventReader<MouseMotion>,
    mouse_wheel_events: &mut EventReader<MouseWheel>,
    _target_pos: &mut ResMut<CameraTarget>,
    time: Res<Time>,
    mut movement_speed: Local<f32>,
) {
    let delta_time = time.delta_secs();
    let base_movement_speed = INPUT_CONFIG.fps_camera_speed;
    let base_rotation_sensitivity = INPUT_CONFIG.fps_camera_sensitivity / 100.; // divide by is to somewhat normalize these values relative to each other
    if *movement_speed == 0.0 {
        *movement_speed = base_movement_speed;
    }

    for event in mouse_wheel_events.read() {
        *movement_speed *= 1.1_f32.powf(event.y);
        *movement_speed = movement_speed.clamp(base_movement_speed, 2000.0);
    }

    let rotation_sensitivity = base_rotation_sensitivity;
    for mut transform in query.iter_mut() {
        let mut accumulated_delta = Vec2::ZERO;
        for event in mouse_motion_events.read() {
            accumulated_delta += event.delta;
        }

        if accumulated_delta.length_squared() > 0.0 {
            let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

            yaw -= accumulated_delta.x * rotation_sensitivity;
            pitch -= accumulated_delta.y * rotation_sensitivity;
            let pitch_limit = std::f32::consts::FRAC_PI_2 - 0.1;
            pitch = pitch.clamp(-pitch_limit, pitch_limit);

            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
        }

        let mut direction = Vec3::ZERO;

        if user_input.key_w.pressed {
            direction += *transform.forward();
        }
        if user_input.key_s.pressed {
            direction -= *transform.forward();
        }
        if user_input.key_a.pressed {
            direction -= *transform.right();
        }
        if user_input.key_d.pressed {
            direction += *transform.right();
        }
        if user_input.key_space.pressed {
            direction += Vec3::Y;
        }
        if user_input.shift_left.pressed {
            direction -= Vec3::Y;
        }

        if direction.length_squared() > 0.0 {
            let movement = direction.normalize() * *movement_speed * delta_time;
            transform.translation += movement;
        }
    }
}

pub fn handle_zoom(
    query: &mut Query<&mut Transform, With<UICamera>>,
    mouse_wheel_events: &mut EventReader<MouseWheel>,
    target_pos: &mut ResMut<CameraTarget>,
) {
    let zoom_speed = INPUT_CONFIG.zoom_camera_sensitivity;

    for event in mouse_wheel_events.read() {
        for mut transform in query.iter_mut() {
            let direction = (target_pos.position - transform.translation).normalize();
            let zoom_amount = event.y * zoom_speed;

            transform.translation += direction * zoom_amount;
        }
    }
}
