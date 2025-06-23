use std::{cell::RefCell, fmt::format, rc::Rc};

use crate::ui::{self, button::{self, Button}, checkbox::{Checkbox, CheckboxWrapper}, container::{Container, RefCellContainerWrapper}, label::Label, slider::{Slider, SliderWrapper}, textbox::{TextBox, TextBoxWrapper}, Alignment, Margin, Position, Size, UIMessage};
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
    pub room_name_text_box: Rc<RefCell<TextBox>>,
    pub max_player_slider: Rc<RefCell<Slider>>,
    pub private_checkbox: Rc<RefCell<Checkbox>>
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

        let mut left_bar = Container::new(
            Position::Align(Alignment::LeTop), 
            Position::Align(Alignment::Centre), 
            Size::Rel(0.3), 
            Size::Rel(0.8),
            PURPLE,
            ui::Layout::ColumnCentre,
            Size::Abs(0.0),
        );

        let mut player_fields = Container::new(
            Position::Align(Alignment::Centre), 
            Position::Align(Alignment::Centre), 
            Size::Rel(0.95), 
            Size::Rel(0.3),
            RED,
            ui::Layout::ColumnCentre,
            Size::Abs(10.0),
        );

        let mut room_fields = Container::new(
            Position::Align(Alignment::Centre), 
            Position::Align(Alignment::LeTop), 
            Size::Rel(0.95), 
            Size::Rel(0.8),
            GREEN,
            ui::Layout::ColumnCentre,
            Size::Abs(10.0),
        );

        let mut room_name_container = Container::new(
            Position::Align(Alignment::Centre), 
            Position::Align(Alignment::RiBot), 
            Size::Rel(1.0), 
            Size::Abs(75.0),
            GREEN,
            ui::Layout::ColumnCentre,
            Size::Abs(5.0),
        );

        let room_name_label = Label::new(
            Position::Align(Alignment::Centre),
            Position::Align(Alignment::Centre),
            24, 
            "Room Name:".to_string(), 
            BLACK
        );

        let room_name_text_box =  Rc::new(RefCell::new(TextBox::new(
            Position::Align(Alignment::Centre), 
            Position::Align(Alignment::Centre), 
            Size::Rel(0.9), 
            Size::Abs(40.0), 
            WHITE, 
            BLACK, 
            BLACK
        )));

        let mut max_players_container = Container::new(
            Position::Align(Alignment::Centre), 
            Position::Align(Alignment::Centre), 
            Size::Rel(1.0), 
            Size::Abs(90.0),
            GREEN,
            ui::Layout::ColumnCentre,
            Size::Abs(5.0),
        );
        
        let max_players_label = Label::new(
            Position::Align(Alignment::Centre),
            Position::Align(Alignment::Centre),
            24, 
            "Max Players:".to_string(), 
            BLACK
        );

        let max_player_slider = Rc::new(RefCell::new(Slider::new(
            Position::Align(Alignment::Centre), 
            Position::Align(Alignment::Centre), 
            Size::Rel(0.9), 
            Size::Abs(40.0), 
            WHITE,
            BLACK,
            BLACK,
            2.0,
            12.0,
            1.0,
            true
        )));

        let mut private_container = Container::new(
            Position::Align(Alignment::Centre), 
            Position::Align(Alignment::Centre), 
            Size::Rel(1.0), 
            Size::Abs(40.0),
            GREEN,
            ui::Layout::RowCentre,
            Size::Abs(20.0),
        );

        let private_label = Label::new(
            Position::Align(Alignment::Centre),
            Position::Align(Alignment::Centre),
            24, 
            "Private?".to_string(), 
            BLACK
        );

        let private_checkbox = Rc::new(RefCell::new(Checkbox::new(
            Position::Align(Alignment::Centre),
            Position::Align(Alignment::Centre),
            Size::Abs(20.0), 
            Size::Abs(20.0),
            WHITE,
            PURPLE,
        )));


        let create_room_btn = Button::new(
            Position::Align(Alignment::Centre),
            Position::Align(Alignment::Centre),
            Size::Rel(0.8),
            Size::Abs(50.0),
            LIME,
            BLACK,
            "Create Room".to_string(),
            32,
            Some(UIMessage::CreateRoom),
        );

        room_name_container.add_child(Box::new(room_name_label));
        room_name_container.add_child(Box::new(TextBoxWrapper(room_name_text_box.clone())));
        room_fields.add_child(Box::new(room_name_container));
        max_players_container.add_child(Box::new(max_players_label));
        max_players_container.add_child(Box::new(SliderWrapper(max_player_slider.clone())));
        room_fields.add_child(Box::new(max_players_container));
        private_container.add_child(Box::new(private_label));
        private_container.add_child(Box::new(CheckboxWrapper(private_checkbox.clone())));
        room_fields.add_child(Box::new(private_container));
        room_fields.add_child(Box::new(create_room_btn));

        left_bar.add_child(Box::new(player_fields));
        left_bar.add_child(Box::new(room_fields));
        root.borrow_mut().add_child(Box::new(left_bar));
        root.borrow_mut().add_child(Box::new(back_btn));
        root.borrow_mut().add_child(Box::new(RefCellContainerWrapper(Rc::clone(&room_container))));

        Self {
            container: root,
            room_container,
            room_name_text_box,
            max_player_slider,
            private_checkbox
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
                Size::Rel(0.7),
                Size::Abs(100.0),
                WHITE,
                ui::Layout::None,
                Size::Rel(0.2),
            );
            let player_count = Label::new(
                Position::Rel(0.65),
                Position::Align(Alignment::Centre),
                32, 
                format!("{}/{}", room.player_count.clone(), room.max_players.clone()), 
                BLACK
            );
            let room_name = Label::new(
                Position::Rel(0.05),
                Position::Align(Alignment::Centre),
                32, 
                room.name.clone(), 
                BLACK
            );
            let button = Button::new(
                Position::Align(Alignment::RiBot),
                Position::Align(Alignment::Centre),
                Size::Abs(100.0),
                Size::Abs(75.0),
                LIME,
                BLACK,
                "Join".to_string(),
                32,
                Some(UIMessage::JoinRoom(room.id)),
            ).with_margin(Margin::new(0.0, 0.0, 5.0, 5.0));

            individual_room.add_child(Box::new(room_name));
            individual_room.add_child(Box::new(player_count));
            individual_room.add_child(Box::new(button));
            
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