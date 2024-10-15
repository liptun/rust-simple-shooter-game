use crate::{
    bullet::{Bullet, Direction},
    helpers,
    player::Player,
    resource::Resource,
    state::State,
};
use collections::storage;
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Enemy {
    pub position: Vec2,
    pub size: Vec2,
    color: Color,
    cooldown: f32,
    pub hp: i32,
}

impl Enemy {
    pub fn new(position: &Vec2) -> Self {
        println!("New enemy at: {}", position);
        Self {
            position: *position,
            size: Vec2::new(64., 64.),
            color: RED,
            cooldown: 0.,
            hp: 100,
        }
    }

    pub fn render(&self) {
        let resource = storage::get::<Resource>();
        draw_texture_ex(
            &resource.player,
            self.position.x,
            self.position.y,
            self.color,
            DrawTextureParams {
                dest_size: Some(self.size),
                flip_y: true,
                ..Default::default()
            },
        );
    }

    pub fn update(
        &mut self,
        delta: f32,
        bullets: &mut Vec<Bullet>,
        player: &Player,
        state: &State,
    ) {
        let distance = self.position.x - player.position.x;

        if player.hp <= 0 {
            self.position.y -= delta * -200.;
        } else if distance.abs() > state.wave as f32 * 20. {
            if distance > 0. {
                self.position.x += -10. * state.wave as f32 * delta;
            } else {
                self.position.x += 10. * state.wave as f32 * delta;
            }
        } else {
            if self.cooldown <= 0. {
                let position = Vec2::new(
                    self.position.x + self.size.x / 2.,
                    self.size.y + self.position.y,
                );
                bullets.push(Bullet::new(&position, Direction::Down));

                self.cooldown = 0.5;
            }
        }

        if self.cooldown > 0. {
            self.cooldown -= delta;
        }
    }

    pub fn is_collision_with_bullet(&self, bullet: &Bullet) -> bool {
        helpers::box_collision(&self.position, &self.size, &bullet.position, &bullet.size)
    }

    pub fn deal_damage(&mut self, damage: i32) {
        self.hp -= damage;
    }
}

pub fn spawn_enemies(enemies: &mut Vec<Enemy>, n: i32) {
    let spacing = screen_width() / n as f32;
    for i in 1..=n {
        let mut new_enemy = Enemy::new(&Vec2::new(i as f32 * spacing - spacing / 2., 10.));
        new_enemy.position.x -= new_enemy.size.x / 2.;
        enemies.push(new_enemy);
    }
}
