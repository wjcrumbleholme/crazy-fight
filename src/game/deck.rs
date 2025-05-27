// Have all of the different decks here
// Item draw pile
// Discard Pile
// Round discard pile
// Character draw pile 
// Deck manager - 

use std::{collections::HashMap, fs::{self, read_dir}};

use super::{card::{Card, CardManager, CardType}, GameManger};


pub struct DeckManager {
    item_draw_pile: CardPile,
    discard_pile: CardPile,
    round_discard_pile: CardPile,
    character_draw_pile: CardPile,
}

impl DeckManager {
    pub fn new() -> Self {
        Self {
            item_draw_pile: CardPile::new(),
            discard_pile: CardPile::new(),
            round_discard_pile: CardPile::new(),
            character_draw_pile: CardPile::new(),
        }
    }
    
    pub fn add_item_draw_pile(&mut self, card: Card) {
        self.item_draw_pile.add(card);
    }

    pub fn add_character_draw_pile(&mut self, card: Card) {
        self.character_draw_pile.add(card);
    }

}


struct CardPile {
    cards: Vec<Card>
}

impl CardPile {
    fn new() -> Self {
        Self { 
            cards: vec![] 
        }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn get_all(self) -> Vec<Card> {
        self.cards
    }
}

