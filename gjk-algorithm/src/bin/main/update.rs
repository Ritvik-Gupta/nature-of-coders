use crate::{model::Model, utils::to_vertex_label};
use itertools::Itertools;
use nannou::{prelude::Update, App};
use nannou_egui::egui;

pub fn update(app: &App, model: &mut Model, update: Update) {
    update_egui(app, model, &update);
}

fn update_egui(app: &App, model: &mut Model, update: &Update) {
    model.egui.set_elapsed_time(update.since_start);
    let ctx = model.egui.begin_frame();

    let shapes = &model.db.shapes;
    let drawing_shape = &model.db.drawing_shape;

    egui::Window::new("Workshop")
        .title_bar(false)
        .show(&ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.code(format!("FPS : {:.1}", app.fps()));
            });
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.style_mut().wrap = Some(true);

                if let Some(shape) = drawing_shape {
                    ui.colored_label(
                        egui::Color32::LIGHT_BLUE,
                        shape
                            .vertices
                            .iter()
                            .cloned()
                            .map(to_vertex_label)
                            .join(" "),
                    );
                    ui.separator();
                }

                shapes.iter().rev().for_each(|shape| {
                    ui.colored_label(
                        egui::Color32::LIGHT_GRAY,
                        shape
                            .vertices
                            .iter()
                            .cloned()
                            .map(to_vertex_label)
                            .join(", "),
                    );
                    ui.separator();
                });
            });
        });
}
