// Need to have a 'room' struct that gets instansiated when a room is created

use std::collections::{HashMap, HashSet};

use common::game::game_state::GameState;
use uuid::Uuid;

pub struct Room {
    room_id: Uuid,
    room_name: String,
    room_code: String,
    players: HashSet<Uuid>,
    spectators: HashSet<Uuid>,
    has_started: bool,
    host: Uuid,
}

impl Room {
    pub fn new() -> Self {
        Self {
            room_id: Uuid::new_v4(),
            room_name: "test".to_string(),
            room_code: "F7HW".to_string(),
            players: HashSet::new(),
            spectators: HashSet::new(),
            has_started: false,
            host: Uuid::new_v4(),
        }
    }
}

pub struct RoomState {
    room: Room,
    game_state: GameState,
    // connections: HashMap<Uuid, ClientConnection>
}