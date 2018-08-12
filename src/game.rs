// Copyright 2018 Dmitry Tantsur <divius.inside@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Game structure.

use rand;

use super::card::{Card, Deck};


#[derive(Debug)]
pub struct Game {
    pub deck: Deck,
    rng: rand::ThreadRng,

    pub player: Vec<Card>,
    pub computer: Vec<Card>,
}

const HAND_SIZE: usize = 6;


fn draw(deck: &mut Deck, hand: &mut Vec<Card>) {
    if hand.len() > HAND_SIZE {
        return
    }

    for _i in hand.len() .. HAND_SIZE {
        if deck.cards.is_empty() {
            return
        }

        hand.push(deck.draw());
        hand.sort_unstable();
    }
}

impl Game {
    pub fn new() -> Game {
        let mut rng = rand::thread_rng();
        let deck = Deck::new(&mut rng);
        let mut game = Game {
            deck: deck,
            rng: rng,
            player: Vec::with_capacity(HAND_SIZE),
            computer: Vec::with_capacity(HAND_SIZE),
        };
        draw(&mut game.deck, &mut game.player);
        draw(&mut game.deck, &mut game.computer);
        assert_eq!(game.player.len(), HAND_SIZE);
        assert_eq!(game.computer.len(), HAND_SIZE);
        game
    }
}
