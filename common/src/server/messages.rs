// These are structs for the messages sent from client to server and from server to client

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use uuid::Uuid;

use crate::{game::game_state::GameState, server::room_info::RoomInfo};

#[derive(Serialize, Deserialize)]
pub enum ClientToMatchmakingServer {
    /// Get a list of public rooms
    GetRooms, 

    /// Create a new public room
    CreateRoom {player_id: Uuid, player_name: String, is_private: bool, max_players: usize}, 

    /// Connect to a room
    JoinRoom {room_id: Uuid, player_id: Uuid, player_name: String},


}

#[derive(Serialize, Deserialize)]
pub enum MatchmakingServerToClient {
    /// Return a list of public rooms - no connection info though
    RoomDirectory(HashMap<Uuid, RoomInfo>),

    /// Let the client know that a room has been created 
    RoomCreated {room_code: String},

    /// Let the client know that the room has been joined and where to go now.
    RoomJoined {server_address: String, room_code: String},

    /// If something happens - room full, cant create room, etc
    Error(String)
}

#[derive(Serialize, Deserialize)]
pub enum ClientToServer {
    /// Register a player to the room 
    RegisterPlayer {
        player_name: String,
        player_id: Uuid,
    },

    // TODO - Add in all of the other possible actions

    /// Leave the room
    Disconnect
}

#[derive(Serialize, Deserialize)]
pub enum ServerToClient {
    /// Acknowledgement of successful connection
    PlayerRegistered {
        player_id: Uuid,
    },

    /// Another player has joined the room
    PlayerJoined {
        player_id: Uuid,
        player_name: String,
    },

    /// A player has left the room
    PlayerDisconnect {
        player_id: Uuid,
    },

    /// Update the entire game state
    GameStateUpdate(GameState),

    // TODO - Add in all of the other possible actions


    /// Error message
    Error(String),
}