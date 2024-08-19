#![allow(dead_code)]

use std::io::{stdin, stdout, Write};

#[derive(Debug, PartialEq)]
pub enum Action {
    Left,
    Right,
    UP,
    Down,
    Damage,
    Heal,
    Save,
    Quit,
}

#[derive(Debug, PartialEq)]
pub enum ActionReadError {
    UnknownAction(String),
}

impl Action {
    pub fn read_stdin() -> Result<Self, ActionReadError> {
        let mut buf = String::new();

        print!("Enter action (Left, Right, Up, Down, dAmage, Heal, Save, Quit): ");
        stdout().flush().unwrap();

        let _ = stdin().read_line(&mut buf);

        match buf.trim().to_lowercase().as_str() {
            "left" | "l" => Ok(Self::Left),
            "right" | "r" => Ok(Self::Right),
            "up" | "u" => Ok(Self::UP),
            "down" | "d" => Ok(Self::Down),
            "damage" | "a" => Ok(Self::Damage),
            "heal" | "h" => Ok(Self::Heal),
            "save" | "s" => Ok(Self::Save),
            "quit" | "q" => Ok(Self::Quit),
            _ => Err(ActionReadError::UnknownAction(buf)),
        }
    }
}
