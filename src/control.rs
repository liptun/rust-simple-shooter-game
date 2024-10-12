use macroquad::prelude::*;

pub struct Control;

#[derive(PartialEq)]
pub enum Command {
    Left,
    Right,
    Shoot,
}

impl Control {
    pub fn update() -> Vec<Command> {
        let mut commands: Vec<Command> = Vec::new();
        if is_key_down(KeyCode::Left) {
            commands.push(Command::Left)
        }
        if is_key_down(KeyCode::Right) {
            commands.push(Command::Right)
        }
        if is_key_down(KeyCode::Space) {
            commands.push(Command::Shoot)
        }
        commands
    }
}
