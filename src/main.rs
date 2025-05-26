use macroquad::prelude::*;
use ui::{Alignment, Container, Position, Size, UIElement};

pub mod ui;


fn window_conf() -> Conf {
    Conf {
        window_title: "Crazy Fight".to_string(),
        window_width: 1920,
        window_height: 1080,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {

    // Background
    let mut root = Container::new(
        Position::Align(Alignment::LeTop), 
        Position::Align(Alignment::LeTop), 
        Size::Rel(1.0), 
        Size::Rel(1.0),
        GREEN,
    );

    //Card example
    let mut child = Container::new(
        Position::Align(Alignment::Centre), 
        Position::Align(Alignment::Centre), 
        Size::Abs(100.0), 
        Size::Abs(140.0),
        WHITE,
    );

    let mut child2 = Container::new(
        Position::Align(Alignment::Centre), 
        Position::Align(Alignment::Centre), 
        Size::Abs(50.0), 
        Size::Abs(50.0),
        RED,
    );

    child.add_child(Box::new(child2));
    root.add_child(Box::new(child));


    loop {
        clear_background(BLACK);

        root.draw(0.0, 0.0, screen_width(), screen_height());

        next_frame().await
    }
}