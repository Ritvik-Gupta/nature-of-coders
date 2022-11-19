use crate::model::Model;
use gjk_algorithm::Shape;
use nannou::{
    event::{ElementState, KeyboardInput},
    prelude::{Key, MouseButton, WindowEvent},
    winit::event::DeviceEvent,
    App, Event,
};

pub fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::DeviceEvent(
            _,
            DeviceEvent::Key(KeyboardInput {
                state: key_state,
                virtual_keycode: Some(key),
                ..
            }),
        ) => {
            if key == Key::S {
                match (model.settings.drawing_shape.is_some(), key_state) {
                    (true, ElementState::Released) => {
                        model.view.highlight_point = false;
                        model
                            .shapes
                            .push(model.settings.drawing_shape.take().unwrap());
                    }
                    (false, ElementState::Pressed) => {
                        model.settings.drawing_shape = Some(Shape::default())
                    }
                    _ => {}
                }
            }
        }
        Event::WindowEvent {
            simple: window_event,
            ..
        } => match (&mut model.settings.drawing_shape, window_event) {
            (Some(_), Some(WindowEvent::MousePressed(MouseButton::Left))) => {
                model.view.highlight_point = true
            }
            (Some(shape), Some(WindowEvent::MouseReleased(MouseButton::Left))) => {
                shape.vertices.push(app.mouse.position());
                model.view.highlight_point = false;
            }
            _ => {}
        },
        _ => {}
    }
}
