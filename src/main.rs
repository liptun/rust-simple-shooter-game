mod bullet;
mod control;
mod helpers;
mod player;

use bullet::Bullet;
use control::Control;
use macroquad::prelude::*;
use player::Player;

#[macroquad::main("Space Duel")]
async fn main() {
    let mut player = Player::new();
    let mut bullets: Vec<Bullet> = Vec::new();

    loop {
        clear_background(BLACK);
        if is_key_down(KeyCode::Escape) {
            return;
        }

        player.update_controls(Control::update(), &mut bullets);

        for bullet in bullets.iter_mut() {
            bullet.update();
            bullet.render();
        }

        bullets = bullets
            .into_iter()
            .filter(|bullet| !bullet.is_out())
            .collect();

        player.update();
        player.render();
        next_frame().await;
    }
}
