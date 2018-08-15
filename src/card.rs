// Copyright 2018 Dmitry Tantsur <divius.inside@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Card definition.

use std::cmp::Ordering;
use std::collections::HashSet;

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

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}

pub const HAND_SIZE: usize = 6;

#[derive(Debug, Clone)]
pub struct Table {
    pub cards: Vec<(Card, Option<Card>)>,
}


impl Card {
    pub fn beats(&self, other: &Card, trump: Suit) -> bool {
        if self.suit == other.suit {
            self.value > other.value
        } else {
            self.suit == trump
        }
    }

    pub fn compare(&self, other: &Card, trump: Suit) -> Ordering {
        if self.suit == other.suit {
            self.value.cmp(&other.value)
        } else if self.suit == trump {
            Ordering::Greater
        } else if other.suit == trump {
            Ordering::Less
        } else {
            self.value.cmp(&other.value)
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
            cards: Vec::with_capacity(HAND_SIZE)
        };
        hand.draw_from(deck);
        hand
    }

    pub fn acceptable_moves(&self, table: &Table, trump: Suit) -> Vec<Card> {
        let mut result = if let Some(ref last) = table.cards.last() {
            if last.1.is_some() {
                // Continued attack, only played values can be used.
                let existing = table.values();
                self.cards.iter().filter(|c| existing.contains(&c.value))
                    .cloned().collect()
            } else {
                // Possible defense
                self.cards.iter().filter(|c| c.beats(&last.0, trump))
                    .cloned().collect()
            }
        } else {
            // New attack, any card can be used.
            self.cards.clone()
        };
        result.sort_unstable_by(|c1, c2| c1.compare(c2, trump));
        result
    }

    pub fn attack_with(&mut self, card: Card, table: &mut Table) {
        assert!(table.cards.len() < HAND_SIZE);
        self.remove(&card);
        table.cards.push((card, None));
    }

    pub fn defend_with(&mut self, card: Card, table: &mut Table) {
        let last = table.cards.pop().expect("Table is empty when defending");
        assert!(last.1.is_none());
        self.remove(&card);
        table.cards.push((last.0, Some(card)));
    }

    pub fn draw_from(&mut self, deck: &mut Deck) {
        while self.cards.len() < HAND_SIZE {
            if deck.cards.is_empty() {
                break
            }

            let _ = self.cards.push(deck.draw());
        }
        self.cards.sort_unstable();
    }

    pub fn take_from(&mut self, table: &mut Table) {
        for (ac, dc) in table.cards.drain(..) {
            let _ = self.cards.push(ac);
            if let Some(c) = dc {
                let _ = self.cards.push(c);
            }
        }
        self.cards.sort_unstable();
    }

    #[inline]
    fn remove(&mut self, card: &Card) {
        self.cards.retain(|c| c != card);
    }
}

impl Table {
    pub fn new() -> Table {
        Table {
            cards: Vec::with_capacity(HAND_SIZE)
        }
    }

    pub fn is_full(&self) -> bool {
        self.cards.len() >= HAND_SIZE
    }

    pub fn values(&self) -> HashSet<Value> {
        let mut result = HashSet::with_capacity(self.cards.len() * 2);
        for (ca, cd) in self.cards.iter() {
            let _ = result.insert(ca.value);
            if let Some(c) = cd {
                let _ = result.insert(c.value);
            }
        }
        result
    }
}
