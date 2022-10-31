pub struct SettingsConfig {
    pub trail_length: usize,
    pub to_show_flow_field: bool,
    pub to_show_particle_head: bool,
    pub to_pause_field: bool,
}

impl Default for SettingsConfig {
    fn default() -> Self {
        Self {
            trail_length: 200,
            to_show_flow_field: true,
            to_show_particle_head: true,
            to_pause_field: false,
        }
    }
}
