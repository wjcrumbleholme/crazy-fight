
use std::collections::HashMap;

use common::server::{messages::{ClientToMatchmakingServer, MatchmakingServerToClient}, room_info::RoomInfo};
use macroquad::prelude::*;
use uuid::Uuid;
mod net;
mod ui;
use crate::{menu::{DirectConnect, InRoom, MainMenu, MatchmakingConnectionError, MenuState, RoomBrowser}, net::{platform, ConnectionResult, WebSocketClient}, ui::{container::Container, Alignment, Position, Size, UIContext, UIElement, UIMessage}};
use net::WsMessage;

mod menu;


pub struct AppState {
    pub menu_state: MenuState,
    pub matchmaking_client: Option<Box<dyn WebSocketClient>>,
    pub rooms: HashMap<Uuid, RoomInfo>,
    pub error_message: Option<String>,
    pub player_id: Uuid,
}

#[macroquad::main("Client")]
async fn main() {
    let mut app_state = AppState {
        menu_state: MenuState::MainMenu,
        matchmaking_client: None,
        error_message: None,
        rooms: HashMap::new(),
        player_id: Uuid::new_v4(),
    };

    let mut ctx = UIContext::new();

    let mut main_menu = MainMenu::new();
    let mut room_browser = RoomBrowser::new();
    let mut direct_connect = DirectConnect::new();
    let mut matchmaking_connection_error = MatchmakingConnectionError::new();
    let mut in_room = InRoom::new();

    loop {
        clear_background(BLACK);

        match app_state.menu_state {
            MenuState::MainMenu => {
                main_menu.container.borrow_mut().draw(&mut ctx, 0.0, 0.0, screen_width(), screen_height());

                // Also try and disconnect the client from any matchmaking server
                if let Some(mm_client) = app_state.matchmaking_client.as_ref() {
                    mm_client.send_text(&serde_json::to_string(&ClientToMatchmakingServer::Disconnect).unwrap());
                }
            },
            MenuState::RoomBrowser => {
                room_browser.container.borrow_mut().draw(&mut ctx, 0.0, 0.0, screen_width(), screen_height());
            },
            MenuState::DirectConnect => {
                direct_connect.container.borrow_mut().draw(&mut ctx, 0.0, 0.0, screen_width(), screen_height());
            },
            MenuState::InGame => {
                // Placeholder for in-game logic
            },
            MenuState::MatchmakingConnectionError => {
                matchmaking_connection_error.container.borrow_mut().draw(&mut ctx, 0.0, 0.0, screen_width(), screen_height());
            },
            MenuState::InRoom => {
                in_room.container.borrow_mut().draw(&mut ctx, 0.0, 0.0, screen_width(), screen_height());
            }
        }

        for msg in ctx.message_queue.drain(..) {
            match msg {
                UIMessage::SwitchView(new_state) => {
                    app_state.menu_state = new_state;
                },
                UIMessage::DrawCard(_card_id) => {
                    // Implement draw logic later
                },
                UIMessage::TryConnectToMatchmaking => {
                    match platform::connect("ws://localhost:9001").await {
                        ConnectionResult::Success(client) => {
                            client.send_text(&serde_json::to_string(&ClientToMatchmakingServer::GetRooms).unwrap());
                            app_state.matchmaking_client = Some(client);
                            app_state.menu_state = MenuState::RoomBrowser;
                        },
                        ConnectionResult::Failure(err) => {
                            app_state.error_message = Some(err);
                            app_state.menu_state = MenuState::MatchmakingConnectionError;
                        }
                    }
                },
                UIMessage::CreateRoom => {
                    if let Some(client) = app_state.matchmaking_client.as_ref() {
                        client.send_text(&serde_json::to_string(&ClientToMatchmakingServer::CreateRoom {room_name: "test".to_string(), is_private: false, max_players: 8 }).unwrap());
                    }
                },
                UIMessage::JoinRoom(room_id) => {
                    if let Some(client) = &app_state.matchmaking_client { 
                        client.send_text(&serde_json::to_string(&ClientToMatchmakingServer::GetRoomInfo { room_id: room_id }).unwrap());
                    }
                }
            }
        }

        // Handle incoming messages
        // Extract messages first to avoid borrowing app_state immutably and mutably at the same time
        let mut messages = Vec::new();
        let connection_failed;
        if let Some(client) = &app_state.matchmaking_client {
            while let Some(WsMessage::Text(text)) = client.try_recv() {
                messages.push(text);
            }
            connection_failed = client.connection_failed();
        } else {
            connection_failed = false;
        }

        for text in messages {
            process_matchmaking_server_message(&text, &mut app_state).await;
            let rooms_vec: Vec<RoomInfo> = app_state.rooms.values().cloned().collect();
            room_browser.update_rooms(&rooms_vec);
        }

        // if connection_failed {
        //     app_state.error_message = Some("Could not connect to matchmaking server.".to_string());
        //     app_state.menu_state = MenuState::MatchmakingConnectionError;
        //     app_state.matchmaking_client = None;
        // }

        next_frame().await;
    }
}


async fn process_matchmaking_server_message(msg: &str, app_state: &mut AppState) {
    match serde_json::from_str::<MatchmakingServerToClient>(msg) {
        Ok(MatchmakingServerToClient::RoomDirectory(room_dir)) => {
            #[cfg(not(target_arch = "wasm32"))]
            println!("Room dir received: {} rooms", room_dir.len());

            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&format!("Room dir received: {} rooms", room_dir.len()).into());

            app_state.rooms = room_dir; 
        },
        Ok(MatchmakingServerToClient::RoomCreated { room_id }) => {
            //Room has been created, now get the info and join it
            if let Some(client) = &app_state.matchmaking_client { 
                client.send_text(&serde_json::to_string(&ClientToMatchmakingServer::GetRoomInfo { room_id: room_id }).unwrap());
            }
        },
        Ok(MatchmakingServerToClient::RoomInfo { room_id, server_address }) => {
            //Room info recieved, now join the room - in the future we will actually join it but for now, just pretend
            if let Some(client) = &app_state.matchmaking_client { 
                client.send_text(&serde_json::to_string(&ClientToMatchmakingServer::JoinedRoom { room_id: room_id, player_id: app_state.player_id.clone() }).unwrap());
            }
            app_state.menu_state = MenuState::InRoom

        }
        Ok(MatchmakingServerToClient::Error(err)) => {
            app_state.error_message = Some(format!("Server error: {}", err));
            app_state.menu_state = MenuState::MatchmakingConnectionError;
        }
        _ => {}
    }
}



