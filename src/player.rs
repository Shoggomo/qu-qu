use crate::card::Card;

pub struct Player<'a> {
    cards: &'a [Card]
}

impl Player {
    pub fn new(cards: &[Card]) -> Player {
        Player {
            cards
        }
    }
}