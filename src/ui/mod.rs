use macroquad::{color::{Color}, shapes::draw_rectangle};


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

//Basic container object
pub struct Container {
    children: Vec<Box<dyn UIElement>>,
    x: Position,
    y: Position,
    w: Size,
    h: Size,
    color: Color,

}

// Basic container object logic
impl Container {
    pub fn new(x: Position, y: Position, w: Size, h: Size, color: Color) -> Container{
        Container { 
            children: vec![], 
            x: x,
            y: y,
            w: w,
            h: h,
            color: color
        }
    }

    pub fn add_child(&mut self, child: Box<dyn UIElement>) {
        self.children.push(child);
    }
}

// Define what all UIElements need to have 
pub trait UIElement {
    fn draw(&self, parent_x: f32, parent_y: f32, parent_w: f32, parent_h: f32);
}

// Implement the trait for the container object
impl UIElement for Container {
    fn draw(&self, parent_x: f32, parent_y: f32, parent_w: f32, parent_h: f32) {
        // Calculate the size
        let w = self.w.caclulate(parent_w);
        let h = self.h.caclulate(parent_h);
        // Calculate the x and y coords
        let x = self.x.coord(parent_x, parent_w, w);
        let y = self.y.coord(parent_y, parent_h, h);

        draw_rectangle(x, y, w, h, self.color);

        // Recursively draw the child's objects
        for child in &self.children {
            child.draw(x, y, w, h);
        }
    }
}




