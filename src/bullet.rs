use crate::helpers;
use macroquad::prelude::*;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
pub struct Bullet {
    position: Vec2,
    size: Vec2,
    color: Color,
    direction: Direction,
}

impl Bullet {
    pub fn new(position: &Vec2, direction: Direction) -> Self {
        Self {
            position: *position,
            size: Vec2::new(2., 12.),
            color: WHITE,
            direction,
        }
    }

    pub fn update(&mut self) {
        match self.direction {
            Direction::Up => self.position.y += -10.,
            Direction::Down => self.position.y += 10.,
        }
    }

    pub fn render(&self) {
        helpers::draw_rect(&self.position, &self.size, &self.color);
    }

    pub fn is_out(&self) -> bool {
        self.position.x < 0.
            || self.position.y < 0.
            || self.position.x > 800.
            || self.position.y > 600.
    }
}
