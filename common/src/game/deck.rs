// Have all of the different decks here
// Item draw pile
// Discard Pile
// Round discard pile
// Character draw pile 
// Deck manager - 

use macroquad::prelude::rand;



pub struct DeckManager {
    item_draw_pile: Vec<String>,
    discard_pile: Vec<String>,
    round_discard_pile: Vec<String>,
    character_draw_pile: Vec<String>,
    super_character_draw_pile: Vec<String>,
}

impl DeckManager {
    pub fn new() -> Self {
        Self {
            item_draw_pile: vec![],
            discard_pile: vec![],
            round_discard_pile: vec![],
            character_draw_pile: vec![],
            super_character_draw_pile: vec![],
        }
    }
    
    pub fn add_item_draw_pile(&mut self, card_id: String) {
        self.item_draw_pile.push(card_id);
    }

    pub fn add_character_draw_pile(&mut self, card_id: String) {
        self.character_draw_pile.push(card_id);
    }

    pub fn add_super_character_draw_pile(&mut self, card_id: String) {
        self.super_character_draw_pile.push(card_id);
    }

    pub fn add_discard_pile(&mut self, card_id: String) {
        self.discard_pile.push(card_id);
    }

    pub fn get_item_draw_pile(&self) -> &Vec<String> {
        &self.item_draw_pile
    }

    pub fn get_random_card_and_remove_item_pile(&mut self) -> Option<String> {
        if self.item_draw_pile.len() != 0 {
            let card_index = rand::gen_range(0, self.item_draw_pile.len());
            let card_id = &self.item_draw_pile[card_index].clone();
            self.item_draw_pile.remove(card_index);

            Some(card_id.to_owned())
        } else {
            None
        }
    }

}

