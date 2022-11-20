mod database;
mod view_config;

use crate::view::view;
use database::Database;
use nannou::{window, App};
use nannou_egui::Egui;
use view_config::ViewConfig;

pub struct Model {
    pub _window_id: window::Id,
    pub egui: Egui,
    pub db: Database,
    pub view: ViewConfig,
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

        Self {
            _window_id: window_id,
            egui,
            db: Database {
                shapes: Vec::new(),
                drawing_shape: None,
            },
            view: ViewConfig::default(),
        }
    }
}
