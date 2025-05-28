use macroquad::{color::WHITE, input::{is_mouse_button_pressed, mouse_position, MouseButton}, math::vec2, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

use super::{Position, Size, UIElement};

pub struct CardUi {
    img: Texture2D,
    x: Position,
    y: Position,
    w: Size,
    h: Size, 
    on_click: Box<dyn Fn()>,
}

impl CardUi {
    pub fn new<F: Fn() + 'static>(x: Position, y: Position, img: Texture2D, on_click: F) -> Self { 
        Self { 
            img: img, 
            x: x, 
            y: y, 
            w: Size::Abs(160.0), 
            h: Size::Abs(224.0), 
            on_click: Box::new(on_click), 
        }
    }
}

//Implement the trait for the Card object
impl UIElement for CardUi {
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
        draw_texture_ex(&self.img, x, y, WHITE, 
                    DrawTextureParams {
                        dest_size: Some(vec2(w, h)),
                        ..Default::default()
                    }
            );

        

    }
    fn get_width(&self, parent_w: f32) -> f32 {
        self.w.caclulate(parent_w)
    }
    fn get_height(&self, parent_h: f32) -> f32 {
        self.h.caclulate(parent_h)
    }
}
