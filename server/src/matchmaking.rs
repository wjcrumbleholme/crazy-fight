// This server handles the server list on clients and then connects them to a room when they have selected them.

use std::collections::HashMap;

use common::server::{messages::{ClientToMatchmakingServer, MatchmakingServerToClient}, room_info::RoomInfo};
use futures_util::{StreamExt, SinkExt};
use tokio::{net::{TcpListener, TcpStream}, sync::{mpsc::{self, Receiver, Sender}, oneshot}};
use tokio_tungstenite::accept_async;
use tungstenite::Message;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::messages::{GSToMM, MMToGS};


pub struct MatchmakingServer {
    rooms: HashMap<Uuid, RoomInfo>,
    room_paths: HashMap<Uuid, String>, // Room uuid, path to server with room

    // Tracks room creation requests in progress: request_id → responder
    room_creation_requests: HashMap<Uuid, oneshot::Sender<(RoomInfo, String)>>,
}

pub type SharedServer = Arc<RwLock<MatchmakingServer>>;

pub async fn run() {
    let addr = "127.0.0.1:9001";
    let listner = TcpListener::bind(&addr).await.expect("Can't bind");

    let server = Arc::new(RwLock::new(MatchmakingServer {
        rooms: HashMap::new(),
        room_paths: HashMap::new(),
        room_creation_requests: HashMap::new()
    }));

    println!("Matchmaking server running on ws://{}", addr);

    let (mm_to_gs_tx, mm_to_gs_rx) = mpsc::channel(100); // MM -> GS
    // let (gs_to_mm_tx, gs_to_mm_rx) = mpsc::channel(100); // GS -> MM
    let server_clone = Arc::clone(&server);
    tokio::spawn(handle_game_server_connection(server_clone, "ws://127.0.0.1:9002", mm_to_gs_rx));

    while let Ok((stream, _)) = listner.accept().await {
        let server_clone = Arc::clone(&server);
        let tx = mm_to_gs_tx.clone();
        tokio::spawn(handle_client_connection(stream, server_clone, tx));
    }
}


async fn handle_game_server_connection(server: SharedServer, gs_addr: &str, mut mm_to_gs_rx: Receiver<MMToGS>,) {
    let (ws_stream, _) = tokio_tungstenite::connect_async(gs_addr).await.expect("Failed to connect to game server");

    let (mut write, mut read) = ws_stream.split();

    // Send messages to the game server
    tokio::spawn(async move {
        while let Some(msg) = mm_to_gs_rx.recv().await {
            let json = serde_json::to_string(&msg).unwrap();
            write.send(Message::Text(json.into())).await.unwrap();
        }
    });

    // Handle the messages from the game server
    while let Some(msg) = read.next().await {
        if let Ok(Message::Text(text)) = msg {
            if let Ok(parsed) = serde_json::from_str::<GSToMM>(&text) {
                match parsed {
                    GSToMM::CreateRoomSuccess{request_id, room_info, port } => {
                        // add the room to the mm server
                        let room_id = room_info.id.clone();
                        let address = format!("ws://127.0.0.1:{}", port);

                        let mut server_guard = server.write().await;
                        server_guard.rooms.insert(room_id, room_info.clone());
                        server_guard.room_paths.insert(room_id, address.clone());

                        if let Some(sender) = server_guard.room_creation_requests.remove(&request_id) {
                            let _ = sender.send((room_info, address));
                        }
                    },
                    GSToMM::ClientJoinedRoom { room_id } => {
                        // Someone has joined, increment the player counter
                        let mut server_guard = server.write().await;
                        if let Some(room) = server_guard.rooms.get_mut(&room_id) {
                            room.player_count += 1;
                        }
                    },
                    GSToMM::ClientDisconnectRoom { room_id } => {
                        // Someone has left, decrement the player counter
                        let mut server_guard = server.write().await;
                        if let Some(room) = server_guard.rooms.get_mut(&room_id) {
                            room.player_count -= 1;
                        }
                    },
                    GSToMM::RoomDeleted { room_id } => {
                        // Room no longer exists, remove it from the room directory
                        let mut server_guard = server.write().await;
                        server_guard.rooms.remove(&room_id);
                        server_guard.room_paths.remove(&room_id);
                    }
                    _ => println!("Unimplemented")
                }
            }
        }
    }
}




async fn handle_client_connection(stream: TcpStream, server: SharedServer, mm_to_gs_tx: Sender<MMToGS>) {
    let ws_stream = accept_async(stream).await.expect("WebSocket accept error");
    println!("New client connected");

    let (mut write, mut read) = ws_stream.split();

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
                        let request_id = Uuid::new_v4();

                        println!("Recieved request from client to make room");

                        let (res_tx, res_rx) = oneshot::channel::<(RoomInfo, String)>();

                        // Save pending request
                        {
                            let mut guard = server.write().await;
                            guard.room_creation_requests.insert(request_id, res_tx);
                        }


                        // Ask the game server to make a room
                        mm_to_gs_tx.send(MMToGS::CreateRoom { request_id, room_name: room_name, is_private: is_private, max_players: max_players }).await.unwrap();
                        

                        // Wait for response from game server
                        if let Ok((room_info, address)) = res_rx.await {
                            let confirmation = MatchmakingServerToClient::RoomCreated {room_id: room_info.id};
                            let json = serde_json::to_string(&confirmation).unwrap();
                            write.send(Message::Text(json.into())).await.unwrap();
                        } else {
                            let error = MatchmakingServerToClient::Error("Room creation failed".into());
                            let json = serde_json::to_string(&error).unwrap();
                            write.send(Message::Text(json.into())).await.unwrap();
                        }

                    },
                    Ok(ClientToMatchmakingServer::GetRoomInfo{ room_id}) => {
                        //Room joining logic - send back the route to the room 
                        let server_guard = server.read().await;
                        if let Some(room_path) = server_guard.room_paths.get(&room_id) {
                            let info = MatchmakingServerToClient::RoomInfo {server_address: room_path.to_string()};
                            let json = serde_json::to_string(&info).unwrap();
                            write.send(Message::Text(json.into())).await.unwrap();
                        } else {
                            let error = MatchmakingServerToClient::Error("Room not found".to_string());
                            let json = serde_json::to_string(&error).unwrap();
                            write.send(Message::Text(json.into())).await.unwrap();
                        }
                        
                    },
                    Ok(ClientToMatchmakingServer:: Disconnect) => {
                        println!("Client requested disconnect");

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
    
}
