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
use super::game::{Action, Game, Winner};


pub struct Ui<R, W: io::Write> {
    game: Game,
    stdin: input::Events<R>,
    stdout: input::MouseTerminal<W>,
}

trait Draw<W: io::Write> {
    fn draw(&self, out: &mut input::MouseTerminal<W>, pos: cursor::Goto)
        -> io::Result<()>;
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
        write!(self.stdout, "{}{}Durak game, press q to exit{}",
               clear::All, cursor::Goto(1, 1), START)?;
        self.game.draw(&mut self.stdout, START)?;
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

fn empty_card<W: io::Write, S: fmt::Display>(f: &mut input::MouseTerminal<W>, symbol: S)
        -> io::Result<()> {
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

impl<W: io::Write> Draw<W> for Game {
    fn draw(&self, out: &mut input::MouseTerminal<W>, pos: cursor::Goto) -> io::Result<()> {
        self.deck.draw(out, pos)?;
        write!(out, "{}", cursor::Goto(START.0 + 40, START.1))?;
        empty_card(out, self.discard.len())?;
        write!(out, "{}Computer:{}",
               cursor::Goto(START.0, START.1 + CARD_HEIGHT),
               cursor::Goto(START.0, START.1 + CARD_HEIGHT + 1))?;
        for _ in 0 .. self.computer.cards.len() {
            empty_card(out, "?")?;
            write!(out, " ")?;
        }
        self.table.draw(out,
                        cursor::Goto(START.0, START.1 + 2 * CARD_HEIGHT + 1))?;
        write!(out, "{}Your cards: ",
               cursor::Goto(START.0, START.1 + 4 * CARD_HEIGHT + 2))?;
        self.player.draw(out,
                         cursor::Goto(START.0, START.1 + 4 * CARD_HEIGHT + 3))?;
        write!(out, "{}",
               cursor::Goto(START.0, 5 * CARD_HEIGHT + 7))?;

        if let Some(winner) = self.winner() {
            write!(out, "{}", winner)
        } else if self.players_turn {
            write!(out, "Play a card or skip turn with space")
        } else {
            write!(out, "Defend with a card or take cards with t")
        }
    }
}


impl<W: io::Write> Draw<W> for Table {
    fn draw(&self, out: &mut input::MouseTerminal<W>, pos: cursor::Goto) -> io::Result<()> {
        write!(out, "{}{}", pos, SEPARATOR)?;
        let mut card_offset = pos.0;
        let attack_start = pos.1 + 1;
        let defense_start = attack_start + 3;
        for (ca, cd) in self.cards.iter() {
            ca.draw(out, cursor::Goto(card_offset, attack_start))?;
            if let Some(c) = cd {
                c.draw(out, cursor::Goto(card_offset + 1, defense_start))?;
            }
            card_offset += CARD_WIDTH + 2;
        }
        write!(out, "{}{}",
               cursor::Goto(pos.0, defense_start + CARD_HEIGHT + 1),
               SEPARATOR)
    }
}

impl<W: io::Write> Draw<W> for Deck {
    fn draw(&self, out: &mut input::MouseTerminal<W>, pos: cursor::Goto) -> io::Result<()> {
        if let Some(ref trump_card) = self.trump_card() {
            empty_card(out, self.cards.len() - 1)?;
            trump_card.draw(out, cursor::Goto(pos.0 + CARD_WIDTH + 1, pos.1))
        } else {
            write!(out, "No cards in the deck, time to win!")
        }
    }
}


impl<W: io::Write> Draw<W> for Hand {
    fn draw(&self, out: &mut input::MouseTerminal<W>, pos: cursor::Goto) -> io::Result<()> {
        let mut i = 0;
        for card in self.cards.iter() {
            let card_offset = pos.0 + (CARD_WIDTH + 1) * i;
            card.draw(out, cursor::Goto(card_offset, pos.1))?;
            let c = ::std::char::from_digit((i + 1) as u32, 16).unwrap_or(' ');
            write!(out, "{}{}",
                   cursor::Goto(card_offset + CARD_WIDTH / 2, pos.1 + CARD_HEIGHT),
                   c)?;
            i += 1;
        }
        Ok(())
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

impl<W: io::Write> Draw<W> for Card {
    fn draw(&self, out: &mut input::MouseTerminal<W>, pos: cursor::Goto) -> io::Result<()> {
        write!(out, "{}╔═════╗{}{}",
               pos,
               cursor::Down(1),
               cursor::Left(CARD_WIDTH))?;
        write!(out, "║{:2}   ║{}{}",
               self.value.to_string(),
               cursor::Down(1),
               cursor::Left(CARD_WIDTH))?;
        write!(out, "║  {}  ║{}{}",
               self.suit.to_string(),
               cursor::Down(1),
               cursor::Left(CARD_WIDTH))?;
        write!(out, "║   {:>2}║{}{}",
               self.value.to_string(),
               cursor::Down(1),
               cursor::Left(CARD_WIDTH))?;
        write!(out, "╚═════╝{}",
               cursor::Up(CARD_HEIGHT - 1))
    }
}

impl fmt::Display for Winner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Winner::Player => "You have won, congratulations!",
            Winner::Computer => "Unfortunately, you have lost the game..",
            Winner::Tie => "It's a tie, let's have a drink :)",
        };
        write!(f, "{}", s)
    }
}
