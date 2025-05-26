use macroquad::{color::Color, input::{is_mouse_button_pressed, mouse_position, MouseButton}, shapes::draw_rectangle, text::{draw_text, measure_text}};


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

//Basic container object
pub struct Container {
    children: Vec<Box<dyn UIElement>>,
    x: Position,
    y: Position,
    w: Size,
    h: Size,
    color: Color,
    padding: Padding

}

// Basic container object logic
impl Container {
    pub fn new(x: Position, y: Position, w: Size, h: Size, color: Color) -> Self{
        Self { 
            children: vec![], 
            x: x,
            y: y,
            w: w,
            h: h,
            color: color,
            padding: Padding::zero()
        }
    }

    /// When calling, wrap the object in a box Box::new()
    pub fn add_child(&mut self, child: Box<dyn UIElement>) {
        self.children.push(child);
    }

    /// Adds padding to a container
    pub fn add_padding(&mut self, padding:Padding) {
        self.padding = padding
    }
}


// Label
pub struct Label {
    text: String,
    x: Position,
    y: Position,
    font_size: u16,
    color: Color
}

// Label logic
impl Label {
    pub fn new(x: Position, y: Position, font_size: u16, text: String, color: Color) -> Self {
        Self { 
            text: text, 
            x: x, 
            y: y, 
            font_size: font_size, 
            color: color,
        }
    }
}

// Button
pub struct Button {
    label: Label,
    x: Position,
    y: Position,
    w: Size,
    h: Size, 
    bg_color: Color,
    on_click: Box<dyn Fn()>,
}

impl Button {
    pub fn new<F: Fn() + 'static>(x: Position, y: Position, w: Size, h: Size, bg_color: Color, txt_color: Color, txt_content:String, txt_size: u16, on_click: F) -> Self {
        let label = Label::new(
            Position::Align(Alignment::Centre), 
            Position::Align(Alignment::Centre), 
            txt_size, 
            txt_content, 
            txt_color,
        );
        Self { 
            label: label, 
            x: x, 
            y: y, 
            w: w, 
            h: h, 
            bg_color: bg_color, 
            on_click: Box::new(on_click), 
        }
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

        let content_x = x + self.padding.left;
        let content_y = y + self.padding.top;
        let content_w = w - self.padding.left - self.padding.right;
        let content_h = h - self.padding.left - self.padding.right;

        // Recursively draw the child's objects
        for child in &self.children {
            child.draw(content_x, content_y, content_w, content_h);
        }
    }
}

//Implement the trait for the Label object
impl UIElement for Label {
    fn draw(&self, parent_x: f32, parent_y: f32, parent_w: f32, parent_h: f32) {
        let text_dim = measure_text(&self.text, None, self.font_size, 1.0);
        let w = text_dim.width;
        let h = text_dim.height;

        let x = self.x.coord(parent_x, parent_w, w);
        let y = self.y.coord(parent_y, parent_h, h);

        draw_text(&self.text, x, y + h, self.font_size as f32, self.color);
    }
}

//Implement the trait for the Button object
impl UIElement for Button {
    fn draw(&self, parent_x: f32, parent_y: f32, parent_w: f32, parent_h: f32) {
        let w = self.w.caclulate(parent_w);
        let h = self.h.caclulate(parent_h);
        let x = self.x.coord(parent_x, parent_w, w);
        let y = self.y.coord(parent_y, parent_h, h);

        // Check if clicked
        if is_mouse_button_pressed(MouseButton::Left) {
            let(mouse_x, mouse_y) = mouse_position();
            if mouse_x >= x && mouse_x <= (x + w) && mouse_y >= y && mouse_y <= (y+h) {
                (self.on_click)();
            }
        }

        // Draw background
        draw_rectangle(x, y, w, h, self.bg_color);

        // Draw label in button
        self.label.draw(x, y, w, h);

    }
}




