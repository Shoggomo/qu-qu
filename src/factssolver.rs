use crate::fact::Fact;

struct FactHistory(Vec<Fact>);

impl FactHistory {
    fn new() -> FactHistory {
        FactHistory(Vec::new())
    }

    fn add_fact(&mut self, &fact: Fact) {
        self.0.push(fact);
    }
}

// apply fact (game)

// apply fact history (game)

fn apply_fact_to_game(&game: Game, &fact: Fact) -> Game {
    match fact {
        Fact::PlayerHasNotCard(player, card) => unimplemented!(),
        Fact::PlayerHasCardOfClass(player, class) => unimplemented!(),
        Fact::PlayerGaveCardToOtherPlayer(giver, reciever, card) => unimplemented!(),
    }
}