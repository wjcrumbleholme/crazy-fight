use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct RoomInfo {
    pub id: Uuid,
    pub name: String,
    pub max_players: usize,
    pub player_count: usize,
    pub is_private: bool,
    pub has_started: bool,
}