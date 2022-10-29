use crate::utils::{closest_checkpoint_position, perlin_to_range};
use nannou::prelude::{Rect, Vec2, Vec2Rotate};
use noise::{NoiseFn, Perlin};
use std::f32::consts::PI;

pub struct Particle {
    pub position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
}

impl Particle {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
        }
    }

    fn update_position_in(&mut self, window_rect: &Rect) {
        self.position += self.velocity;

        let (l, r, b, t) = window_rect.l_r_b_t();

        self.position.x = (self.position.x + r - l) % (r - l) + l;
        self.position.y = (self.position.y + t - b) % (t - b) + b;
    }

    fn update_velocity(&mut self) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length_max(7.5);
    }

    pub fn update(&mut self, perlin_rng: &mut Perlin, field_time: f32, window_rect: &Rect) {
        let closest_field_vector = closest_checkpoint_position(self.position);

        let field_acl_direction = perlin_to_range(
            perlin_rng.get([
                closest_field_vector.x as f64,
                closest_field_vector.y as f64,
                field_time as f64,
            ]),
            -PI,
            PI,
        );
        self.acceleration = Vec2::new(2.5, 0.0).rotate(field_acl_direction);

        self.update_velocity();
        self.update_position_in(window_rect);
    }
}
