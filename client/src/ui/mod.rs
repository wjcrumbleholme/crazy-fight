use crate::menu::{MenuState};


pub mod cardui;
pub mod container;
pub mod label;
pub mod button;
pub mod slider;
pub mod textbox;
pub mod checkbox;

// Align along an axis
pub enum Alignment {
    Centre,
    LeTop,
    RiBot,
}

// Position the object relative to the parent object
pub enum Position {
    Abs(f32),
    Rel(f32), // Percentage of the parent
    Align(Alignment)
}

impl Position {
    fn coord(&self, parent_offset: f32, parent_size: f32, own_size: f32) -> f32 {
        match self {
            Position::Abs(val) => parent_offset + val,
            Position::Rel(percent) => parent_offset + (parent_size * percent),
            Position::Align(alignment) => match alignment {
                Alignment::Centre => parent_offset + (parent_size / 2.0) - (own_size / 2.0),
                Alignment::LeTop => parent_offset,
                Alignment::RiBot => parent_offset + parent_size - own_size,
            }
        }

    }
}

pub struct Padding {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

impl Padding {
    pub fn uniform(value: f32) -> Self {
        Self {
            top: value,
            bottom: value,
            left: value,
            right: value,
        }
    }

    pub fn zero() -> Self {
        Self::uniform(0.0)
    }

    pub fn new(top: f32, bottom: f32, left: f32, right: f32) -> Self {
        Self {
            top: top,
            bottom: bottom,
            left: left,
            right: right,
        }
    }

}

// Size of the object
pub enum Size {
    Abs(f32),
    Rel(f32),
}

impl Size {
    fn caclulate(&self, parent_size: f32) -> f32 {
        match self {
            Size::Abs(abs_size) => *abs_size,
            Size::Rel(rel_size) => parent_size * rel_size,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Margin {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl Margin {
    pub fn zero() -> Self {
        Self { top: 0.0, bottom: 0.0, left: 0.0, right: 0.0 }
    }

    pub fn uniform(val: f32) -> Self {
        Self { top: val, bottom: val, left: val, right: val }
    }

    pub fn new(top: f32, bottom: f32, left: f32, right: f32) -> Self {
        Self { top, bottom, left, right }
    }
}



// Layout - similar to flex
pub enum Layout {
    None,
    RowCentre,
    ColumnCentre,
}

// Define what all UIElements need to have 
pub trait UIElement {
    fn draw(&mut self, ctx: &mut UIContext, parent_x: f32, parent_y: f32, parent_w: f32, parent_h: f32);
    fn get_width(&self, parent_w: f32) -> f32;
    fn get_height(&self, parent_h: f32) -> f32;

    fn get_margin(&self) -> Margin {
        Margin::zero()
    }
}




pub struct UIContext {
    pub message_queue: Vec<UIMessage>
}

impl UIContext {
    pub fn new() -> Self {
        Self { message_queue: vec![] }
    }
}


#[derive(Clone)]
pub enum UIMessage {
    DrawCard(uuid::Uuid),
    SwitchView(MenuState),
    TryConnectToMatchmaking,
    CreateRoom,
    JoinRoom(uuid::Uuid),
}


