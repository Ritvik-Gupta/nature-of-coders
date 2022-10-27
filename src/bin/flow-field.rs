use itertools_num::linspace;
use nannou::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;

static mut RANDOM_SATURATION: f32 = 0.0;

fn main() {
    unsafe {
        let mut rng = rand::thread_rng();
        RANDOM_SATURATION = rng.gen_range(0.0..=1.0);
    }

    nannou::sketch(view).size(500, 500).run();
}

fn perlin_to_range(perlin_noise: f64, min: f32, max: f32) -> f32 {
    map_range(perlin_noise as f32, -1.0, 1.0, min, max)
}

fn view(app: &App, frame: Frame) {
    let perlin = Perlin::new(3);

    let draw = app.draw();
    draw.background().color(STEELBLUE);

    let (l, r, b, t) = app.window_rect().l_r_b_t();

    for x in linspace(l, r, 30) {
        for y in linspace(b, t, 30) {
            let rotation = perlin_to_range(
                perlin.get([x as f64, y as f64, app.time as f64 / 2.0]),
                0.0,
                360.0,
            );

            let draw = draw.translate(vec3(x, y, 0.0));

            draw.arrow()
                .start(pt2(0.0, 0.0))
                .end(pt2(10.0, 0.0))
                .z_degrees(rotation)
                .color(BLANCHEDALMOND)
                .finish();
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
