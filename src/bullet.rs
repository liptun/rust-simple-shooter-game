use crate::helpers;
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Bullet {
    position: Vec2,
    size: Vec2,
    color: Color,
}

impl Bullet {
    pub fn new(position: &Vec2) -> Self {
        Self {
            position: *position,
            size: Vec2::new(2., 12.),
            color: WHITE,
        }
    }

    pub fn render(&self) {
        helpers::draw_rect(&self.position, &self.size, &self.color);
    }
}
