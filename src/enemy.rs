use crate::{
    bullet::{Bullet, Direction}, helpers, player::Player, resource::Resource
};
use collections::storage;
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Enemy {
    pub position: Vec2,
    pub size: Vec2,
    color: Color,
    cooldown: i32,
    pub hp: i32,
}

impl Enemy {
    pub fn new(position: &Vec2) -> Self {
        Self {
            position: *position,
            size: Vec2::new(64., 64.),
            color: RED,
            cooldown: 0,
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

    pub fn update(&mut self, bullets: &mut Vec<Bullet>, player: &Player) {
        let distance = self.position.x - player.position.x;

        if distance.abs() > 3. {
            if distance > 0. {
                self.position.x += -0.5;
            } else {
                self.position.x += 0.5;
            }
        } else {
            if self.cooldown == 0 {
                let position = Vec2::new(
                    self.position.x + self.size.x / 2.,
                    self.size.y + self.position.y,
                );
                bullets.push(Bullet::new(&position, Direction::Down));
                self.cooldown = 100;
            }
        }

        if self.cooldown > 0 {
            self.cooldown -= 1;
        }
    }

    pub fn is_collision_with_bullet(&self, bullet: &Bullet) -> bool {
        helpers::box_collision(&self.position, &self.size, &bullet.position, &bullet.size)
    }

    pub fn deal_damage(&mut self, damage: i32) {
        self.hp -= damage;
    }
}
