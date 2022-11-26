use crate::{model::Model, utils::to_vertex_label};
use nannou::{
    prelude::{gray, Rect, Vec2, WHITE},
    App, Draw, Frame,
};

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.main_window();
    let win_rect = window.rect();
    draw.background().rgb(0.11, 0.12, 0.13);

    draw_graph_sheet(&draw, win_rect);

    let mouse_pos = app.mouse.position();

    // Ellipse and position text at mouse.
    draw.ellipse().wh([5.0; 2].into()).xy(mouse_pos);
    if model.view.highlight_point {
        draw.ellipse()
            .no_fill()
            .stroke_weight(2.0)
            .wh([15.0; 2].into())
            .xy(mouse_pos);
    }
    draw.text(&to_vertex_label(mouse_pos))
        .xy(mouse_pos + Vec2::new(0.0, 20.0))
        .font_size(14)
        .color(WHITE);

    draw_shapes(&draw, model, mouse_pos);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn draw_shapes(draw: &Draw, model: &Model, mouse_pos: Vec2) {
    model.db.shapes.iter().for_each(|shape| {
        draw.polygon()
            .stroke_weight(1.0)
            .points(shape.vertices.iter().cloned())
            .color(nannou::prelude::LIGHTGREY)
            .finish();
    });

    if let Some(shape) = &model.db.drawing_shape {
        if !shape.vertices.is_empty() {
            draw.polygon()
                .stroke_weight(2.0)
                .points(shape.vertices.iter().cloned())
                .color(nannou::prelude::LIGHTBLUE)
                .finish();
        }

        if let Some(&last_vertex) = shape.vertices.last() {
            draw.line()
                .stroke_weight(2.0)
                .points(last_vertex, mouse_pos)
                .color(nannou::prelude::LIGHTBLUE)
                .finish();
        }
    }
}

fn draw_graph_sheet(draw: &Draw, win_rect: Rect) {
    // 100-step and 10-step grids.
    draw_grid(draw, &win_rect, 100.0, 1.0);
    draw_grid(draw, &win_rect, 25.0, 0.5);

    // Crosshair.
    let crosshair_color = gray(0.5);
    let ends = [
        win_rect.mid_top(),
        win_rect.mid_right(),
        win_rect.mid_bottom(),
        win_rect.mid_left(),
    ];
    for &end in &ends {
        draw.arrow()
            .start_cap_round()
            .head_length(16.0)
            .head_width(8.0)
            .color(crosshair_color)
            .end(end);
    }

    // Crosshair text.
    draw.text("0.0")
        .x_y(15.0, 15.0)
        .color(crosshair_color)
        .font_size(14);

    let x_off = 30.0;
    let y_off = 20.0;

    draw.text(&format!("{:.1}", win_rect.top()))
        .h(win_rect.h())
        .font_size(14)
        .align_text_top()
        .color(crosshair_color)
        .x(x_off);

    draw.text(&format!("{:.1}", win_rect.bottom()))
        .h(win_rect.h())
        .font_size(14)
        .align_text_bottom()
        .color(crosshair_color)
        .x(x_off);

    draw.text(&format!("{:.1}", win_rect.left()))
        .w(win_rect.w())
        .font_size(14)
        .left_justify()
        .color(crosshair_color)
        .y(y_off);

    draw.text(&format!("{:.1}", win_rect.right()))
        .w(win_rect.w())
        .font_size(14)
        .right_justify()
        .color(crosshair_color)
        .y(y_off);
}

fn draw_grid(draw: &Draw, win: &Rect, step: f32, weight: f32) {
    let step_by = || (0..).map(|i| i as f32 * step);

    let r_iter = step_by().take_while(|&f| f < win.right());
    let l_iter = step_by().map(|f| -f).take_while(|&f| f > win.left());
    r_iter.chain(l_iter).for_each(|x| {
        draw.line()
            .weight(weight)
            .points(Vec2::new(x, win.bottom()), Vec2::new(x, win.top()));
    });

    let t_iter = step_by().take_while(|&f| f < win.top());
    let b_iter = step_by().map(|f| -f).take_while(|&f| f > win.bottom());
    t_iter.chain(b_iter).for_each(|y| {
        draw.line()
            .weight(weight)
            .points(Vec2::new(win.left(), y), Vec2::new(win.right(), y));
    });
}
