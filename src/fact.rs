use crate::card::{Card, Class};
use crate::player::Player;

pub enum Fact {
    PlayerHasNotCard(Player, Card),
    PlayerHasCardOfClass(Player, Class),
    PlayerGaveCardToOtherPlayer(Player, Player, Card)
}

// collect facts over a game and always deduce the current state from that fact list and possibly find a paradox
