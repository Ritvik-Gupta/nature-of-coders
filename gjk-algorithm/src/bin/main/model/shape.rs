use nannou::prelude::Vec2;

pub struct Shape {
    vertices: Vec<Vec2>,
}

impl Shape {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    pub fn regular_polygon(poly: RegularPolygon, centroid: Vec2) -> Self {
        use RegularPolygon::*;

        match poly {
            Triangle => todo!(),
            Square => todo!(),
            Pentagon => todo!(),
            Hexagon => todo!(),
        }
    }
}

pub enum RegularPolygon {
    Triangle,
    Square,
    Pentagon,
    Hexagon,
}
