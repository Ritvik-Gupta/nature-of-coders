use super::particle::Particle;

structstruck::strike! {
    pub struct ViewConfig {
        pub particles: Vec<Particle>,
        pub field: struct FieldConfig {
            pub time: f32,
            pub is_paused: bool,
        }
    }
}
