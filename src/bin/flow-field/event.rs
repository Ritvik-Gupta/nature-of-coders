use crate::{
    model::{particle::Particle, Model},
    utils::FIELD_TIME_NORMALIZATION_FACTOR,
};
use nannou::{
    event::ElementState,
    prelude::{Key, MouseButton, WindowEvent},
    winit::event::DeviceEvent,
    App, Event,
};

pub fn event(app: &App, model: &mut Model, event: Event) {
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
        Event::Update(update_event) => {
            if !model.view.field.is_paused {
                model.view.field.time +=
                    update_event.since_last.as_secs_f32() / FIELD_TIME_NORMALIZATION_FACTOR;
            }

            model.view.particles.iter_mut().for_each(|particle| {
                particle.update(
                    &mut model.perlin_rng,
                    model.view.field.time,
                    &app.window_rect(),
                );
            });
        }
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
