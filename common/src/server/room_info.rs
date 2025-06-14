use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct RoomInfo {
    id: Uuid,
    name: String,
    max_players: usize,
    player_count: usize,
    is_private: bool,
    has_started: bool,
}