
use common::server::messages::{ClientToMatchmakingServer, MatchmakingServerToClient};
use macroquad::prelude::*;
mod net;
mod ui;
use crate::{menu::{DirectConnect, MainMenu, MenuState, RoomBrowser}, net::ws, ui::{container::Container, Alignment, Position, Size, UIContext, UIElement, UIMessage}};
use net::WsMessage;

mod menu;

#[macroquad::main("Client")]
async fn main() {
    let mut ctx = UIContext::new();
    let mut menu_state = MenuState::MainMenu;
    let mut main_menu = MainMenu::new();
    let mut room_browser = RoomBrowser::new();
    let mut direct_connect = DirectConnect::new();

    loop {
        clear_background(BLACK);

        match menu_state {
            MenuState::MainMenu => {
                main_menu.container.borrow_mut().draw(&mut ctx, 0.0, 0.0, screen_width(), screen_height());
            },
            MenuState::RoomBrowser => {
                room_browser.container.borrow_mut().draw(&mut ctx, 0.0, 0.0, screen_width(), screen_height());
            },
            MenuState::DirectConnect => {
                direct_connect.container.borrow_mut().draw(&mut ctx, 0.0, 0.0, screen_width(), screen_height());
            },
            MenuState::InGame => {

            }
        }


        for msg in ctx.message_queue.drain(..) {
            match msg {
                UIMessage::SwitchView(menu_to_switch_to) => {
                    match menu_to_switch_to {
                        MenuState::MainMenu => menu_state = MenuState::MainMenu,
                        MenuState::DirectConnect => menu_state = MenuState::DirectConnect,
                        MenuState::InGame => menu_state = MenuState::InGame,
                        MenuState::RoomBrowser => menu_state = MenuState::RoomBrowser,
                    }
                },
                UIMessage::DrawCard(card_id) => {

                }
            }
        }




        next_frame().await;
    }

}


async fn process_matchmaking_server_message(msg: String) {
    match serde_json::from_str::<MatchmakingServerToClient>(&msg) {
        Ok(MatchmakingServerToClient::RoomDirectory(room_dir)) => {
            #[cfg(not(target_arch = "wasm32"))]
            println!("Room dir received");
            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&format!("Room dir received").into());

        },
        _=> (),
    }
}
