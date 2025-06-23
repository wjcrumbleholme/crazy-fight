use common::server::room_info::RoomInfo;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Link between matchmaking and gameserver
#[derive(Serialize, Deserialize)]
pub enum MMToGS {
    CreateRoom{request_id: Uuid, room_name: String, is_private: bool, max_players: usize},
}

// Link between gameserver and matchmaking
#[derive(Serialize, Deserialize)]
pub enum GSToMM {
    CreateRoomSuccess{request_id: Uuid, room_info: RoomInfo, port: u16},
    ClientJoinedRoom{room_id: Uuid},
    ClientDisconnectRoom{room_id: Uuid},
    RoomDeleted{room_id: Uuid},
    Error(String),
}