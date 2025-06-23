use std::{cell::RefCell, rc::Rc};

use crate::ui::{Position, Size, UIContext, UIElement};
use macroquad::prelude::*;

pub struct Slider {
    value: f32,     // current value between min and max
    x: Position,
    y: Position,
    w: Size,
    h: Size,
    dragging: bool,
    bg_color: Color,
    knob_color: Color,
    tick_color: Color,
    min: f32,
    max: f32,
    step: f32,
    show_ticks: bool,
    label_every: u32,
    major_tick_height: f32,
    minor_tick_height: f32,
}

impl Slider {
    pub fn new(
        x: Position,
        y: Position,
        w: Size,
        h: Size,
        bg_color: Color,
        knob_color: Color,
        tick_color: Color,
        min: f32,
        max: f32,
        step: f32,
        show_ticks: bool,
    ) -> Self {
        Self {
            value: min,
            x,
            y,
            w,
            h,
            dragging: false,
            bg_color,
            knob_color,
            tick_color,
            min,
            max,
            step,
            show_ticks,
            label_every: 3,
            major_tick_height: 6.0,
            minor_tick_height: 3.0,
        }
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, value: f32) {
        let stepped = ((value - self.min) / self.step).round() * self.step + self.min;
        self.value = stepped.clamp(self.min, self.max);
    }
}

impl UIElement for Slider {
    fn draw(&mut self, ctx: &mut UIContext, parent_x: f32, parent_y: f32, parent_w: f32, parent_h: f32) {
        let w = self.w.caclulate(parent_w);
        let total_h = self.h.caclulate(parent_h);
        let x = self.x.coord(parent_x, parent_w, w);
        let y = self.y.coord(parent_y, parent_h, total_h);

        // Layout: 2/3 for slider, 1/3 for ticks + labels
        let slider_h = total_h * 2.0 / 3.0;
        let tick_area_h = total_h / 3.0;

        let track_y = y + slider_h / 2.0 - 2.0;
        let knob_y = y;

        // Interaction
        let (mx, my) = mouse_position();
        let mouse_over = mx >= x && mx <= x + w && my >= y && my <= y + total_h;

        if is_mouse_button_pressed(MouseButton::Left) && mouse_over {
            self.dragging = true;
        } else if is_mouse_button_released(MouseButton::Left) {
            self.dragging = false;
        }

        if self.dragging {
            let rel_pos = ((mx - x) / w).clamp(0.0, 1.0);
            let new_val = self.min + rel_pos * (self.max - self.min);
            self.set_value(new_val);
        }


        // Draw track
        draw_rectangle(x, track_y, w, 4.0, self.bg_color);

        // Draw knob
        let rel_value = (self.value - self.min) / (self.max - self.min);
        let knob_x = x + rel_value * w - 5.0;
        draw_rectangle(knob_x, knob_y, 10.0, slider_h, self.knob_color);

        // Draw ticks + labels
        if self.show_ticks {
            let steps = ((self.max - self.min) / self.step).round() as u32;
            for i in 0..=steps {
                let t = i as f32 / steps as f32;
                let tick_x = x + t * w;
                let tick_y = y + slider_h + 2.0;

                let is_major = i % self.label_every == 0;
                let tick_height = if is_major { self.major_tick_height } else { self.minor_tick_height };

                draw_line(tick_x, tick_y, tick_x, tick_y + tick_height, 1.0, self.tick_color);

                if is_major {
                    let val = self.min + i as f32 * self.step;
                    let label = format!("{}", val.round() as i32);
                    let label_size = measure_text(&label, None, 16, 1.0).width;
                    draw_text(
                        &label,
                        tick_x - label_size / 2.0,
                        tick_y + tick_height + 14.0,
                        16.0,
                        self.tick_color,
                    );
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

pub struct SliderWrapper(pub Rc<RefCell<crate::ui::slider::Slider>>);

impl UIElement for SliderWrapper {
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