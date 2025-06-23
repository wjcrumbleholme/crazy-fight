// This handles the central rooms that may be created - talks to a matchmaking server

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

use common::server::room_info::RoomInfo;
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tungstenite::Message;
use uuid::Uuid;

use crate::{common::{handle_player_connection, Room}, messages::{GSToMM, MMToGS}};

//Need to have a room manager that has all of the rooms currently running - although each game will be its own seperate task
pub struct CentralServer {
    pub rooms: HashMap<Uuid, Arc<RwLock<Room>>>,
    pub avaliable_ports: Vec<u16>,
    pub mm_sender: Option<mpsc::UnboundedSender<GSToMM>>
}

pub type SharedServer = Arc<RwLock<CentralServer>>;

// Create room server and bind it to port 9002
pub async fn run() {
    let addr = "127.0.0.1:9002";
    let listner = TcpListener::bind(&addr).await.expect("Can't bind");

    let server = Arc::new(RwLock::new(CentralServer {
        rooms: HashMap::new(),
        avaliable_ports: vec![9003, 9004, 9005, 9006, 9007, 9008, 9009, 9010],
        mm_sender: None,
    }));

    println!("Central game server running on ws://{}", addr);

    while let Ok((stream, _)) = listner.accept().await {
        let server_clone = Arc::clone(&server);
        tokio::spawn(handle_matchmaking_connection(stream, server_clone));
    }
}


// Handle the connection to the matchmaking server
async fn handle_matchmaking_connection(stream: TcpStream, server: SharedServer) {
    let ws_stream = accept_async(stream).await.expect("WebSocket accept error");
    println!("Match making server connected");

    let (mut write, mut read) = ws_stream.split();

    // Channel for sending messages to matchmaking server (write half)
    let (to_write_tx, mut to_write_rx) = mpsc::unbounded_channel::<String>();

    let (mm_tx, mut mm_rx) = mpsc::unbounded_channel::<GSToMM>();

    // Save sender to server state
    {
        let mut server_guard = server.write().await;
        server_guard.mm_sender = Some(mm_tx);
    }

    // Task to forward messages from game server to matchmaking server
    let mut write_clone = write;
    tokio::spawn(async move {
        while let Some(json) = to_write_rx.recv().await {
            if write_clone.send(Message::Text(json.into())).await.is_err() {
                break;
            }
        }
    });

    // Task to forward GSToMM messages to matchmaking server
    let to_write_tx_clone = to_write_tx.clone();
    tokio::spawn(async move {
        while let Some(msg) = mm_rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                let _ = to_write_tx_clone.send(json);
            }
        }
    });
    // ------------------------

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                match serde_json::from_str::<MMToGS>(&text) {
                    Ok(MMToGS::CreateRoom{request_id, room_name, is_private, max_players}) => {
                        //Create a room and then accept connections to it

                        match spawn_room(room_name.clone(), is_private, max_players, Arc::clone(&server)).await {
                            Ok((room_info, port)) => {
                                // Room created succesfully - return the route to this game server and add it to the room list
                                let data = GSToMM::CreateRoomSuccess{request_id, room_info, port};
                                let json = serde_json::to_string(&data).unwrap();
                                let _ = to_write_tx.send(json);
                                println!("Room: {} created successfully", room_name)
                            },
                            Err(e) => {
                                println!("Failed to create room: {}", e)
                            }
                            
                        }

                    },
                    _ => {
                        let error = GSToMM::Error("Unknown or unsupported message".to_string());
                        let json = serde_json::to_string(&error).unwrap();
                        let _ = to_write_tx.send(json);
                    }
                    
                }
            }
            _ => {
                println!("Invalid WebSocket message of the connection has been closed");
                break;
            }
        }
    }
}



// Handles the creation of a room 
async fn spawn_room(room_name: String, is_private: bool, max_players: usize, server: SharedServer) -> Result<(RoomInfo, u16), String> {
    // Try to get a port and error if not
    let port = {
        let server_guard = server.read().await;
        if let Some(port) = server_guard.avaliable_ports.first() {
            *port
        } else {
            return Err("No more ports avaliable".to_string());
        }
    };

    let addr = format!("127.0.0.1:{}", port);
    if let Ok(listener) = TcpListener::bind(&addr).await {
        println!("Room {} listening on {}", room_name, addr);

        let room_id = Uuid::new_v4();

        let room_info = RoomInfo {
            id: room_id,
            name: room_name.clone(),
            max_players: max_players,
            player_count: 0,
            is_private: is_private,
            has_started: false
        };

        // Get mm_sender (clone) before acquiring write lock
        let mm_sender = {
            let server_guard = server.read().await;
            server_guard.mm_sender.clone()
        };

        let room = Arc::new(RwLock::new(Room {
            room_id: room_id,
            room_name: room_name.clone(),
            room_code: "GHZX".to_string(),
            players: HashMap::new(),
            spectators: HashSet::new(),
            has_started: false,
            mm_sender,
        }));

        // Spawn new room
        let room_clone = Arc::clone(&room);
        tokio::spawn(async move {
            // Listen out for player actions
            while let Ok((stream, _)) = listener.accept().await {
                let room_for_player = Arc::clone(&room_clone);
                tokio::spawn(handle_player_connection(stream, room_for_player));
            }
        });

        // Now the room has been made, remove the port from the avaliable ports and add the room to the rooms list
        let mut server_guard = server.write().await;

        server_guard.avaliable_ports.remove(0);
        server_guard.rooms.insert(room_id, room);

        Ok((room_info, port))
    } else {
        Err("Cannot bind address".to_string())
    }
    

}