use std::{cell::RefCell, rc::Rc};

use crate::{ui::{self, button::Button, container::Container, Alignment, Position, Size, UIMessage}, views::MenuState};

use macroquad::prelude::*;


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