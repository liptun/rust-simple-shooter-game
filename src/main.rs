mod bullet;
mod control;
mod helpers;
mod player;
mod resource;
mod enemy;

use bullet::Bullet;
use collections::storage;
use control::Control;
use enemy::Enemy;
use macroquad::prelude::*;
use player::Player;
use resource::Resource;

#[macroquad::main("Space Duel")]
async fn main() {
    storage::store(Resource::new().await);

    let mut player = Player::new();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();

    enemies.push(Enemy::new(&Vec2::new(10.,10.)));


    loop {
        clear_background(BLACK);
        if is_key_down(KeyCode::Escape) {
            return;
        }

        player.update_controls(Control::update(), &mut bullets);
        player.update();
        player.render();

        for enemy in enemies.iter_mut() {
            enemy.update(&mut bullets, &player);
            enemy.render();
        }

        for bullet in bullets.iter_mut() {
            bullet.update();
            bullet.render();
        }
        bullets.retain(|bullet| !bullet.is_out());

        next_frame().await;
    }
}
