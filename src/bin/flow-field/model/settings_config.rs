pub struct SettingsConfig {
    pub trail_length: usize,
    pub to_show_flow_field: bool,
    pub to_show_particle_head: bool,
}

impl Default for SettingsConfig {
    fn default() -> Self {
        Self {
            trail_length: 10,
            to_show_flow_field: true,
            to_show_particle_head: true,
        }
    }
}
