use macroquad::{prelude::*};
use ui::{button::Button, cardui::CardUi, container::Container, Alignment, Position, Size, UIElement};
use game::{card::Card, GameManger};
use std::fs::{read_dir, FileType};

pub mod ui;
pub mod game;


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
        Size::Rel(0.0),
    );

    let mut card_view = Container::new(
        Position::Align(Alignment::Centre), 
        Position::Align(Alignment::Centre), 
        Size::Rel(0.4), 
        Size::Rel(0.2),
        LIGHTGRAY,
        ui::Layout::RowCentre,
        Size::Rel(0.1)
    );

    //Card example
    let mut example_card = CardUi::new(
        Position::Align(Alignment::Centre), 
        Position::Align(Alignment::Centre),
        load_texture("res/decks/example_deck/test_card.png").await.unwrap(), 
        || {println!("Card clicked")},
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

    // // Load in the cards
    // for path in read_dir("res").unwrap() {
    //     if path.as_ref().unwrap().path().extension().unwrap() == "json" {
    //         let card_load = Card::load_from_file(path.unwrap().path().to_str().unwrap());
    //         let card_load_ui = CardUi::new(
    //             Position::Align(Alignment::Centre), 
    //             Position::Align(Alignment::Centre),
    //             load_texture(&card_load.get_img_path()).await.unwrap(), 
    //             || {println!("Card loaded clicked")}
    //         );
    //         card_view.add_child(Box::new(card_load_ui));
    //     }
    // }

    let mut game_manager = GameManger::new();
    game_manager.load_deck("res/decks/example_deck");

    

    
    card_view.add_child(Box::new(example_card));
    root.add_child(Box::new(card_view));
    root.add_child(Box::new(example_btn));

    

    loop {
        clear_background(BLACK);

        root.draw(0.0, 0.0, screen_width(), screen_height());

        next_frame().await
    }
}