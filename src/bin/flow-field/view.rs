use crate::{
    model::{particle::TrailMarker, Model},
    utils::{closest_checkpoint_position, evenly_distributed_points_in, perlin_to_field_angle},
};
use itertools::Itertools;
use lazy_static::lazy_static;
use nannou::{
    prelude::{hsla, map_range, Hsla, Rgb, Vec2, BLACK},
    App, Frame,
};
use noise::NoiseFn;

lazy_static! {
    static ref FLOW_FIELD_VECTOR_COLOR: Hsla = hsla(218.0 / 360.0, 0.8, 0.4, 0.25);
}
const BACKGROUND_COLOR: Rgb<u8> = BLACK;

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BACKGROUND_COLOR);

    let window_rect = app.window_rect();

    // Draw the Flow Field Vectors
    if model.settings.to_show_flow_field {
        evenly_distributed_points_in(&window_rect).for_each(|(x, y)| {
            let pos = Vec2::new(x, y);

            let rotation = perlin_to_field_angle(model.perlin_rng.get([
                pos.x as f64,
                pos.y as f64,
                model.view.field.time as f64,
            ]));

            let draw = draw.translate(pos.extend(0.0));

            draw.arrow()
                .start(Vec2::ZERO)
                .end(Vec2::new(10.0, 0.0))
                .rotate(rotation)
                .color(FLOW_FIELD_VECTOR_COLOR.clone())
                .finish();
        });
    }

    // Draw the Particles
    {
        let (l, r, b, t) = window_rect.l_r_b_t();
        let max_trail_alpha = model.settings.trail_length as f32;

        model.view.particles.iter().for_each(|particle| {
            if model.settings.to_show_particle_head {
                let pos = particle.position;
                let color_hue = map_range(pos.x + pos.y, l + b, r + t, 0.0, 1.0);

                draw.ellipse()
                    .xy(pos)
                    .radius(3.0)
                    .color(hsla(color_hue, 0.8, 0.30, 0.75))
                    .finish();
            }

            particle
                .trail
                .iter()
                .enumerate()
                .group_by(|(_, marker)| marker.eq(&&TrailMarker::Outside))
                .into_iter()
                .step_by(2)
                .for_each(|(_, markers)| {
                    draw.polyline()
                        .points_colored(markers.map(|(idx, marker)| {
                            let alpha_idx = (idx + 1) as f32;
                            let pos = marker.position();
                            let color_hue = map_range(pos.x + pos.y, l + b, r + t, 0.0, 1.0);

                            (pos, hsla(color_hue, 0.8, 0.30, alpha_idx / max_trail_alpha))
                        }))
                        .finish();
                });
        });
    }

    // Draw the Checkpoint arrow for nearest Flow Field Vector
    {
        let pos = closest_checkpoint_position(model.window.mouse.location);

        let rotation = perlin_to_field_angle(model.perlin_rng.get([
            pos.x as f64,
            pos.y as f64,
            model.view.field.time as f64,
        ]));

        let draw = draw.translate(pos.extend(0.0));

        draw.arrow()
            .start(Vec2::ZERO)
            .end(Vec2::new(20.0, 0.0))
            .rotate(rotation)
            .stroke_weight(3.0)
            .color(FLOW_FIELD_VECTOR_COLOR.color)
            .finish();
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
