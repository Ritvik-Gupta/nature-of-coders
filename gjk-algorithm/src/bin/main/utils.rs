use nannou::prelude::Vec2;

pub fn to_vertex_label(pos: Vec2) -> String {
    format!("[{:.1}, {:.1}]", pos.x, pos.y)
}
