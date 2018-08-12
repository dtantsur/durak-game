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

use super::card::Deck;
use super::game::Game;


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
        loop {
            self.draw()?;

            let cmd = self.stdin.next().unwrap()?;
            match cmd {
                Event::Key(Key::Char('q')) => return self.exit(),
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

    fn exit(&mut self) -> Result<(), io::Error> {
        write!(self.stdout, "{}{}Bye", clear::All, cursor::Goto(1, 1))
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.deck)?;
        write!(f, "{}Computer has {} cards",
               cursor::Goto(START.0, START.1 + 1),
               self.computer.len())?;
        write!(f, "{}Your cards: {:?}",
               cursor::Goto(START.0, START.1 + 2),
               self.player)
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref trump_card) = self.trump_card() {
            write!(f, "Deck: Remaining = {}, Trump = {:?}",
                   self.cards.len(), trump_card)
        } else {
            write!(f, "No cards in the deck, time to win!")
        }
    }
}
