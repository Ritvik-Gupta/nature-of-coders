use gjk_algorithm::Shape;

pub struct Database {
    pub shapes: Vec<Shape>,
    pub drawing_shape: Option<Shape>,
}
