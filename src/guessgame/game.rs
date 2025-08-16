use rand::Rng;
use std::io;

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty{
    pub fn from_str() {

    }

    pub fn settings() {

    }
}

pub struct GameStats {
    rounds_played: u32,
    wins: u32,
    total_guesses: u32,
}

impl GameStats {

    pub fn new() {

    }
}

pub fn read_line_trim() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
