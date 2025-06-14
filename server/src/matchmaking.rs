// This server handles the server list on clients and then connects them to a room when they have selected them.

use std::collections::HashMap;

use common::server::{messages::{ClientToMatchmakingServer, MatchmakingServerToClient}, room_info::RoomInfo};
use futures_util::{StreamExt, SinkExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tungstenite::Message;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;


pub struct MatchmakingServer {
    rooms: HashMap<Uuid, RoomInfo>,
    room_paths: HashMap<Uuid, String>,
    client_rooms: HashMap<Uuid, Uuid> // Player, Room
}

pub type SharedServer = Arc<RwLock<MatchmakingServer>>;

pub async fn run() {
    let addr = "127.0.0.1:9001";
    let listner = TcpListener::bind(&addr).await.expect("Can't bind");

    let server = Arc::new(RwLock::new(MatchmakingServer {
        rooms: HashMap::new(),
        room_paths: HashMap::new(),
        client_rooms: HashMap::new(),
    }));

    println!("Matchmaking server running on ws://{}", addr);

    while let Ok((stream, _)) = listner.accept().await {
        let server_clone = Arc::clone(&server);
        tokio::spawn(handle_connection(stream, server_clone));
    }
}

async fn handle_connection(stream: TcpStream, server: SharedServer) {
    let ws_stream = accept_async(stream).await.expect("WebSocket accept error");
    println!("New client connected");

    let (mut write, mut read) = ws_stream.split();
    let mut current_player_id: Option<Uuid> = None;

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                match serde_json::from_str::<ClientToMatchmakingServer>(&text) {
                    Ok(ClientToMatchmakingServer::GetRooms) => {
                        // Temporary info
                        println!("Sending room directory");
                        // READ lock since we're not modifying
                        let server_guard = server.read().await;
                        let response = MatchmakingServerToClient::RoomDirectory(
                            server_guard.rooms.clone() // Make sure RoomInfo is Clone
                        );
                        let json = serde_json::to_string(&response).unwrap();
                        write.send(Message::Text(json.into())).await.unwrap();
                    },
                    Ok(ClientToMatchmakingServer::CreateRoom{room_name, is_private, max_players}) => {
                        //Create room logic - need to add to the central game server in the future
                        let room_id = Uuid::new_v4();
                        let room = RoomInfo {
                            id: room_id,
                            name: room_name,
                            max_players: max_players,
                            player_count: 0,
                            is_private: is_private,
                            has_started: false,
                        };

                        // WRITE lock to modify rooms
                        let mut server_guard = server.write().await;
                        server_guard.rooms.insert(room_id, room);
                        // TEMPORARY - later include the actuall room path
                        server_guard.room_paths.insert(room_id, "".to_string());

                        // When confirming, send the room id back so that the client can then join the room immediately after
                        let confirmation = MatchmakingServerToClient::RoomCreated { room_id: room_id };
                        let json = serde_json::to_string(&confirmation).unwrap();
                        write.send(Message::Text(json.into())).await.unwrap();

                    },
                    Ok(ClientToMatchmakingServer::GetRoomInfo{ room_id}) => {
                        //Room joining logic - send back the route to the room 
                        let server_guard = server.read().await;
                        if let Some(room_path) = server_guard.room_paths.get(&room_id) {
                            let info = MatchmakingServerToClient::RoomInfo { room_id: room_id, server_address: room_path.to_string()};
                            let json = serde_json::to_string(&info).unwrap();
                            write.send(Message::Text(json.into())).await.unwrap();
                        } else {
                            let error = MatchmakingServerToClient::Error("Room not found".to_string());
                            let json = serde_json::to_string(&error).unwrap();
                            write.send(Message::Text(json.into())).await.unwrap();
                        }
                        
                    },
                    Ok(ClientToMatchmakingServer::JoinedRoom { room_id, player_id }) => {
                        //Increment the relavent room player count by 1
                        let mut server_guard = server.write().await;
                        if let  Some(room) = server_guard.rooms.get_mut(&room_id) {
                            room.player_count += 1;
                            server_guard.client_rooms.insert(player_id, room_id);
                            current_player_id = Some(player_id);
                        }
                    },
                    Ok(ClientToMatchmakingServer:: Disconnect) => {
                        println!("Client requested disconnect");

                        // Perform cleanup
                        if let Some(player_id) = current_player_id {
                            let mut server_guard = server.write().await;
                            if let Some(room_id) = server_guard.client_rooms.remove(&player_id) {
                                if let Some(room) = server_guard.rooms.get_mut(&room_id) {
                                    if room.player_count > 0 {
                                        room.player_count -= 1;
                                    }
                                }
                            }
                        }

                        // Close the connection
                        write.close().await.unwrap_or_else(|e| {
                            eprintln!("Failed to close WebSocket: {}", e);
                        });

                        break;
                    }
                    _ => {
                        let error = MatchmakingServerToClient::Error("Unknown or unsupported message".to_string());
                        let json = serde_json::to_string(&error).unwrap();
                        write.send(Message::Text(json.into())).await.unwrap()
                    }
                    
                }
            }
            _ => {
                println!("Invalid WebSocket message of the connection has been closed");
                break;
            }
        }
    }

    println!("Client disconnected");
    if let Some(player_id) = current_player_id {
    let mut server_guard = server.write().await;

    if let Some(room_id) = server_guard.client_rooms.remove(&player_id) {
        if let Some(room) = server_guard.rooms.get_mut(&room_id) {
            if room.player_count > 0 {
                room.player_count -= 1;
            }
        }
    }
}
}