use macroquad::prelude::*;

pub fn draw_rect(position: &Vec2, size: &Vec2, color: &Color) {
    draw_rectangle(position.x, position.y, size.x, size.y, *color);
}
