use std::collections::HashSet;

use macroquad::prelude::*;

pub struct Control;

#[derive(PartialEq, Eq, Hash)]
pub enum Command {
    Left,
    Right,
    Shoot,
}

impl Control {
    pub fn update() -> HashSet<Command> {
        let mut commands: HashSet<Command> = HashSet::new();
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            commands.insert(Command::Left);
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            commands.insert(Command::Right);
        }
        if is_key_down(KeyCode::Space) {
            commands.insert(Command::Shoot);
        }
        commands
    }
}
