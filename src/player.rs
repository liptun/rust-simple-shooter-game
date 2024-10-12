use std::collections::HashSet;

use crate::{bullet::{self, Bullet}, control::Command, helpers};
use macroquad::prelude::*;

pub struct Player {
    position: Vec2,
    size: Vec2,
    color: Color,
    cooldown: i32
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(400., 500.),
            size: Vec2::new(20., 30.),
            color: BLUE,
            cooldown: 0,
        }
    }

    pub fn render(&self) {
        helpers::draw_rect(&self.position, &self.size, &self.color);
    }

    pub fn update(&mut self) {
        if self.cooldown > 0 {
            self.cooldown -= 1;
        }


    }

    pub fn update_controls(&mut self, commands: HashSet<Command>, bullets: &mut Vec<Bullet>) {
        if commands.contains(&Command::Left) {
            self.position.x += -4.0;
        }
        if commands.contains(&Command::Right) {
            self.position.x += 4.0;
        }
        if commands.contains(&Command::Shoot) && self.cooldown == 0 {

            let position = Vec2::new(self.position.x + self.size.x / 2., self.position.y - 10.);

            bullets.push(Bullet::new(&position, bullet::Direction::Up));
            self.cooldown = 16;
        }
    }
}
