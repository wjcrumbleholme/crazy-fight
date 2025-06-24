use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;
use super::{UIContext, UIElement, Position, Size, Alignment, UIMessage, Padding};

pub struct Checkbox {
    x: Position,
    y: Position,
    w: Size,
    h: Size,
    checked: bool,
    box_color: Color,
    check_color: Color,
}

impl Checkbox {
    pub fn new(
        x: Position,
        y: Position,
        w: Size,
        h: Size,
        box_color: Color,
        check_color: Color,
    ) -> Self {
        Self {
            x,
            y,
            w,
            h,
            checked: false,
            box_color,
            check_color,
        }
    }
    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn set_checked(&mut self, state: bool) {
        self.checked = state;
    }
}

impl UIElement for Checkbox {
    fn draw(&mut self, ctx: &mut UIContext, parent_x: f32, parent_y: f32, parent_w: f32, parent_h: f32) {
        let w = self.w.caclulate(parent_w);
        let h = self.h.caclulate(parent_h);
        let x = self.x.coord(parent_x, parent_w, w);
        let y = self.y.coord(parent_y, parent_h, h);
        let box_size = h.min(w);

        // Handle click
        let (mx, my) = mouse_position();
        let clicked = is_mouse_button_pressed(MouseButton::Left)
            && mx >= x && mx <= x + box_size
            && my >= y && my <= y + box_size;

        if clicked {
            self.checked = !self.checked;
        }

        // Draw checkbox square
        draw_rectangle(x, y, box_size, box_size, self.box_color);

        // Draw checkmark
        if self.checked {
            let inset = box_size * 0.2;
            draw_line(x + inset, y + inset, x + box_size - inset, y + box_size - inset, 2.0, self.check_color);
            draw_line(x + inset, y + box_size - inset, x + box_size - inset, y + inset, 2.0, self.check_color);
        }

    }
    fn get_width(&self, parent_w: f32) -> f32 {
        self.w.caclulate(parent_w)
    }

    fn get_height(&self, parent_h: f32) -> f32 {
        self.h.caclulate(parent_h)
    }
    
}

pub struct CheckboxWrapper(pub Rc<RefCell<Checkbox>>);

impl UIElement for CheckboxWrapper {
    fn draw(&mut self, ctx: &mut UIContext, parent_x: f32, parent_y: f32, parent_w: f32, parent_h: f32) {
        {
            let mut checkbox = self.0.borrow_mut();
            checkbox.draw(ctx, parent_x, parent_y, parent_w, parent_h);
        }
    }

    fn get_width(&self, parent_w: f32) -> f32 {
        self.0.borrow().get_width(parent_w)
    }

    fn get_height(&self, parent_h: f32) -> f32 {
        self.0.borrow().get_height(parent_h)
    }
}