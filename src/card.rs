// Copyright 2018 Dmitry Tantsur <divius.inside@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Card definition.

use std::collections::HashSet;
use std::mem;

use rand;


#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades
}

const ALL_SUITS: [Suit; 4] = [Suit::Clubs,
                              Suit::Diamonds,
                              Suit::Hearts,
                              Suit::Spades];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Value {
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

const ALL_VALUES: [Value; 9] = [Value::Six,
                                Value::Seven,
                                Value::Eight,
                                Value::Nine,
                                Value::Ten,
                                Value::Jack,
                                Value::Queen,
                                Value::King,
                                Value::Ace];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
    pub trump: Suit,
}

const DECK_SIZE: usize = 36;

#[derive(Debug)]
pub struct Hand {
    pub cards: HashSet<Card>,
}

pub const HAND_SIZE: usize = 6;

pub type Table = Vec<(Card, Option<Card>)>;


impl Card {
    pub fn beats(&self, other: &Card, trump: Suit) -> bool {
        if self.suit == other.suit {
            self.value > other.value
        } else {
            self.suit == trump
        }
    }
}

impl Deck {
    pub fn new_sorted() -> Deck {
        let mut cards = Vec::with_capacity(DECK_SIZE);
        for suit in ALL_SUITS.iter() {
            for value in ALL_VALUES.iter() {
                cards.push(Card { suit: *suit, value: *value });
            }
        }

        let trump = cards[0].suit;
        Deck {
            cards: cards,
            trump: trump,
        }
    }

    pub fn new<R: rand::Rng>(rng: &mut R) -> Deck {
        let mut deck = Deck::new_sorted();
        rng.shuffle(&mut deck.cards);
        deck.trump = deck.cards[0].suit;
        deck
    }

    pub fn trump_card(&self) -> Option<&Card> {
        self.cards.get(0)
    }

    pub fn draw(&mut self) -> Card {
        self.cards.pop().expect("No cards to draw")
    }
}

impl Hand {
    pub fn new(deck: &mut Deck) -> Hand {
        let mut hand = Hand {
            cards: HashSet::with_capacity(HAND_SIZE)
        };
        hand.draw_from(deck);
        hand
    }

    pub fn attack_with(&mut self, card: Card, table: &mut Table) {
        assert!(table.len() < HAND_SIZE);
        self.play(&card);
        table.push((card, None));
    }

    pub fn defend_with(&mut self, card: Card, table: &mut Table) {
        let last = table.pop().expect("Table is empty when defending");
        assert!(last.1.is_none());
        self.play(&card);
        table.push((last.0, Some(card)));
    }

    pub fn draw_from(&mut self, deck: &mut Deck) {
        while self.cards.len() < HAND_SIZE {
            if deck.cards.is_empty() {
                break
            }

            let _ = self.cards.insert(deck.draw());
        }
    }

    pub fn take_from(&mut self, table: &mut Table) {
        let mut new = Vec::with_capacity(HAND_SIZE);
        mem::swap(&mut new, table);
        for (ac, dc) in new.into_iter() {
            let _ = self.cards.insert(ac);
            if let Some(c) = dc {
                let _ = self.cards.insert(c);
            }
        }
    }

    #[inline]
    fn play(&mut self, card: &Card) {
        let res = self.cards.remove(card);
        assert!(res);
    }
}
