use crate::{
    model::{settings_config::SettingsConfig, Model},
    utils::FIELD_TIME_NORMALIZATION_FACTOR,
};
use nannou::{prelude::Update, App};
use nannou_egui::egui;
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

pub fn update(app: &App, model: &mut Model, update: Update) {
    // Update the Particle positions relative to Flow Field
    {
        if !model.settings.to_pause_field {
            model.view.field.time +=
                update.since_last.as_secs_f32() / FIELD_TIME_NORMALIZATION_FACTOR;
        }

        let window_rect = app.window_rect();
        model.view.particles.par_iter_mut().for_each(|particle| {
            particle.update(
                model.settings.trail_length,
                model.view.field.time,
                &model.perlin_rng,
                &window_rect,
            );
        });
    }

    // Update the EGUI interface
    update_egui(app, model, &update);
}
fn update_egui(app: &App, model: &mut Model, update: &Update) {
    model.egui.set_elapsed_time(update.since_start);
    let ctx = model.egui.begin_frame();

    let SettingsConfig {
        trail_length,
        to_show_flow_field,
        to_show_particle_head,
        to_pause_field,
    } = &mut model.settings;

    egui::Window::new("Workshop").show(&ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.horizontal_wrapped(|ui| {
                ui.code(format!("FPS : {:.1}", app.fps()));
            });
            ui.separator();

            ui.horizontal_wrapped(|ui| {
                ui.label("Flow Field Vectors");
                ui.radio_value(to_show_flow_field, true, "Show");
                ui.radio_value(to_show_flow_field, false, "Hide");
            });

            ui.horizontal_wrapped(|ui| {
                ui.label("Particle Head");
                ui.radio_value(to_show_particle_head, true, "Show");
                ui.radio_value(to_show_particle_head, false, "Hide");
            });

            ui.horizontal_wrapped(|ui| {
                ui.label("Field Loop");
                ui.radio_value(to_pause_field, true, "Pause");
                ui.radio_value(to_pause_field, false, "Loop");
            });

            ui.separator();
            ui.horizontal_wrapped(|ui| {
                ui.label("Trail Length");
                ui.add(egui::Slider::new(trail_length, 10..=200));
            });
        });
    });
}
