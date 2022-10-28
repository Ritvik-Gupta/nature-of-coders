pub mod particle;
mod view_config;
mod window_config;

use crate::utils::{evenly_distributed_points_in, FIELD_TIME_NORMALIZATION_FACTOR};
use crate::view::view;
use nannou::{prelude::Vec2, window, App};
use noise::Perlin;
use particle::Particle;
use rand::Rng;
use view_config::{FieldConfig, ViewConfig};
use window_config::{MouseConfig, WindowConfig};

pub struct Model {
    pub _window_id: window::Id,
    pub perlin_rng: Perlin,
    pub window: WindowConfig,
    pub view: ViewConfig,
}

impl Model {
    pub fn for_app(app: &App) -> Self {
        let _window_id = app.new_window().view(view).build().unwrap();

        let particles = evenly_distributed_points_in(&app.window_rect())
            .map(Vec2::from)
            .map(Particle::new)
            .collect();

        // let particles = Vec::new();

        Self {
            _window_id,
            perlin_rng: Perlin::new(rand::thread_rng().gen()),
            window: WindowConfig {
                mouse: MouseConfig {
                    location: Vec2::ZERO,
                    is_pressed: false,
                },
            },
            view: ViewConfig {
                field: FieldConfig {
                    time: app.time / FIELD_TIME_NORMALIZATION_FACTOR,
                    is_paused: false,
                },
                particles,
            },
        }
    }
}
