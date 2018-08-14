// Copyright 2018 Dmitry Tantsur <divius.inside@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Super dangerous AI.

use super::card::Card;
use super::game::Game;


pub fn plan_attack(game: &mut Game) -> Option<Card> {
    game.computer.acceptable_moves(&game.table, game.deck.trump)
        .iter().next().map(|c| *c)
}

pub fn plan_defense(game: &mut Game, attack: Card) -> Option<Card> {
    None  // TODO
}
