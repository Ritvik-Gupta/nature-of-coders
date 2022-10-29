use crate::{
    model::{particle::Particle, Model},
    utils::FIELD_TIME_NORMALIZATION_FACTOR,
};
use nannou::{
    event::ElementState,
    prelude::{Key, MouseButton, Update, WindowEvent},
    winit::event::DeviceEvent,
    App, Event,
};
use nannou_egui::egui;

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

pub fn update(app: &App, model: &mut Model, update: Update) {
    // Update the Particle positions relative to Flow Field
    {
        if !model.view.field.is_paused {
            model.view.field.time +=
                update.since_last.as_secs_f32() / FIELD_TIME_NORMALIZATION_FACTOR;
        }

        model.view.particles.iter_mut().for_each(|particle| {
            particle.update(
                &mut model.perlin_rng,
                model.view.field.time,
                &app.window_rect(),
            );
        });
    }

    // Update the EGUI interface
    {
        model.egui.set_elapsed_time(update.since_start);
        let ctx = model.egui.begin_frame();
        egui::Window::new("Workshop window").show(&ctx, |ui| {
            ui.heading("Hello World");
        });
    }
}
