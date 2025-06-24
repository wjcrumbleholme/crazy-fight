use std::{cell::RefCell, rc::Rc};

use crate::{ui::{self, button::Button, container::Container, Alignment, Position, Size, UIMessage}, views::MenuState};

use macroquad::prelude::*;

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