use qu_qu::card::{Card, Class, Expression};
use qu_qu::game::Game;

fn main() {
    let game = Game::new(2);
    // player 1 asks for Animal Tiger card, player 2 says yes

    // this may generate new facts (asking player must have card of this class)
    let player_has_card: Result<bool> = game.player_asks_for_card(0, 1, Card::new_defined(Class::Defined("Animal"), Expression::Defined("Tiger")));
    // if player_has_card => true
    // this creates new facts (player 1 has new card, player 2 has card less)
    game.hand_over_card(0, 1, Card::new_defined(Class::Defined("Animal"), Expression::Defined("Tiger")));
    // let player give card
    // print new game state (cards by players)
    // let other player give card
    // print new game state (cards by players)
}
