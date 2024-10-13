mod bullet;
mod control;
mod enemy;
mod helpers;
mod player;
mod resource;

use bullet::Bullet;
use collections::storage;
use control::Control;
use enemy::{spawn_enemies, Enemy};
use macroquad::prelude::*;
use player::Player;
use resource::Resource;

#[macroquad::main("Space Duel")]
async fn main() {
    storage::store(Resource::new().await);

    let mut player = Player::new();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut wave: i32 = 0;
    let mut spawn_cooldown = 0;

    loop {
        clear_background(BLACK);
        if is_key_down(KeyCode::Escape) {
            return;
        }

        if is_key_pressed(KeyCode::H) {
            for enemy in enemies.iter() {
                bullets.push(Bullet::new(
                    &Vec2::new(enemy.position.x + enemy.size.x / 2., screen_height()),
                    bullet::Direction::Up,
                ))
            }
        }

        if enemies.len() == 0 {
            if spawn_cooldown > 0 {
                spawn_cooldown -= 1;
            } else {
                spawn_cooldown = 60;
                wave += 1;
                spawn_enemies(&mut enemies, wave);
            }
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
            if let bullet::Direction::Up = bullet.direction {
                bullet.check_collision_with_enemies(&mut enemies);
            } else {
                bullet.check_collision_with_player(&mut player);
            }
            bullet.render();
        }
        bullets.retain(|bullet| !bullet.is_out() && !bullet.destroy);
        enemies.retain(|enemy| enemy.hp > 0);

        draw_text(
            &format!("Player HP: {}", player.hp).to_string(),
            10.,
            30.,
            32.,
            WHITE,
        );
        draw_text(
            &format!("Enemies: {}", enemies.len()).to_string(),
            10.,
            60.,
            32.,
            WHITE,
        );
        if enemies.len() > 0 {
            draw_text(&format!("Wave: {}", wave).to_string(), 10., 90., 32., WHITE);
        } else {
            draw_text(
                &format!("Next wave: {}", spawn_cooldown).to_string(),
                10.,
                90.,
                32.,
                WHITE,
            );
        }

        next_frame().await;
    }
}
