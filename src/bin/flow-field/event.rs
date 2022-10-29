use crate::model::{particle::Particle, Model};
use nannou::{
    event::ElementState,
    prelude::{Key, MouseButton, WindowEvent},
    winit::event::DeviceEvent,
    App, Event,
};

pub fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::DeviceEvent(_, device_event) => match device_event {
            DeviceEvent::Key(keyboard_input) => {
                match (keyboard_input.state, keyboard_input.virtual_keycode) {
                    (ElementState::Pressed, Some(Key::P)) => {
                        model.view.field.is_paused = !model.view.field.is_paused
                    }
                    _ => {}
                }
            }
            _ => {}
        },
        Event::WindowEvent {
            simple: window_event,
            ..
        } => match window_event {
            Some(WindowEvent::MouseMoved(mouse_location)) => {
                model.window.mouse.location = mouse_location;
                if model.window.mouse.is_pressed {
                    model
                        .view
                        .particles
                        .push(Particle::new(model.window.mouse.location));
                }
            }
            Some(WindowEvent::MousePressed(MouseButton::Left)) => {
                model.window.mouse.is_pressed = true
            }
            Some(WindowEvent::MouseReleased(MouseButton::Left)) => {
                model.window.mouse.is_pressed = false
            }
            _ => {}
        },
        _ => {}
    }
}
