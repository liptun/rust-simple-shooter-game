pub struct State {
    pub wave: i32,
    pub spawn_cooldown: f32,
    pub game_over_countdown: f32,
}

impl State {
    pub fn new() -> Self {
        Self {
            wave: 0,
            spawn_cooldown: 0.,
            game_over_countdown: 3.,
        }
    }
}
