
use std::{clone, collections::HashMap};

use common::server::{messages::{ClientToMatchmakingServer, ClientToServer, MatchmakingServerToClient, ServerToClient}, room_info::RoomInfo};
use macroquad::prelude::*;
use uuid::Uuid;
mod net;
mod ui;

use net::WsMessage;

use crate::{net::{platform, ConnectionResult, WebSocketClient}, ui::{label::Label, Alignment, Position, UIContext, UIElement, UIMessage}, views::{connection_error::ConnectionError, direct_connect::DirectConnect, in_room::InRoom, main_menu::MainMenu, room_browser::RoomBrowser, MenuState}};

mod views;


pub struct AppState {
    pub menu_state: MenuState,
    pub matchmaking_client: Option<Box<dyn WebSocketClient>>,
    pub game_server_client: Option<Box<dyn WebSocketClient>>,
    pub rooms: HashMap<Uuid, RoomInfo>,
    pub error_message: Option<String>,
    pub player_id: Uuid,
    pub player_name: String,
    pub other_players: HashMap<Uuid, String>
}

#[macroquad::main("Client")]
async fn main() {
    let mut app_state = AppState {
        menu_state: MenuState::MainMenu,
        matchmaking_client: None,
        game_server_client: None,
        error_message: None,
        rooms: HashMap::new(),
        player_id: Uuid::new_v4(),
        player_name: "NO NAME".to_string(),
        other_players: HashMap::new(),
    };

    let mut ctx = UIContext::new();

    let mut main_menu = MainMenu::new();
    let mut room_browser = RoomBrowser::new();
    let mut direct_connect = DirectConnect::new();
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
                 // Also try and disconnect the client from any game server
                if let Some(gs_client) = app_state.game_server_client.as_ref() {
                    gs_client.send_text(&serde_json::to_string(&ClientToServer::Disconnect).unwrap());
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
            MenuState::ConnectionError(ref error) => {
                let connection_error = ConnectionError::new(error.clone());
                connection_error.container.borrow_mut().draw(&mut ctx, 0.0, 0.0, screen_width(), screen_height());
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
                            app_state.error_message = Some(err.clone());
                            app_state.menu_state = MenuState::ConnectionError(err);
                        }
                    }
                },
                UIMessage::CreateRoom => {
                    if let Some(client) = &app_state.matchmaking_client {

                        let room_name = {
                            let name = room_browser.room_name_text_box.borrow().get_text();
                            if name.trim().is_empty() {
                                "Room with no name".to_string()
                            } else {
                                name
                            }
                        };

                        let max_players = { room_browser.max_player_slider.borrow().get_value() as usize };

                        let is_private = {room_browser.private_checkbox.borrow().is_checked()};

                        client.send_text(&serde_json::to_string(&ClientToMatchmakingServer::CreateRoom {room_name, is_private , max_players}).unwrap());
                    }
                },
                UIMessage::JoinRoom(room_id) => {
                    if let Some(client) = &app_state.matchmaking_client { 
                        client.send_text(&serde_json::to_string(&ClientToMatchmakingServer::GetRoomInfo { room_id: room_id }).unwrap());
                    }
                },
            }
        }

        // Handle incoming messages
        // Extract messages first to avoid borrowing app_state immutably and mutably at the same time
        let mut mm_messages: Vec<String> = Vec::new();

        if let Some(client) = &app_state.matchmaking_client {
            while let Some(WsMessage::Text(text)) = client.try_recv() {
                mm_messages.push(text);
            }
        }

        for text in mm_messages {
            process_matchmaking_server_message(&text, &mut app_state, &room_browser, &in_room).await;
            let rooms_vec: Vec<RoomInfo> = app_state.rooms.values().cloned().collect();
            room_browser.update_rooms(&rooms_vec);
        }

        let mut gs_messages: Vec<String> = Vec::new();

        if let Some(client) = &app_state.game_server_client {
            while let Some(WsMessage::Text(text)) = client.try_recv() {
                gs_messages.push(text)
            }
        }

        for text in gs_messages {
            process_game_server_message(&text, &mut app_state, &in_room).await;
        }

        next_frame().await;
    }
}


async fn process_game_server_message(msg: &str, app_state: &mut AppState, in_room: &InRoom) {
    match serde_json::from_str::<ServerToClient>(msg) {
        // Instead of doing it this way, just request the player list from the server
        Ok(ServerToClient::PlayerJoined { player_id, player_name }) => {
            // First check if the player id is not the current one
            if app_state.player_id != player_id {
                // The id's are different so act on that info
                // Check if the menu state is in the in_room state, and if it is then add the player to the current list
                if app_state.menu_state == MenuState::InRoom {

                    let player_label_to_add = Label::new(
                        Position::Align(Alignment::Centre),
                        Position::Align(Alignment::LeTop), 
                        24, 
                        player_name.clone(), 
                        BLACK
                    );

                    in_room.player_container.borrow_mut().add_child(Box::new(player_label_to_add));
                }

                // Add the player to the other players hashmap
                app_state.other_players.insert(player_id, player_name);
            }
            // The player id is the current player id so do nothing
        },
        _ => {},
    }
}



async fn process_matchmaking_server_message(msg: &str, app_state: &mut AppState, room_browser: &RoomBrowser, in_room: &InRoom) {
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
                println!("Getting room info");
                client.send_text(&serde_json::to_string(&ClientToMatchmakingServer::GetRoomInfo { room_id: room_id }).unwrap());
            }
        },
        Ok(MatchmakingServerToClient::RoomInfo {server_address }) => {
            //Room info recieved, now try join the room 
            println!("Room info recieved - trying to connect to: {}", server_address);
            match platform::connect(&server_address).await {
                ConnectionResult::Success(client) => {
                    // Connected to the game server
                    println!("Connected to game server");
                    //Get the players name
                    let player_name = {
                            let name = room_browser.player_name_text_box.borrow().get_text();
                            if name.trim().is_empty() {
                                "NO NAME".to_string()
                            } else {
                                name
                            }
                        };
                    app_state.player_name = player_name.clone();


                    client.send_text(&serde_json::to_string(&ClientToServer::RegisterPlayer { player_name: player_name.clone(), player_id: app_state.player_id.clone() }).unwrap());
                    app_state.game_server_client = Some(client);
                    app_state.menu_state = MenuState::InRoom;
                    // Add the player to the list
                    let player_label_to_add = Label::new(
                        Position::Align(Alignment::Centre),
                        Position::Align(Alignment::LeTop), 
                        24, 
                        format!("{player_name} (self)"), 
                        BLACK
                    );

                    in_room.player_container.borrow_mut().add_child(Box::new(player_label_to_add));
                },
                ConnectionResult::Failure(err) => {
                    println!("Cannot connect to the game server");
                    app_state.error_message = Some(err.clone());
                    app_state.menu_state = MenuState::ConnectionError(err);
                }
            }
            
        }
        Ok(MatchmakingServerToClient::Error(err)) => {
            app_state.error_message = Some(format!("Server error: {}", err));
            app_state.menu_state = MenuState::ConnectionError(format!("Server error: {}", err));
        }
        _ => {}
    }
}



