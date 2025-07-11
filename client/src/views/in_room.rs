use std::{cell::{RefCell}, rc::Rc};

use crate::{ui::{self, button::Button, container::{Container, RefCellContainerWrapper}, label::Label, Alignment, Position, Size, UIMessage}, views::MenuState};

use macroquad::prelude::*;


pub struct InRoom {
    pub container: Rc<RefCell<Container>>,
    pub player_container: Rc<RefCell<Container>>, // For the players that are currently in the room
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
            Size::Rel(0.3),
            Size::Rel(0.1),
            LIME,
            BLACK,
            "Back to Main menu".to_string(),
            24,
            Some(UIMessage::SwitchView(MenuState::MainMenu)),
        );

        let player_container = Rc::new(RefCell::new(Container::new(
            Position::Rel(0.025),
            Position::Align(Alignment::Centre),
            Size::Rel(0.3),
            Size::Rel(0.7),
            BEIGE,
            ui::Layout::ColumnTop,
            Size::Rel(0.05),
        )));

        let player_container_label = Label::new(
            Position::Align(Alignment::Centre),
            Position::Align(Alignment::LeTop), 
            32, 
            "Players".to_string(), 
            BLACK
        );


        player_container.borrow_mut().add_child(Box::new(player_container_label));
        root.add_child(Box::new(RefCellContainerWrapper(Rc::clone(&player_container))));
        root.add_child(Box::new(back_btn));

        Self {
            container: Rc::new(RefCell::new(root)),
            player_container,
        }

    }
}