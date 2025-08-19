use bevy::prelude::{Gizmos, LinearRgba, Transform, Vec3};

use crate::input::GizmoAxis;

pub fn draw_axis_line(gizmos: &mut Gizmos, axis: Option<GizmoAxis>, transform: &Transform) {
    let start_pos;
    let end_pos;
    let color;
    const LINE_HALF_LENGTH: f32 = 200.;
    const RED: LinearRgba = LinearRgba::RED;
    const GREEN: LinearRgba = LinearRgba::GREEN;
    const BLUE: LinearRgba = LinearRgba::BLUE;
    const NUDGE: f32 = 0.001; // Small offset to avoid rendering issues

    let mut adjust_translation = transform.translation;
    if adjust_translation.x == 0.0 {
        adjust_translation.x = NUDGE;
    }
    if adjust_translation.y == 0.0 {
        adjust_translation.y = NUDGE;
    }
    if adjust_translation.z == 0.0 {
        adjust_translation.z = NUDGE;
    }

    match axis {
        Some(GizmoAxis::X) => {
            start_pos = Vec3::new(
                -LINE_HALF_LENGTH,
                adjust_translation.y,
                adjust_translation.z,
            );
            end_pos = Vec3::new(LINE_HALF_LENGTH, adjust_translation.y, adjust_translation.z);
            color = RED;
        }
        Some(GizmoAxis::Y) => {
            start_pos = Vec3::new(
                adjust_translation.x,
                -LINE_HALF_LENGTH,
                adjust_translation.z,
            );
            end_pos = Vec3::new(adjust_translation.x, LINE_HALF_LENGTH, adjust_translation.z);
            color = GREEN;
        }
        Some(GizmoAxis::Z) => {
            start_pos = Vec3::new(
                adjust_translation.x,
                adjust_translation.y,
                -LINE_HALF_LENGTH,
            );
            end_pos = Vec3::new(adjust_translation.x, adjust_translation.y, LINE_HALF_LENGTH);
            color = BLUE;
        }
        _ => return,
    }

    gizmos.line(start_pos, end_pos, color);
}
