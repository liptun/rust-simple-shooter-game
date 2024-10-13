use macroquad::prelude::*;

pub fn draw_rect(position: &Vec2, size: &Vec2, color: &Color) {
    draw_rectangle(position.x, position.y, size.x, size.y, *color);
}

pub fn box_collision(a_pos: &Vec2, a_size: &Vec2, b_pos: &Vec2, b_size: &Vec2) -> bool {
    a_pos.x < b_pos.x + b_size.x && 
    a_pos.x + a_size.x > b_pos.x && 
    a_pos.y < b_pos.y + b_size.y && 
    a_pos.y + a_size.y > b_pos.y
}
