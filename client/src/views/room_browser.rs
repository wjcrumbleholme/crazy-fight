use std::{cell::RefCell, rc::Rc};

use crate::{ui::{self, button::Button, checkbox::{Checkbox, CheckboxWrapper}, container::{Container, RefCellContainerWrapper}, label::Label, slider::{Slider, SliderWrapper}, textbox::{TextBox, TextBoxWrapper}, Alignment, Padding, Position, Size, UIMessage}, views::MenuState};

use common::server::room_info::RoomInfo;
use macroquad::prelude::*;


pub struct RoomBrowser {
    pub container: Rc<RefCell<Container>>, // For draw() and input
    pub room_container: Rc<RefCell<Container>>, // For modifying the room list
    pub room_name_text_box: Rc<RefCell<TextBox>>,
    pub max_player_slider: Rc<RefCell<Slider>>,
    pub private_checkbox: Rc<RefCell<Checkbox>>,
    pub player_name_text_box: Rc<RefCell<TextBox>>,
}

impl RoomBrowser {
    pub fn new() -> Self {
        let root =  Rc::new(RefCell::new(Container::new(
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
            Size::Rel(0.3),
            Size::Rel(0.1),
            LIME,
            BLACK,
            "Back to Main menu".to_string(),
            24,
            Some(UIMessage::SwitchView(MenuState::MainMenu)),
        );

        let mut left_bar = Container::new(
            Position::Align(Alignment::LeTop), 
            Position::Align(Alignment::Centre), 
            Size::Rel(0.3), 
            Size::Rel(0.8),
            PURPLE,
            ui::Layout::None,
            Size::Rel(0.05),
        );

        let mut player_fields = Container::new(
            Position::Align(Alignment::Centre), 
            Position::Rel(0.05), 
            Size::Rel(0.95), 
            Size::Rel(0.2),
            RED,
            ui::Layout::ColumnCentre,
            Size::Abs(10.0),
        );

        let player_name_label = Label::new(
            Position::Align(Alignment::Centre),
            Position::Align(Alignment::Centre),
            24, 
            "Player Name:".to_string(), 
            BLACK
        );

        let player_name_text_box =  Rc::new(RefCell::new(TextBox::new(
            Position::Align(Alignment::Centre), 
            Position::Align(Alignment::Centre), 
            Size::Rel(0.9), 
            Size::Abs(40.0), 
            WHITE, 
            BLACK, 
            BLACK
        )));

        let mut room_fields = Container::new(
            Position::Align(Alignment::Centre), 
            Position::Rel(0.30), 
            Size::Rel(0.95), 
            Size::Rel(0.65),
            GREEN,
            ui::Layout::ColumnCentre,
            Size::Abs(10.0),
        );

        let mut room_name_container = Container::new(
            Position::Align(Alignment::Centre), 
            Position::Align(Alignment::RiBot), 
            Size::Rel(1.0), 
            Size::Abs(75.0),
            PINK,
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
            GOLD,
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
            BROWN,
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

        player_fields.add_child(Box::new(player_name_label));
        player_fields.add_child(Box::new(TextBoxWrapper(player_name_text_box.clone())));

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
            private_checkbox,
            player_name_text_box,
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
                Size::Rel(0.85),
                Size::Abs(100.0),
                WHITE,
                ui::Layout::None,
                Size::Rel(0.2),
            );
            individual_room.add_padding(Padding::uniform(20.0));
            let player_count = Label::new(
                Position::Align(Alignment::Centre),
                Position::Align(Alignment::Centre),
                32, 
                format!("{}/{}", room.player_count.clone(), room.max_players.clone()), 
                BLACK
            );
            let room_name = Label::new(
                Position::Align(Alignment::LeTop),
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
            );

            individual_room.add_child(Box::new(room_name));
            individual_room.add_child(Box::new(player_count));
            individual_room.add_child(Box::new(button));
            
            container.add_child(Box::new(individual_room));
        }
    }
}

