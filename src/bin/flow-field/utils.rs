use itertools_num::linspace;
use nannou::prelude::{map_range, Rect, Vec2};

pub const FIELD_TIME_NORMALIZATION_FACTOR: f32 = 4.0;
pub const VECTOR_SEPARATION: f32 = 30.0;
pub const MAX_PARTICLE_VELOCITY: f32 = 6.0;

pub fn compare_range_sign(value: f32, lower: f32, upper: f32) -> f32 {
    if value < lower {
        -1.0
    } else if value > upper {
        1.0
    } else {
        0.0
    }
}

pub fn perlin_to_range(perlin_noise: f64, min: f32, max: f32) -> f32 {
    map_range(perlin_noise as f32, -1.0, 1.0, min, max)
}

pub fn evenly_distributed_points_in(window_rect: &Rect) -> impl Iterator<Item = (f32, f32)> {
    let (l, r, b, t) = window_rect.l_r_b_t();
    let (width, height) = window_rect.w_h();

    linspace(l, r, (width / VECTOR_SEPARATION) as usize + 1).flat_map(move |x| {
        linspace(b, t, (height / VECTOR_SEPARATION) as usize + 1).map(move |y| (x, y))
    })
}

pub fn closest_checkpoint_position(position: Vec2) -> Vec2 {
    (position / VECTOR_SEPARATION).round() * VECTOR_SEPARATION
}
