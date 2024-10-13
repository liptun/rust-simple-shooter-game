use crate::{enemy::Enemy, helpers, player::Player};
use macroquad::prelude::*;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
pub struct Bullet {
    pub position: Vec2,
    pub size: Vec2,
    color: Color,
    pub direction: Direction,
    pub destroy: bool,
    speed: f32,
}

impl Bullet {
    pub fn new(position: &Vec2, direction: Direction) -> Self {
        Self {
            position: *position,
            size: Vec2::new(2., 12.),
            color: WHITE,
            direction,
            destroy: false,
            speed: 4.,
        }
    }

    pub fn update(&mut self) {
        match self.direction {
            Direction::Up => self.position.y += -self.speed,
            Direction::Down => self.position.y += self.speed,
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

    pub fn check_collision_with_enemies(&mut self, enemies: &mut Vec<Enemy>) {
        for enemy in enemies.iter_mut() {
            if enemy.is_collision_with_bullet(&self) {
                enemy.deal_damage(100);
                self.destroy = true;
                println!("collison {:?} {:?}", enemy, &self);
                break;
            }
        }
    }

    pub fn check_collision_with_player(&mut self, player: &mut Player) {
        if helpers::box_collision(&self.position, &self.size, &player.position, &player.size) {
            self.destroy = true;
            player.deal_damage(20);
        }
    }
}
