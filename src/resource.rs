use macroquad::prelude::*;

pub struct Resource {
    pub player: Texture2D,
}

impl Resource {
    pub async fn new() -> Self {
        let player = load_texture("src/player.png").await.unwrap();
        player.set_filter(FilterMode::Nearest);

        Self { player }
    }
}
