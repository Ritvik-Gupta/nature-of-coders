pub mod shape;

use crate::view::view;
use nannou::{window, App};
use nannou_egui::Egui;

pub struct Model {
    pub _window_id: window::Id,
    pub egui: Egui,
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
        }
    }
}
