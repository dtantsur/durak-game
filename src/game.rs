// Copyright 2018 Dmitry Tantsur <divius.inside@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Game structure.

use rand;

use super::card::{Card, Deck, Hand};


#[derive(Debug)]
pub struct Game {
    pub deck: Deck,
    pub discard: Vec<Card>,
    pub player: Hand,
    pub computer: Hand,
}

impl Game {
    pub fn new() -> Game {
        let mut deck = Deck::new(&mut rand::thread_rng());
        let player = Hand::new(&mut deck);
        let computer = Hand::new(&mut deck);
        Game {
            deck: deck,
            discard: Vec::new(),
            player: player,
            computer: computer,
        }
    }
}
