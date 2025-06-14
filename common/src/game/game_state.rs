use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameState {
    round: i32,
    phase: Phase
}

#[derive(Serialize, Deserialize)]
pub enum Phase {
    Round,
    Battle
}


impl GameState {
    pub fn new() -> Self {
        Self {
            round: 1,
            phase: Phase::Round,
        }
    }
}