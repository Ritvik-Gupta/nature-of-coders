use nannou::prelude::Vec2;

structstruck::strike! {
    pub struct WindowConfig {
        pub mouse: struct MouseConfig {
            pub location: Vec2,
            pub is_pressed: bool,
        },
    }
}
