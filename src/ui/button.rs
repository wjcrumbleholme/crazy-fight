use macroquad::{color::Color, input::{is_mouse_button_pressed, mouse_position, MouseButton}, shapes::draw_rectangle};

use super::{label::Label, Alignment, Position, Size, UIElement};
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
    fn get_width(&self, parent_w: f32) -> f32 {
        self.w.caclulate(parent_w)
    }
    fn get_height(&self, parent_h: f32) -> f32 {
        self.h.caclulate(parent_h)
    }
}
