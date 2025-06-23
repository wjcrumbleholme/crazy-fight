use macroquad::prelude::*;
use std::{cell::RefCell, rc::Rc, time::Instant};

use crate::ui::{Position, Size, UIContext, UIElement};

pub struct TextBox {
    text: String,
    is_focused: bool,
    cursor_visible: bool,
    last_cursor_blink: Instant,
    x: Position,
    y: Position,
    w: Size,
    h: Size,
    bg_color: Color,
    border_color: Color,
    text_color: Color,
    padding: f32,
}

impl TextBox {
    pub fn new(x: Position, y: Position, w: Size, h: Size, bg_color: Color, border_color: Color, text_color: Color) -> Self {
        Self {
            text: String::new(),
            is_focused: false,
            cursor_visible: true,
            last_cursor_blink: Instant::now(),
            x,
            y,
            w,
            h,
            bg_color: bg_color,
            border_color: border_color,
            text_color: text_color,
            padding: 5.0,
        }
    }
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl UIElement for TextBox {
    fn draw(&mut self, _ctx: &mut UIContext, parent_x: f32, parent_y: f32, parent_w: f32, parent_h: f32) {
        let w = self.w.caclulate(parent_w);
        let h = self.h.caclulate(parent_h);
        let x = self.x.coord(parent_x, parent_w, w);
        let y = self.y.coord(parent_y, parent_h, h);

        // Handle mouse focus
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            self.is_focused = mx >= x && mx <= x + w && my >= y && my <= y + h;
        }

        // Draw background
        draw_rectangle(x, y, w, h, self.bg_color);
        draw_rectangle_lines(x, y, w, h, 2.0, self.border_color);

        // Draw text
        let font_size = 20.0;
        let text_x = x + self.padding;
        let text_y = y + h / 2.0 - font_size / 2.0;
        draw_text(&self.text, text_x, text_y + font_size, font_size, self.text_color);

        // Cursor blinking
        if self.is_focused {
            if self.last_cursor_blink.elapsed().as_secs_f32() > 0.5 {
                self.cursor_visible = !self.cursor_visible;
                self.last_cursor_blink = Instant::now();
            }

            if self.cursor_visible {
                let text_width = measure_text(&self.text, None, font_size as u16, 1.0).width;
                draw_line(
                    text_x + text_width + 2.0,
                    text_y,
                    text_x + text_width + 2.0,
                    text_y + font_size,
                    2.0,
                    self.text_color,
                );
            }

            // Handle text input
            if let Some(c) = get_char_pressed() {
                if c == '\u{8}' {
                    self.text.pop(); // backspace
                } else if !c.is_control() {
                    self.text.push(c);
                }
            }
        }
    }

    fn get_width(&self, parent_w: f32) -> f32 {
        self.w.caclulate(parent_w)
    }

    fn get_height(&self, parent_h: f32) -> f32 {
        self.h.caclulate(parent_h)
    }
}

pub struct TextBoxWrapper(pub Rc<RefCell<crate::ui::textbox::TextBox>>);

impl UIElement for TextBoxWrapper {
    fn draw(&mut self, ctx: &mut UIContext, parent_x: f32, parent_y: f32, parent_w: f32, parent_h: f32) {
        self.0.borrow_mut().draw(ctx, parent_x, parent_y, parent_w, parent_h);
    }

    fn get_width(&self, parent_w: f32) -> f32 {
        self.0.borrow().get_width(parent_w)
    }

    fn get_height(&self, parent_h: f32) -> f32 {
        self.0.borrow().get_height(parent_h)
    }
}