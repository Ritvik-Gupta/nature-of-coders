use crate::utils::{closest_checkpoint_position, compare_range_sign, perlin_to_range};
use nannou::prelude::{Rect, Vec2, Vec2Rotate};
use noise::{NoiseFn, Perlin};
use std::{collections::VecDeque, f32::consts::PI};

pub struct Particle {
    pub position: Vec2,
    pub trail: VecDeque<Vec2>,
    velocity: Vec2,
    acceleration: Vec2,
}

impl Particle {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            trail: VecDeque::with_capacity(1),
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
        }
    }

    fn update_position_in(&mut self, trail_length: usize, window_rect: &Rect) {
        self.trail.push_back(self.position);
        while self.trail.len() > trail_length {
            self.trail.pop_front();
        }

        let pos = &mut self.position;

        *pos += self.velocity;
        let (l, r, b, t) = window_rect.l_r_b_t();

        if pos.x < l || pos.x > r || pos.y < b || pos.y > t {
            self.trail.clear();
        }

        pos.x += (l - r) * compare_range_sign(pos.x, l, r);
        pos.y += (b - t) * compare_range_sign(pos.y, b, t);
    }

    fn update_velocity(&mut self) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length_max(7.5);
    }

    pub fn update(
        &mut self,
        trail_length: usize,
        field_time: f32,
        perlin_rng: &mut Perlin,
        window_rect: &Rect,
    ) {
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
        self.update_position_in(trail_length, window_rect);
    }
}
