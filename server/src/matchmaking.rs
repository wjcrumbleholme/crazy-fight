// This server handles the server list on clients and then connects them to a room when they have selected them.

use std::collections::HashMap;

use common::server::{messages::{ClientToMatchmakingServer, MatchmakingServerToClient}, room_info::RoomInfo};
use futures_util::{StreamExt, SinkExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tungstenite::Message;
use std::net::SocketAddr;
use uuid::Uuid;


pub struct MatchmakingServer {
    rooms: HashMap<Uuid, RoomInfo>,
}

pub async fn run() {
    let addr = "127.0.0.1:9001";
    let listner = TcpListener::bind(&addr).await.expect("Can't bind");

    println!("Matchmaking server running on ws://{}", addr);

    while let Ok((stream, _)) = listner.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: TcpStream) {
    let ws_stream = accept_async(stream).await.expect("WebSocket accept error");
    println!("New client connected");

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                match serde_json::from_str::<ClientToMatchmakingServer>(&text) {
                    Ok(ClientToMatchmakingServer::GetRooms) => {
                        // Temporary info
                        let response = MatchmakingServerToClient::RoomDirectory(Default::default());
                        let json = serde_json::to_string(&response).unwrap();
                        write.send(Message::Text(json.into())).await.unwrap();
                    },
                    Ok(ClientToMatchmakingServer::CreateRoom{player_id, player_name, is_private, max_players}) => {
                        //Create room logic

                    },
                    Ok(ClientToMatchmakingServer::JoinRoom{ room_id, player_id, player_name }) => {
                        //Room joining logic - send back the route to the room 
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

    println!("Client disconnected")
}