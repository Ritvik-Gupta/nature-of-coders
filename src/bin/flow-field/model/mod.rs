pub mod particle;
pub mod settings_config;
pub mod view_config;
mod window_config;

use crate::utils::{evenly_distributed_points_in, FIELD_TIME_NORMALIZATION_FACTOR};
use crate::view::view;
use nannou::{prelude::Vec2, window, App};
use nannou_egui::Egui;
use noise::Perlin;
use particle::Particle;
use rand::Rng;
use view_config::{FieldConfig, ViewConfig};
use window_config::{MouseConfig, WindowConfig};

use self::settings_config::SettingsConfig;

pub struct Model {
    pub _window_id: window::Id,
    pub egui: Egui,
    pub perlin_rng: Perlin,
    pub window: WindowConfig,
    pub view: ViewConfig,
    pub settings: SettingsConfig,
}

impl Model {
    pub fn for_app(app: &App) -> Self {
        let window_id = app
            .new_window()
            .view(view)
            .raw_event(|_app, model: &mut Self, event| model.egui.handle_raw_event(event))
            .build()
            .unwrap();

        let window = app.window(window_id).unwrap();
        let egui = Egui::from_window(&window);

        let particles = evenly_distributed_points_in(&app.window_rect())
            .map(Vec2::from)
            .map(Particle::new)
            .collect();

        Self {
            _window_id: window_id,
            egui,
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
            settings: SettingsConfig::default(),
        }
    }
}
