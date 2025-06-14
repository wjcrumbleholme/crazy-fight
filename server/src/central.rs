// This handles the central rooms that may be created - talks to a matchmaking server

use std::collections::HashMap;

use uuid::Uuid;

use crate::common::RoomState;

//Need to have a room manager that has all of the rooms currently running on this server
pub struct CentralServer {
    rooms: HashMap<Uuid, RoomState>
}

