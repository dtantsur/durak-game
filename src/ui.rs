// Copyright 2018 Dmitry Tantsur <divius.inside@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Game UI.

use std::fmt;
use std::io;

use termion::{clear, cursor};
use termion::event::{Event, Key};
use termion::input::{self, TermRead};

use super::card::{Card, Deck, Hand, Suit, Table, Value};
use super::game::{Action, Game};


pub struct Ui<R, W: io::Write> {
    game: Game,
    stdin: input::Events<R>,
    stdout: input::MouseTerminal<W>,
}


const START: cursor::Goto = cursor::Goto(1, 2);

impl<R: io::Read, W: io::Write> Ui<R, W> {
    pub fn new(game: Game, stdin: R, stdout: W) -> Ui<R, W> {
        Ui {
            game: game,
            stdin: stdin.events(),
            stdout: stdout.into(),
        }
    }

    pub fn start(&mut self) -> Result<(), io::Error> {
        self.game.start();

        loop {
            self.draw()?;

            let cmd = self.stdin.next().unwrap()?;
            match cmd {
                Event::Key(Key::Char('q')) => return self.exit(),
                Event::Key(Key::Char(c)) if c.is_digit(16) =>
                    self.process_card(c.to_digit(16).unwrap() as usize),
                Event::Key(Key::Char(' ')) => self.process_end_turn(),
                Event::Key(Key::Char('t')) => self.process_take(),
                _ => ()
            }
        }
    }

    fn draw(&mut self) -> Result<(), io::Error> {
        write!(self.stdout, "{}{}Durak game, press q to exit{}{}",
               clear::All, cursor::Goto(1, 1), START, self.game)?;
        self.stdout.flush()?;

        Ok(())
    }

    fn process_end_turn(&mut self) {
        if self.game.players_turn {
            let _ = self.game.player_action(Action::EndTurn);
        }
    }

    fn process_card(&mut self, index: usize) {
        if index <= self.game.player.cards.len() {
            let card = self.game.player.cards[index - 1];
            if self.game.is_valid_move(&card) {
                let _ = self.game.player_action(Action::Play(card));
            }
        }
    }

    fn process_take(&mut self) {
        if !self.game.players_turn {
            let _ = self.game.player_action(Action::EndTurn);
        }
    }

    fn exit(&mut self) -> Result<(), io::Error> {
        write!(self.stdout, "{}{}Bye", clear::All, cursor::Goto(1, 1))
    }
}

const SEPARATOR: &'static str =
    "-----------------------------------------------";

const CARD_WIDTH: u16 = 7;
const CARD_HEIGHT: u16 = 5;

fn empty_card<S: fmt::Display>(f: &mut fmt::Formatter, symbol: S) -> fmt::Result {
    write!(f, "╔═════╗{}{}",
           cursor::Down(1),
           cursor::Left(CARD_WIDTH))?;
    write!(f, "║     ║{}{}",
           cursor::Down(1),
           cursor::Left(CARD_WIDTH))?;
    write!(f, "║ {:^3} ║{}{}",
           symbol,
           cursor::Down(1),
           cursor::Left(CARD_WIDTH))?;
    write!(f, "║     ║{}{}",
           cursor::Down(1),
           cursor::Left(CARD_WIDTH))?;
    write!(f, "╚═════╝{}",
           cursor::Up(CARD_HEIGHT - 1))
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}",
               self.deck,
               cursor::Goto(START.0 + 40, START.1))?;
        empty_card(f, self.discard.len())?;
        write!(f, "{}Computer:{}",
               cursor::Goto(START.0, START.1 + CARD_HEIGHT),
               cursor::Goto(START.0, START.1 + CARD_HEIGHT + 1))?;
        for _ in 0 .. self.computer.cards.len() {
            empty_card(f, "?")?;
            write!(f, " ")?;
        }
        write!(f, "{}{}{}{}{}",
               cursor::Goto(START.0, START.1 + 2 * CARD_HEIGHT + 1),
               SEPARATOR,
               self.table,
               cursor::Goto(START.0, START.1 + 4 * CARD_HEIGHT + 1),
               SEPARATOR)?;
        write!(f, "{}Your cards: {}{}{}",
               cursor::Goto(START.0, START.1 + 4 * CARD_HEIGHT + 2),
               cursor::Goto(START.0, START.1 + 4 * CARD_HEIGHT + 3),
               self.player,
               cursor::Goto(START.0, 5 * CARD_HEIGHT + 5))?;

        if self.players_turn {
            write!(f, "Play a card or skip turn with space")
        } else {
            write!(f, "Defend with a card or take cards with t")
        }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let table_start = START.1 + 2 * CARD_HEIGHT + 2;
        let mut card_offset = START.0;
        for (ca, cd) in self.cards.iter() {
            write!(f, "{}{}", cursor::Goto(card_offset, table_start), ca)?;
            if let Some(c) = cd {
                write!(f, "{}{}",
                       cursor::Goto(card_offset + 1, table_start + 3), c)?;
            }
            card_offset += CARD_WIDTH + 2;
        }
        Ok(())
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref trump_card) = self.trump_card() {
            empty_card(f, self.cards.len() - 1)?;
            write!(f, "{}", trump_card)
        } else {
            write!(f, "No cards in the deck, time to win!")
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Value::Six => "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nine => "9",
            Value::Ten => "10",
            Value::Jack => "J",
            Value::Queen => "Q",
            Value::King => "K",
            Value::Ace => "A"
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
            Suit::Spades => "♠"
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "╔═════╗{}{}",
               cursor::Down(1),
               cursor::Left(CARD_WIDTH))?;
        write!(f, "║{:2}   ║{}{}",
               self.value.to_string(),
               cursor::Down(1),
               cursor::Left(CARD_WIDTH))?;
        write!(f, "║  {}  ║{}{}",
               self.suit.to_string(),
               cursor::Down(1),
               cursor::Left(CARD_WIDTH))?;
        write!(f, "║   {:>2}║{}{}",
               self.value.to_string(),
               cursor::Down(1),
               cursor::Left(CARD_WIDTH))?;
        write!(f, "╚═════╝{}",
               cursor::Up(CARD_HEIGHT - 1))
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in self.cards.iter() {
            write!(f, "{} ", card)?;
        }
        Ok(())
    }
}
