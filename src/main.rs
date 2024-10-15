mod bullet;
mod control;
mod enemy;
mod helpers;
mod player;
mod resource;
mod state;

use bullet::Bullet;
use collections::storage;
use control::Control;
use enemy::{spawn_enemies, Enemy};
use macroquad::prelude::*;
use player::Player;
use resource::Resource;
use state::State;

#[macroquad::main("Space Duel")]
async fn main() {
    storage::store(Resource::new().await);

    let mut player = Player::new();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut state = State::new();
    let mut delta: f32;

    loop {
        delta = macroquad::time::get_frame_time();
        let max_delta = 0.05; // About 20 FPS
        if delta > max_delta {
            delta = max_delta;
        }
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
            if state.spawn_cooldown > 0. {
                state.spawn_cooldown -= delta;
            } else {
                state.spawn_cooldown = 3.;
                state.wave += 1;
                spawn_enemies(&mut enemies, state.wave);
                if state.wave > 1 {
                    player.hp += 2 * state.wave;
                }
            }
        }

        if player.hp > 0 {
            player.update_controls(delta, Control::update(), &mut bullets);
            player.update(delta);
            player.render();
        } else {
            state.game_over_countdown -= delta;
            draw_text(
                "Game over",
                screen_width() / 2. - 200.,
                screen_height() / 2.,
                100.,
                WHITE,
            );
            if state.game_over_countdown <= 0. {
                return;
            }
        }

        for enemy in enemies.iter_mut() {
            enemy.update(delta, &mut bullets, &player, &state);
            enemy.render();
        }

        for bullet in bullets.iter_mut() {
            bullet.update(delta);
            if let bullet::Direction::Up = bullet.direction {
                bullet.check_collision_with_enemies(&mut enemies);
            } else {
                bullet.check_collision_with_player(&mut player);
            }
            bullet.render();
        }
        bullets.retain(|bullet| !bullet.is_out() && !bullet.destroy);
        enemies.retain(|enemy| enemy.hp > 0);

        if player.hp > 0 {
            draw_text(
                &format!("Player HP: {}", player.hp).to_string(),
                10.,
                30.,
                32.,
                WHITE,
            );
        } else {
            draw_text(&format!("Player is dead").to_string(), 10., 30., 32., WHITE);
        }
        draw_text(
            &format!("Enemies: {}", enemies.len()).to_string(),
            10.,
            60.,
            32.,
            WHITE,
        );
        if enemies.len() > 0 {
            draw_text(
                &format!("Wave: {}", state.wave).to_string(),
                10.,
                90.,
                32.,
                WHITE,
            );
        } else {
            draw_text(
                &format!("Next wave: {}", state.spawn_cooldown).to_string(),
                10.,
                90.,
                32.,
                WHITE,
            );
        }

        next_frame().await;
    }
}
