use macroquad::{prelude::*};
use ui::{button::Button, cardui::Card, container::Container, Alignment, Position, Size, UIElement};

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
        ui::Layout::None,
        0.0
    );

    let mut card_view = Container::new(
        Position::Align(Alignment::Centre), 
        Position::Align(Alignment::Centre), 
        Size::Rel(0.4), 
        Size::Rel(0.2),
        LIGHTGRAY,
        ui::Layout::RowCentre,
        100.0
    );

    //Card example
    let mut example_card = Card::new(
        Position::Align(Alignment::Centre), 
        Position::Align(Alignment::Centre),
        load_texture("res/test_card.png").await.unwrap(), 
        || {println!("Card clicked")},
    );

    let mut example_card_2 = Card::new(
        Position::Align(Alignment::Centre), 
        Position::Align(Alignment::Centre),
        load_texture("res/test_card.png").await.unwrap(), 
        || {println!("Card 2 clicked")},
    );

    let mut example_card_3 = Card::new(
        Position::Align(Alignment::Centre), 
        Position::Align(Alignment::Centre),
        load_texture("res/test_card.png").await.unwrap(), 
        || {println!("Card 3 clicked")},
    );

    let example_btn = Button::new(
        Position::Align(Alignment::Centre), 
        Position::Rel(0.7), 
        Size::Abs(400.0), 
        Size::Abs(100.0), 
        LIME, 
        BLACK, 
        "Click Me!".to_string(), 
        32, 
        || {println!("Button has been clicked!")},
    );

    card_view.add_child(Box::new(example_card));
    card_view.add_child(Box::new(example_card_2));
    card_view.add_child(Box::new(example_card_3));
    root.add_child(Box::new(card_view));
    root.add_child(Box::new(example_btn));



    loop {
        clear_background(BLACK);

        root.draw(0.0, 0.0, screen_width(), screen_height());

        next_frame().await
    }
}