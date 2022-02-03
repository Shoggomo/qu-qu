use crate::card::{Card, Class};
use crate::player::Player;

pub struct Game<'a> {
    players: Vec<Player<'a>>,
    cards: Vec<Card>,
    current_player: &'a Player<'a>,
}

impl Game {
    pub const CARDS_PER_PLAYER: usize = 4;

    pub fn new(playercount: usize) -> Game {
        assert!(playercount > 1);

        let mut players = Vec::with_capacity(playercount);
        let cards = vec![Card::new(); playercount * Game::CARDS_PER_PLAYER];

        for cards in cards.chunks(Game::CARDS_PER_PLAYER) {
            players.push(Player::new(cards));
        }

        Game {
            players,
            cards,
            current_player: players.first().unwrap()
        }
    }

    pub fn get_classes(&self) -> Vec<&Class> {
        let mut classes = self.cards.iter().map(|c| c.class()).collect::<Vec<_>>();
        classes.sort();
        classes.dedup_by_key(|c| c);
        classes
    }

    pub fn get_cards_by_class(&self, class: Class) -> Vec<&Class> {
        let cards = self.cards.iter().filter(|c| c.class() == class).collect();
        cards
    }
}
