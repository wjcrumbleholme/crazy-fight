use std::{cell::RefCell, rc::Rc};

use macroquad::{prelude::*};
use ui::{button::Button, cardui::CardUi, container::{Container, RefCellContainerWrapper}, Alignment, Position, Size, UIContext, UIElement, UIMessage};
use game::{ GameManger};

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

    let mut card_view = Rc::new(RefCell::new(Container::new(
        Position::Align(Alignment::Centre), 
        Position::Align(Alignment::Centre), 
        Size::Rel(0.4), 
        Size::Rel(0.2),
        LIGHTGRAY,
        ui::Layout::RowCentre,
        Size::Rel(0.1)
    )));

    let mut game_manager = GameManger::new();
    game_manager.load_deck("res/decks/base_deck");
    let test_player = game_manager.test_create_player();
    let mut ctx = UIContext::new();

    let example_btn = Button::new(
        Position::Align(Alignment::Centre), 
        Position::Rel(0.7), 
        Size::Abs(400.0), 
        Size::Abs(100.0), 
        LIME, 
        BLACK, 
        "Click Me!".to_string(), 
        32, 
        Some(UIMessage::DrawCard(test_player)),
    );
    

    
    root.add_child(Box::new(RefCellContainerWrapper(card_view.clone())));

    root.add_child(Box::new(example_btn));

    

    loop {
        clear_background(BLACK);

        root.draw(&mut ctx, 0.0, 0.0, screen_width(), screen_height());


        for msg in ctx.message_queue.drain(..) {
            match msg {
                UIMessage::DrawCard(_) => {
                    println!("Button is clicked");
                    if let Some(cards_instantiated) = game_manager.test_draw_pile(test_player) {
                        card_view.borrow_mut().clear_children();
                        for card_instance_id in cards_instantiated {
                            if let Some(card) = game_manager.get_card_manager().get_card_from_instance_id(&card_instance_id) {
                                let card_load_ui = CardUi::new(
                                Position::Align(Alignment::Centre), 
                                Position::Align(Alignment::Centre),
                                load_texture(&card.get_img_path()).await.unwrap(), 
                                || {println!("Card loaded clicked")}
                                );
                                card_view.borrow_mut().add_child(Box::new(card_load_ui));
                            }
                        }
                    }
                }
            }
        }


        next_frame().await
    }
}