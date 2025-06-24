use std::{cell::RefCell, rc::Rc};

use crate::{ui::{self, button::Button, container::Container, label::Label, Alignment, Position, Size, UIMessage}, views::MenuState};

use macroquad::prelude::*;

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