mod bullet;
mod control;
mod helpers;
mod player;
mod resource;

use bullet::Bullet;
use collections::storage;
use control::Control;
use macroquad::prelude::*;
use player::Player;
use resource::Resource;

#[macroquad::main("Space Duel")]
async fn main() {
    storage::store(Resource::new().await);

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

        bullets.retain(|bullet| !bullet.is_out());

        player.update();
        player.render();
        next_frame().await;
    }
}
