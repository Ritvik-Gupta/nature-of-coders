use crate::model::Model;
use nannou::{
    prelude::{gray, Rect, Vec2, WHITE},
    App, Draw, Frame,
};

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.main_window();
    let win_rect = window.rect();
    draw.background().rgb(0.11, 0.12, 0.13);

    // 100-step and 10-step grids.
    draw_grid(&draw, &win_rect, 100.0, 1.0);
    draw_grid(&draw, &win_rect, 25.0, 0.5);

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

    // Ellipse at mouse.
    draw.ellipse().wh([5.0; 2].into()).xy(app.mouse.position());

    // Mouse position text.
    let mouse = app.mouse.position();
    let pos = format!("[{:.1}, {:.1}]", mouse.x, mouse.y);
    draw.text(&pos)
        .xy(mouse + Vec2::new(0.0, 20.0))
        .font_size(14)
        .color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_grid(draw: &Draw, win: &Rect, step: f32, weight: f32) {
    let step_by = || (0..).map(|i| i as f32 * step);
    let r_iter = step_by().take_while(|&f| f < win.right());
    let l_iter = step_by().map(|f| -f).take_while(|&f| f > win.left());
    let x_iter = r_iter.chain(l_iter);
    for x in x_iter {
        draw.line()
            .weight(weight)
            .points(Vec2::new(x, win.bottom()), Vec2::new(x, win.top()));
    }
    let t_iter = step_by().take_while(|&f| f < win.top());
    let b_iter = step_by().map(|f| -f).take_while(|&f| f > win.bottom());
    let y_iter = t_iter.chain(b_iter);
    for y in y_iter {
        draw.line()
            .weight(weight)
            .points(Vec2::new(win.left(), y), Vec2::new(win.right(), y));
    }
}
