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

#[derive(Debug)]
pub struct AI;

impl AI {
    pub fn new() -> AI { AI }

    pub fn plan_attack(&self, game: &Game) -> Option<Card> {
        game.computer.acceptable_moves(&game.table, game.deck.trump)
            .into_iter().next()
    }

    pub fn plan_defense(&self, game: &Game) -> Option<Card> {
        game.computer.acceptable_moves(&game.table, game.deck.trump)
            .into_iter().next()
    }
}
