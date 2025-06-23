use std::{cell::RefCell, fmt::format, rc::Rc};

use crate::ui::{self, button::{self, Button}, container::{Container, RefCellContainerWrapper}, label::Label, Alignment, Position, Size, UIMessage};
use common::server::room_info::RoomInfo;
use macroquad::prelude::*;


#[derive(Clone)]
pub enum MenuState {
    MainMenu,
    RoomBrowser,
    DirectConnect,
    InGame,
    ConnectionError(String),
    InRoom,
}

pub struct MainMenu {
    pub container: Rc<RefCell<Container>>,
}

impl MainMenu {
    pub fn new() -> Self {
        let mut root = Container::new(
            Position::Align(Alignment::LeTop), 
            Position::Align(Alignment::LeTop), 
            Size::Rel(1.0), 
            Size::Rel(1.0),
            GREEN,
            ui::Layout::ColumnCentre,
            Size::Rel(0.05),
        );

        let room_browser_btn = Button::new(
            Position::Align(Alignment::Centre), 
            Position::Align(Alignment::Centre), 
            Size::Abs(400.0), 
            Size::Abs(100.0), 
            LIME, 
            BLACK, 
            "Room Browser".to_string(), 
            32, 
            Some(UIMessage::TryConnectToMatchmaking),
        );

        let direct_connect_btn = Button::new(
            Position::Align(Alignment::Centre), 
            Position::Align(Alignment::Centre), 
            Size::Abs(400.0), 
            Size::Abs(100.0), 
            LIME, 
            BLACK, 
            "Direct Connect".to_string(), 
            32, 
            Some(UIMessage::SwitchView(MenuState::DirectConnect)),
        );

        root.add_child(Box::new(room_browser_btn));
        root.add_child(Box::new(direct_connect_btn));



        Self {
            container: Rc::new(RefCell::new(root))
        }

    }
}


pub struct RoomBrowser {
    pub container: Rc<RefCell<Container>>, // For draw() and input
    pub room_container: Rc<RefCell<Container>>, // For modifying the room list
}

impl RoomBrowser {
    pub fn new() -> Self {
        let mut root =  Rc::new(RefCell::new(Container::new(
            Position::Align(Alignment::LeTop),
            Position::Align(Alignment::LeTop),
            Size::Rel(1.0),
            Size::Rel(1.0),
            BLUE,
            ui::Layout::None,
            Size::Rel(0.1),
        )));

        let room_container = Rc::new(RefCell::new(Container::new(
            Position::Rel(0.3),
            Position::Align(Alignment::Centre),
            Size::Rel(0.6),
            Size::Rel(0.8),
            BEIGE,
            ui::Layout::ColumnCentre,
            Size::Rel(0.05),
        )));

        let back_btn = Button::new(
            Position::Align(Alignment::LeTop),
            Position::Align(Alignment::LeTop),
            Size::Abs(300.0),
            Size::Abs(100.0),
            LIME,
            BLACK,
            "Back to Main menu".to_string(),
            32,
            Some(UIMessage::SwitchView(MenuState::MainMenu)),
        );

        let create_room_btn = Button::new(
            Position::Align(Alignment::LeTop),
            Position::Align(Alignment::RiBot),
            Size::Abs(200.0),
            Size::Abs(100.0),
            LIME,
            BLACK,
            "Create Room".to_string(),
            32,
            Some(UIMessage::CreateRoom),
        );

        root.borrow_mut().add_child(Box::new(back_btn));
        root.borrow_mut().add_child(Box::new(create_room_btn));
        root.borrow_mut().add_child(Box::new(RefCellContainerWrapper(Rc::clone(&room_container))));

        Self {
            container: root,
            room_container,
        }
    }

    pub fn update_rooms(&mut self, rooms: &[RoomInfo]) {
        let mut container = self.room_container.borrow_mut();
        container.clear_children(); // Optional: clear existing buttons

        for room in rooms {
            // Create a room object that has the name, and player count and a join button
            let mut individual_room = Container::new(
                Position::Align(Alignment::Centre), 
                Position::Align(Alignment::Centre), 
                Size::Abs(400.0),
                Size::Abs(100.0),
                WHITE,
                ui::Layout::None,
                Size::Rel(0.05),
            );
            let player_count = Label::new(
                Position::Align(Alignment::Centre),
                Position::Align(Alignment::Centre),
                32, 
                format!("{}/{}", room.player_count.clone(), room.max_players.clone()), 
                BLACK
            );
            let room_name = Label::new(
                Position::Rel(0.1),
                Position::Align(Alignment::Centre),
                32, 
                room.name.clone(), 
                BLACK
            );
            let button = Button::new(
                Position::Rel(0.7),
                Position::Align(Alignment::Centre),
                Size::Abs(100.0),
                Size::Abs(75.0),
                LIME,
                BLACK,
                "Join".to_string(),
                32,
                Some(UIMessage::JoinRoom(room.id)),
            );

            individual_room.add_child(Box::new(button));
            individual_room.add_child(Box::new(player_count));
            individual_room.add_child(Box::new(room_name));
            container.add_child(Box::new(individual_room));
        }
    }
}


pub struct DirectConnect {
    pub container: Rc<RefCell<Container>>,
}

impl DirectConnect {
    pub fn new() -> Self {
        let mut root = Container::new(
            Position::Align(Alignment::LeTop), 
            Position::Align(Alignment::LeTop), 
            Size::Rel(1.0), 
            Size::Rel(1.0),
            RED,
            ui::Layout::None,
            Size::Rel(0.1),
        );

        let back_btn = Button::new(
            Position::Align(Alignment::LeTop), 
            Position::Align(Alignment::LeTop), 
            Size::Abs(300.0), 
            Size::Abs(100.0), 
            LIME, 
            BLACK, 
            "Back to Main menu".to_string(), 
            32, 
            Some(UIMessage::SwitchView(MenuState::MainMenu)),
        );

        root.add_child(Box::new(back_btn));

        Self {
            container: Rc::new(RefCell::new(root))
        }

    }
}

pub struct ConnectionError {
    pub container: Rc<RefCell<Container>>,
}

impl ConnectionError {
    pub fn new(error: String) -> Self {
        let mut root = Container::new(
            Position::Align(Alignment::LeTop), 
            Position::Align(Alignment::LeTop), 
            Size::Rel(1.0), 
            Size::Rel(1.0),
            BEIGE,
            ui::Layout::None,
            Size::Rel(0.1),
        );

        // let error_btn = Button::new(
        //     Position::Align(Alignment::Centre), 
        //     Position::Align(Alignment::Centre), 
        //     Size::Abs(400.0), 
        //     Size::Abs(100.0), 
        //     RED, 
        //     BLACK, 
        //     "Retry Connection".to_string(), 
        //     32, 
        //     Some(UIMessage::TryConnectToMatchmaking),
        // );
        let error_msg = Label::new(
                Position::Align(Alignment::Centre),
                Position::Align(Alignment::Centre),
                32, 
                error, 
                BLACK
            );

        let back_btn = Button::new(
            Position::Align(Alignment::LeTop), 
            Position::Align(Alignment::LeTop), 
            Size::Abs(300.0), 
            Size::Abs(100.0), 
            LIME, 
            BLACK, 
            "Back to Main menu".to_string(), 
            32, 
            Some(UIMessage::SwitchView(MenuState::MainMenu)),
        );

        

        root.add_child(Box::new(back_btn));
        root.add_child(Box::new(error_msg));
        // root.add_child(Box::new(error_btn));

        Self {
            container: Rc::new(RefCell::new(root))
        }

    }
}

pub struct InRoom {
    pub container: Rc<RefCell<Container>>,
}

impl InRoom {
    pub fn new() -> Self {
        let mut root = Container::new(
            Position::Align(Alignment::LeTop), 
            Position::Align(Alignment::LeTop), 
            Size::Rel(1.0), 
            Size::Rel(1.0),
            PURPLE,
            ui::Layout::None,
            Size::Rel(0.1),
        );


        let back_btn = Button::new(
            Position::Align(Alignment::LeTop), 
            Position::Align(Alignment::LeTop), 
            Size::Abs(200.0), 
            Size::Abs(100.0), 
            LIME, 
            BLACK, 
            "Back to Main menu".to_string(), 
            32, 
            Some(UIMessage::SwitchView(MenuState::MainMenu)),
        );

        

        root.add_child(Box::new(back_btn));

        Self {
            container: Rc::new(RefCell::new(root))
        }

    }
}