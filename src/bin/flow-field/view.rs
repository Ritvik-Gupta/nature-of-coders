use super::Model;
use itertools_num::linspace;
use nannou::{
    prelude::{map_range, pt2, vec3, STEELBLUE},
    App, Frame,
};
use noise::{NoiseFn, Perlin};

pub fn view(app: &App, model: &Model, frame: Frame) {
    let perlin = Perlin::new(3);

    let draw = app.draw();
    draw.background().color(STEELBLUE);

    let (l, r, b, t) = app.window_rect().l_r_b_t();

    for x in linspace(l, r, 30) {
        for y in linspace(b, t, 30) {
            let hue = perlin_to_range(
                perlin.get([x as f64, y as f64, app.time as f64 / 3.0]),
                0.0,
                1.0,
            );
            let rotation = perlin_to_range(
                perlin.get([x as f64, y as f64, app.time as f64 / 3.0]),
                0.0,
                360.0,
            );

            let draw = draw.translate(vec3(x, y, 0.0));

            draw.arrow()
                .start(pt2(0.0, 0.0))
                .end(pt2(10.0, 0.0))
                .z_degrees(rotation)
                .hsl(hue, model.chosen_saturation, 0.1)
                .finish();
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn perlin_to_range(perlin_noise: f64, min: f32, max: f32) -> f32 {
    map_range(perlin_noise as f32, -1.0, 1.0, min, max)
}
