use macroquad::{color::Color, text::{draw_text, measure_text}};

use super::{Position, Size, UIElement};

// Label
pub struct Label {
    text: String,
    x: Position,
    y: Position,
    w: Size,
    h: Size,
    font_size: u16,
    color: Color
}

// Label logic
impl Label {
    pub fn new(x: Position, y: Position, font_size: u16, text: String, color: Color) -> Self {
        let text_dim = measure_text(&text, None, font_size, 1.0);
        let w = text_dim.width;
        let h = text_dim.height;
        Self { 
            text: text, 
            x: x, 
            y: y, 
            w: Size::Abs(w),
            h: Size::Abs(h),
            font_size: font_size, 
            color: color,
        }
    }
}

//Implement the trait for the Label object
impl UIElement for Label {
    fn draw(&self, parent_x: f32, parent_y: f32, parent_w: f32, parent_h: f32) {
        
        let x = self.x.coord(parent_x, parent_w, self.w.caclulate(parent_w));
        let y = self.y.coord(parent_y, parent_h, self.h.caclulate(parent_h));

        draw_text(&self.text, x, y + self.h.caclulate(parent_h), self.font_size as f32, self.color);
    }

    fn get_width(&self, parent_w: f32) -> f32 {
        self.w.caclulate(parent_w)
    }
    fn get_height(&self, parent_h: f32) -> f32 {
        self.h.caclulate(parent_h)
    }
}