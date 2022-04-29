use itertools::Itertools;
use crate::card::{Card, Class};
use crate::paradox_checks::Paradox;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Player<'a> {
    hand: Vec<Card<'a>>,
    has_not_cards: Vec<Card<'a>>,
}

impl<'a> Player<'a> {
    pub fn new(cards: Vec<Card<'a>>) -> Self {
        Player {
            hand: cards,
            has_not_cards: Vec::new(),
        }
    }

    pub fn has_not_cards(&self) -> &Vec<Card<'a>> {
        &self.has_not_cards
    }
    pub fn hand(&self) -> &Vec<Card<'a>> {
        &self.hand
    }

    pub fn has_4_of_class(&self, class: &Class) -> bool {
        let amount_of_class = self.hand.iter().filter(|card| card.class() == class).count();
        amount_of_class >= 4
    }

    pub fn has_card_of_class(&self, class: &Class) -> bool {
        self.hand.iter().find(|card| card.class() == class).is_some()
    }

    fn try_add_card_of_class(&mut self, class: &Class<'a>) -> Result<(), Paradox> {
        let undefined_card = self.hand.iter_mut().find(|card| card.class() == &Class::Undefined);
        match undefined_card {
            Some(undefined_card) => {
                undefined_card.set_class(class.clone());
                Ok(())
            }
            None => Err("Player has no card with undefined class."),
        }
    }

    pub fn define_has_class(&mut self, class: &Class<'a>) -> Result<(), Paradox> {
        if !self.has_card_of_class(class) {
            self.try_add_card_of_class(class)
        } else {
            // player already has card of class
            Ok(())
        }
    }

    pub fn define_has_card(&mut self, card: &'a Card<'a>) -> Result<(), Paradox> {
        let card_of_class = self.hand.iter_mut().find(|c| c.class() == card.class());
        if let Some(c) = card_of_class {
            c.copy_card_values(card);
            return Ok(())
        };

        match self.hand.iter_mut().find(|c| *c.class() == Class::Undefined) {
            Some(c) => {
                c.copy_card_values(card);
                return Ok(())
            },
            None => Err("Player has no space for card"),
        }
    }

    pub fn define_has_not_card()

    pub fn amount_defined_cards_of_class(&self, class: &Class) -> usize {
        self.hand.iter().filter(|c| c.class() == class).count()
    }

    pub fn has_undefined_card(&self) -> bool {
        self.hand.iter().find(|c| *c.class() == Class::Undefined).is_some()
    }

    pub fn remove_card(&mut self, card: &Card) -> Result<(), Paradox> {
        match self.hand.iter().find_position(|c| *c == card) {
            Some(index) => {
                self.hand.swap_remove(index.0);
                Ok(())
            }
            None => Err("Player does not have card. Cannot remove!")
        }
    }

    pub fn add_card(&mut self, card: &Card<'a>) {
        if let Some((index, )) = self.has_not_cards.iter().find_position(card) {
            self.has_not_cards.swap_remove(index);
        };

        self.hand.push(card.clone());
    }
}