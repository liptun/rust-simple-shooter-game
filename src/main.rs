mod control;
mod helpers;
mod player;
use control::Control;
use macroquad::prelude::*;
use player::Player;

#[macroquad::main("Space Duel")]
async fn main() {
    let mut player = Player::new();

    loop {
        if is_key_down(KeyCode::Escape) {
            return;
        }
        player.update_controls(Control::update());

        clear_background(BLACK);
        player.render();
        next_frame().await;
    }
}
