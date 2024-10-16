use std::collections::HashSet;

use crate::{
    bullet::{self, Bullet},
    control::Command,
    resource::Resource,
};
use collections::storage;
use macroquad::prelude::*;

pub struct Player {
    pub position: Vec2,
    pub size: Vec2,
    pub speed: f32,
    color: Color,
    cooldown: f32,
    pub hp: i32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(screen_width() / 2. - 32. * 2., 500.),
            size: Vec2::new(64., 64.),
            speed: 200.,
            color: BLUE,
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
                ..Default::default()
            },
        );
    }

    pub fn update(&mut self, delta: f32) {
        if self.cooldown > 0. {
            self.cooldown -= delta;
        }
    }

    pub fn update_controls(&mut self, delta: f32, commands: HashSet<Command>, bullets: &mut Vec<Bullet>) {
        if commands.contains(&Command::Left) {
            self.position.x += -self.speed * delta;
        }
        if commands.contains(&Command::Right) {
            self.position.x += self.speed * delta;
        }
        if commands.contains(&Command::Shoot) && self.cooldown <= 0. {
            let position = Vec2::new(self.position.x + self.size.x / 2., self.position.y - 10.);

            bullets.push(Bullet::new(&position, bullet::Direction::Up));
            self.cooldown = 0.3;
        }
    }

    pub fn deal_damage(&mut self, damage: i32) {
        self.hp -= damage;
    }
}
