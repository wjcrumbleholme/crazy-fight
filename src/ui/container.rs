use std::{cell::RefCell, rc::Rc};

use macroquad::{color::{Color}, shapes::draw_rectangle};

use super::{Layout, Padding, Position, Size, UIContext, UIElement, UIMessage};


//Basic container object
pub struct Container {
    children: Vec<Box<dyn UIElement>>,
    x: Position,
    y: Position,
    w: Size,
    h: Size,
    color: Color,
    padding: Padding,
    layout: Layout,
    gap: Size,

}


// Basic container object logic
impl Container {
    pub fn new(x: Position, y: Position, w: Size, h: Size, color: Color, layout: Layout, gap: Size) -> Self{
        Self { 
            children: vec![], 
            x: x,
            y: y,
            w: w,
            h: h,
            color: color,
            padding: Padding::zero(),
            layout: layout,
            gap: gap,
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

    pub fn clear_children(&mut self) {
        self.children.clear();
    }
}

// Implement the trait for the container object
impl UIElement for Container {
    fn draw(&mut self, ctx: &mut UIContext, parent_x: f32, parent_y: f32, parent_w: f32, parent_h: f32) {
        // Calculate the size
        let w = self.w.caclulate(parent_w);
        let h = self.h.caclulate(parent_h);
        // Calculate the x and y coords
        let x = self.x.coord(parent_x, parent_w, w);
        let y = self.y.coord(parent_y, parent_h, h);

        let content_x = x + self.padding.left;
        let content_y = y + self.padding.top;
        let content_w = w - self.padding.left - self.padding.right;
        let content_h = h - self.padding.left - self.padding.right;

        draw_rectangle(x, y, w, h, self.color);



        match self.layout {
            Layout::None => {
                // Recursively draw the child's objects
                for child in &mut self.children {
                    child.draw(ctx, content_x, content_y, content_w, content_h);
                }
            },
            Layout::RowCentre => {
                // Get the width of all of the children
                let mut total_width = 0.0;
                let mut child_sizes = vec![];

                // Loop over all children and add their width
                for child in &self.children {
                    let child_width = child.get_width(w);
                    total_width += child_width;
                    child_sizes.push(child_width);
                }

                // Calculate the gap size 
                let calc_gap = match self.gap {
                    Size::Abs(px) => px,
                    Size::Rel(rel) => {
                        w * rel
                    }
                };

                // Add the missing gap
                total_width += calc_gap * (self.children.len().saturating_sub(1)) as f32;

                // Find starting point
                let mut current_x = x + (w - total_width) / 2.0;

                // Draw all of the children
                for (child, child_w) in self.children.iter_mut().zip(child_sizes.iter()) {
                    child.draw(ctx, current_x, y, *child_w, h);
                    current_x += child_w + calc_gap;
                }

            },
            Layout::ColumnCentre => {
                // Get the height of all of the children
                let mut total_height = 0.0;
                let mut child_sizes = vec![];

                // Loop over all children and add their width
                for child in &self.children {
                    let child_height = child.get_height(h);
                    total_height += child_height;
                    child_sizes.push(child_height);
                }

                // Calculate the gap size 
                let calc_gap = match self.gap {
                    Size::Abs(px) => px,
                    Size::Rel(rel) => {
                        h * rel
                    }
                };

                // Add the missing gap
                total_height += calc_gap * (self.children.len().saturating_sub(1)) as f32;

                // Find starting point
                let mut current_y = y + (h - total_height) / 2.0;

                // Draw all of the children
                for (child, child_h) in self.children.iter_mut().zip(child_sizes.iter()) {
                    child.draw(ctx, x, current_y, w, *child_h);
                    current_y += child_h + calc_gap;
                }

            },
        }


        

        
    }
    fn get_width(&self, parent_w: f32) -> f32 {
        self.w.caclulate(parent_w)
    }
    fn get_height(&self, parent_h: f32) -> f32 {
        self.h.caclulate(parent_h)
    }
}

pub struct RefCellContainerWrapper(pub Rc<RefCell<Container>>);

impl UIElement for RefCellContainerWrapper {
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