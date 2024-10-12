use macroquad::prelude::*;
use crate::{control::Command, helpers};

pub struct Player {
    position: Vec2,
    size: Vec2,
    color: Color,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(100., 100.),
            size: Vec2::new(20., 30.),
            color: BLUE,
        }
    }

    pub fn render(&self) {
        helpers::draw_rect(&self.position, &self.size, &self.color);
    }

    pub fn update_controls(&mut self, commands: Vec<Command>) {
        if commands.contains(&Command::Left) {
            self.position.x += -4.0;
        }
        if commands.contains(&Command::Right) {
            self.position.x += 4.0;
        }
    }

}

