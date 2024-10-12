mod control;
mod helpers;
mod player;
mod bullet;
use bullet::Bullet;
use control::Control;
use macroquad::prelude::*;
use player::Player;

#[macroquad::main("Space Duel")]
async fn main() {
    let mut player = Player::new();
    let mut bullets: Vec<Bullet> = Vec::new();

    bullets.push(Bullet::new(&Vec2::new(500., 500.)));

    println!("{:?}", bullets);

    loop {
        if is_key_down(KeyCode::Escape) {
            return;
        }
        player.update_controls(Control::update());


        clear_background(BLACK);

        for bullet in bullets.iter() {
            bullet.render();
        }
        player.render();
        next_frame().await;
    }
}
