// Have all of the different decks here
// Item draw pile
// Discard Pile
// Round discard pile
// Character draw pile 
// Deck manager - 


pub struct DeckManager {
    item_draw_pile: CardPile,
    discard_pile: CardPile,
    round_discard_pile: CardPile,
    character_draw_pile: CardPile,
    super_character_draw_pile: CardPile,
}

impl DeckManager {
    pub fn new() -> Self {
        Self {
            item_draw_pile: CardPile::new(),
            discard_pile: CardPile::new(),
            round_discard_pile: CardPile::new(),
            character_draw_pile: CardPile::new(),
            super_character_draw_pile: CardPile::new(),
        }
    }
    
    pub fn add_item_draw_pile(&mut self, card_id: String) {
        self.item_draw_pile.add(card_id);
    }

    pub fn add_character_draw_pile(&mut self, card_id: String) {
        self.character_draw_pile.add(card_id);
    }

    pub fn get_item_draw_pile(&self) -> &Vec<String> {
        self.item_draw_pile.get_all()
    }

}



struct CardPile {
    cards: Vec<String>
}

impl CardPile {
    fn new() -> Self {
        Self { 
            cards: vec![] 
        }
    }

    fn add(&mut self, card_id: String) {
        self.cards.push(card_id);
    }

    fn get_all(&self) -> &Vec<String> {
        &self.cards
    }
}

