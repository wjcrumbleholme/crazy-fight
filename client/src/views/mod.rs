pub mod main_menu;
pub mod room_browser;
pub mod direct_connect;
pub mod connection_error;
pub mod in_room;

#[derive(Clone)]
pub enum MenuState {
    MainMenu,
    RoomBrowser,
    DirectConnect,
    InGame,
    ConnectionError(String),
    InRoom,
}








