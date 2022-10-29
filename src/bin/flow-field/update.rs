use crate::{model::Model, utils::FIELD_TIME_NORMALIZATION_FACTOR};
use nannou::{prelude::Update, App};
use nannou_egui::egui;
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

pub fn update(app: &App, model: &mut Model, update: Update) {
    // Update the Particle positions relative to Flow Field
    {
        if !model.view.field.is_paused {
            model.view.field.time +=
                update.since_last.as_secs_f32() / FIELD_TIME_NORMALIZATION_FACTOR;
        }

        let window_rect = app.window_rect();
        model.view.particles.par_iter_mut().for_each(|particle| {
            particle.update(
                model.view.trail_length,
                model.view.field.time,
                &model.perlin_rng,
                &window_rect,
            );
        });
    }

    // Update the EGUI interface
    update_egui(app, model, update);
}
fn update_egui(app: &App, model: &mut Model, update: Update) {
    model.egui.set_elapsed_time(update.since_start);
    let ctx = model.egui.begin_frame();

    let trail_length = &mut model.view.trail_length;

    egui::Window::new("Workshop").show(&ctx, |ui| {
        ui.code(format!("FPS : {:.1}", app.fps()));

        ui.add(egui::Slider::new(trail_length, 0..=100));
    });
}
