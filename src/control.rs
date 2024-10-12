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
        if is_key_down(KeyCode::Left) {
            commands.insert(Command::Left);
        }
        if is_key_down(KeyCode::Right) {
            commands.insert(Command::Right);
        }
        if is_key_down(KeyCode::Space) {
            commands.insert(Command::Shoot);
        }
        commands
    }
}
