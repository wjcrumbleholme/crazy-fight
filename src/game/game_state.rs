pub struct GameState {
    round: i32,
    phase: Phase
}

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