// Need to have a 'room' struct that gets instansiated when a room is created

use std::{collections::{HashMap, HashSet}, sync::{Arc}};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::{mpsc, RwLock};

use common::{game::game_state::GameState, server::messages::{ClientToServer, ServerToClient}};
use tokio::net::TcpStream;
use tokio_tungstenite::accept_async;
use tungstenite::Message;
use uuid::Uuid;

use crate::messages::GSToMM;

pub struct Room {
    pub room_id: Uuid,
    pub room_name: String,
    pub room_code: String,
    pub players: HashMap<Uuid, PlayerInfo>,
    pub spectators: HashSet<Uuid>,
    pub has_started: bool,
    pub mm_sender: Option<mpsc::UnboundedSender<GSToMM>>
}

impl Room {
    pub async fn broadcast(&self, msg: ServerToClient) {
        let json = serde_json::to_string(&msg).unwrap();
        for player in self.players.values() {
            let _ = player.sender.send(Message::Text(json.clone().into()));
        }
    }

}

pub struct PlayerInfo {
    pub player_id: Uuid,
    pub player_name: String,
    pub sender: mpsc::UnboundedSender<Message>
}

pub async fn handle_player_connection(stream: TcpStream, room: Arc<RwLock<Room>>) {
    // Handles the players connection to a game server
    let ws_stream = accept_async(stream).await.expect("WebSocket accept error");
    let (mut write, mut read) = ws_stream.split();

    let mut local_player_id: Option<Uuid> = None;

    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
    // Spawn task to send messages
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if write.send(msg).await.is_err() {
                break;
            }
        }
    });

    while let Some(msg) = read.next().await {
        if let Ok(Message::Text(text)) = msg {
            if let Ok(parsed) = serde_json::from_str::<ClientToServer>(&text) {
                match parsed {
                    ClientToServer::RegisterPlayer { player_name, player_id } => {
                        let mut room = room.write().await;
                        //Check if player is already in room
                        if room.players.contains_key(&player_id) {
                            let err = ServerToClient::Error("Player already connected".to_string());
                            let json = serde_json::to_string(&err).unwrap();
                            tx.send(Message::Text(json.into())).unwrap();
                            continue;
                        }
                        
                        // Add player to room
                        room.players.insert(player_id, PlayerInfo { player_id: player_id, player_name: player_name.clone(), sender: tx.clone() });

                        // Broadcast the event to all players
                        room.broadcast(ServerToClient::PlayerJoined { player_id: player_id, player_name: player_name.clone() }).await;

                        // Store the player id for later use
                        local_player_id = Some(player_id);

                        // Notify the client of a successful connection
                        let reg = ServerToClient::PlayerRegistered { player_id };
                        let _ = tx.send(Message::Text(serde_json::to_string(&reg).unwrap().into()));


                        // Notify the matchmaking server that a player joined
                        if let Some(sender) = &room.mm_sender {
                            let info = GSToMM::ClientJoinedRoom { room_id: room.room_id };
                            let _ = sender.send(info);
                        }
                        println!("Player: {} connected", player_name);

                    },
                    ClientToServer::Disconnect => {
                        break;
                    }
                }
            }
        }
    }

    // Player has disconnected, so remove them from the room and broadcast their disconnect
    if let Some(player_id) = local_player_id {
        let mut room = room.write().await;
        room.players.remove(&player_id);
        room.broadcast(ServerToClient::PlayerDisconnect { player_id }).await;

        // Notify the matchmaking server that a player disconnected
        if let Some(sender) = &room.mm_sender {
            let info = GSToMM::ClientDisconnectRoom{ room_id: room.room_id };
            let _ = sender.send(info);
        }
    }
    println!("Player {:?} disconnected", local_player_id);
}