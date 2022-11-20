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
            if key == Key::LShift {
                match (&model.db.drawing_shape, key_state) {
                    (Some(shape), ElementState::Released) => {
                        model.view.highlight_point = false;
                        if shape.vertices.len() > 2 {
                            model.db.shapes.push(shape.clone());
                        }
                        model.db.drawing_shape = None;
                    }
                    (None, ElementState::Pressed) => {
                        model.db.drawing_shape = Some(Shape::default())
                    }
                    _ => {}
                }
            }
        }
        Event::WindowEvent {
            simple: window_event,
            ..
        } => match (&mut model.db.drawing_shape, window_event) {
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
