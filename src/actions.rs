//! Here are all actions that modify a games state.

use crate::card::{Card, Class};
use crate::game::GameData;
use crate::paradox_checks::Paradox;


pub fn define_player_has_card_of_class<'a>(game: &GameData<'a>, class: &Class<'a>) -> Result<GameData<'a>, Paradox<'a>> {
    let mut new_state = game.clone();
    let player = new_state.players.get_mut(new_state.current_player_index).expect("Player not found!");

    match player.define_has_class(class) {
        Ok(_) => Ok(new_state),
        Err(paradox) => Err(paradox)
    }
}

pub fn define_player_has_card<'a>(game: &GameData<'a>, card: &'a Card<'a>) -> Result<GameData<'a>, Paradox<'a>> {
    let mut new_state = game.clone();
    let player = new_state.players.get_mut(new_state.current_player_index).expect("Player not found!");

    match player.define_has_card(card) {
        Ok(_) => Ok(new_state),
        Err(paradox) => Err(paradox),
    }
}

pub fn move_card<'a>(game: &GameData<'a>, from_player_index: usize, to_player_index: usize, card: &Card<'a>) -> Result<GameData<'a>, Paradox<'a>> {
    let mut new_state = game.clone();
    let from_player = new_state.players.get_mut(from_player_index).expect("Player not found!");
    let to_player = new_state.players.get_mut(to_player_index).expect("Player not found!");

    match from_player.remove_card(card) {
        Ok(_) => {
            to_player.add_card(card);
            Ok(new_state)
        }
        Err(paradox) => Err(paradox)
    }
}
