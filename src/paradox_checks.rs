//! Here are all checks, if a game state contains a paradox.

use itertools::Itertools;
use crate::card::Class;
use crate::game::GameData;

pub type Paradox<'a> = &'a str;


// Possible paradoxes:
//  - Too many classes
//  - Too many expressions for class

pub fn run_checks<'a>(game: &GameData) -> Result<(), Paradox<'a>> {
    let checks = [
        check_amount_classes,
    ];

    for check in checks {
        match check(game) {
            Ok(_) => continue,
            Err(e) => return Err(e),
        }
    }

    Ok(())
}


fn check_amount_classes<'a>(game: &GameData) -> Result<(), Paradox<'a>> {
    let classes = game.all_cards().iter()
        .unique_by(|c| c.class())
        .collect::<Vec<_>>();

    if classes.len() <= 4 {
        Ok(())
    } else {
        Err("Too many classes.")
    }
}

fn check_amount_expressions_per_class<'a>(game: &GameData) -> Result<(), Paradox<'a>> {
    let mut classes = game.all_cards().iter()
        .filter(|c| *c.class() != Class::Undefined)
        .counts_by(|c| c.class());

    classes.retain(|_, amount| *amount > 4);

    if classes.len() > 0 {
        let message = format!("Too many expressions for classes: {:?}", classes.keys().join(", "));
        Err(message.as_str())
    } else {
        Ok(())
    }
}