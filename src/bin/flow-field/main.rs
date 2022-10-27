mod view;

use nannou::{window, winit::event::DeviceEvent, App, Event};
use rand::Rng;
use view::view;

fn main() {
    nannou::app(Model::for_app)
        .event(event)
        .size(500, 500)
        .run();
}

pub struct Model {
    _window: window::Id,
    chosen_saturation: f32,
}

impl Model {
    pub fn for_app(app: &App) -> Self {
        let _window = app.new_window().view(view).build().unwrap();
        let mut rng = rand::thread_rng();
        let chosen_saturation = rng.gen_range(0.0..=1.0);

        Self {
            _window,
            chosen_saturation,
        }
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
    match _event {
        Event::DeviceEvent(_, device_event) => match device_event {
            DeviceEvent::Key(key_input) => {
                println!("{:?}", key_input.virtual_keycode.unwrap());
            }
            DeviceEvent::Button { .. } => println!("button event"),
            _ => {}
        },
        _ => {}
    }
}
