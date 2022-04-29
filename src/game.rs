use crate::card::{Card, Class};
use crate::player::Player;

#[derive(Debug, Clone)]
pub struct GameData<'a> {
    pub players: Vec<Player<'a>>,
    pub current_player_index: usize,
}

#[derive(Debug)]
enum State<'a> {
    AskingForCard { game_data: GameData<'a> },
    HandingOverCard { game_data: GameData<'a> },
    PlayerWon { player: Player<'a> },
    ParadoxCreated { reason: String },
    Error(String),
}

#[derive(Debug)]
enum Event<'a> {
    AskedForCard { asker: &'a mut Player<'a>, asked: &'a Player<'a>, card: Card<'a> },
    CardHandedOver { giver: &'a mut Player<'a>, reciever: &'a mut Player<'a>, card: Card<'a> },
    CardNotHandedOver { giver: &'a mut Player<'a>, card: Card<'a> },
}

impl<'a> GameData<'a> {
    pub const CARDS_PER_PLAYER: usize = 4;

    pub fn new(playercount: usize) -> Self {
        assert!(playercount > 1);

        GameData {
            players: vec![Player::new(vec![Card::new(); GameData::CARDS_PER_PLAYER]); playercount],
            current_player_index: 0,
        }
    }

    // pub fn define_player_has_class(&self, player: &mut Player, class: &Class<'a>) -> Result<(), &str> {
    //     // TODO check, if class can exist and player can own card of class
    //     // see class_can_exist
    //     // other players own a maximum of 3 cards of this class
    //     // player already owns card of class, or room for new card
    //     if !self.class_can_exist(class) {
    //         Err("Class cannot exist.")
    //     } else if self.amount_defined_cards_of_class_for_other_players(player, class) > 3 {
    //         Err("Other players already have 4 cards of this class.")
    //     }else if !(player.has_card_of_class(class) || player.has_undefined_card()) {
    //         Err("Player has no space for a new class and does not already have a card of this class.")
    //     } else {
    //         player.define_has_class(class)
    //     }
    // }
    //
    // pub fn define_player_has_not_card(&self, player: &mut Player, card: Card) -> Result<(), &str> {
    //     // TODO check, if player must own card (if true -> paradox)
    //     // see player_must_have_card
    //
    //     // TODO add card to cannot_have list
    //
    //     unimplemented!()
    // }
    //
    // pub fn move_card(&mut self, player_from: &mut Player, player_to: &mut Player, card: Card) -> Result<(), &str> {
    //     // TODO check, if giver can own card
    //     // TODO remove card from giver, add to asker
    //
    //     unimplemented!()
    // }
    //
    //
    // fn class_can_exist(&self, class: &Class) -> bool {
    //     // class already exists in deck, or there is room for a new class
    //     unimplemented!()
    // }
    //
    // fn card_can_exist(&self, card: Card) -> bool {
    //     // class can exist (see class_can-exist)
    //
    //     // card already in deck, or
    //     // card of same class, without expression in deck, or
    //     // less than 4 fully defined cards with class in deck
    //     unimplemented!()
    // }
    //
    // fn player_can_own_card(&self, player: &Player, card: Card) -> bool {
    //     // check, if card can exist (card_can_exist)
    //
    //     // card already in hand, or
    //     // card of same class, without expression in hand, or
    //     // undefined card in hand
    //     unimplemented!()
    // }
    //
    // fn player_must_own_card(&self, player: &Player, card: Card) -> bool {
    //     // other players cannot own card (see player_can_have_card)
    //     unimplemented!()
    // }

    pub fn all_cards(&self) -> Vec<&Card> {
        self.players.iter()
            .map(|player| player.hand())
            .flatten()
            .collect()
    }

    // fn amount_defined_cards_of_class_for_other_players(&self, player: &Player, class: &Class) -> usize {
    //     self.players.iter()
    //         .filter(|p| *p != player)
    //         .map(|p| p.amount_defined_cards_of_class(class))
    //         .sum()
    // }
}


impl<'a> State<'a> {
    pub fn new(playercount: usize) -> Self {
        let game_data = GameData::new(playercount);

        State::AskingForCard { game_data }
    }

    fn next(self, event: Event<'a>) -> State {
        match (self, event) {
            (State::AskingForCard { game_data }, Event::AskedForCard { asked, asker, card }) => {
                // TODO create paradox (player cannot have card of class)
                // State::ParadoxCreated { reason: "could not have card of class".to_string() }
                // TODO asker now has card of class

                // define asker has card of class
                // check for paradox
                State::HandingOverCard { game_data }
            }
            (State::HandingOverCard { game_data }, Event::CardHandedOver { giver, reciever, card }) => {
                // TODO create paradox (asked cannot have card)
                // State::ParadoxCreated { reason: "asked  not have card".to_string() }
                // TODO asker now has card more, asking now has card less (may remove card from "cannot have" list)

                // move card
                // check for paradox
                State::AskingForCard { game_data }
            }
            (State::HandingOverCard { game_data }, Event::CardNotHandedOver { giver, card }) => {
                // TODO create paradox (asked must have card)
                // State::ParadoxCreated { reason: "asked must have card".to_string() }
                // TODO asker won (has four of a kind)
                // State::PlayerWon { player: (), game_data }
                // TODO asked now cannot have card (add to "cannot have" card list)

                // define player does not have card
                // check for paradox
                State::AskingForCard { game_data }
            }
            (s, e) => State::Error(format!("Impossible transition: {s:?} -> {e:?}")),
        }
    }

    // pub fn get_classes(&self) -> Vec<&Class> {
    //     let mut classes = self.cards.iter().map(|c| c.class()).collect::<Vec<_>>();
    //     classes.sort();
    //     classes.dedup_by_key(|c| c);
    //     classes
    // }
    //
    // pub fn get_cards_by_class(&self, class: Class) -> Vec<&Class> {
    //     let cards = self.cards.iter().filter(|c| c.class() == class).collect();
    //     cards
    // }
}
