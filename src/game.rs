// Copyright 2018 Dmitry Tantsur <divius.inside@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Game structure.

use rand::{self, Rng};

use super::ai::AI;
use super::card::{Card, Deck, Hand, Table};

#[derive(Debug)]
pub struct Game {
    pub ai: AI,
    pub deck: Deck,
    pub discard: Vec<Card>,
    pub player: Hand,
    pub computer: Hand,
    pub players_turn: bool,
    pub table: Table,
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    /// Attack/defend with the card.
    Play(Card),
    /// Take cards or finish attack.
    EndTurn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Winner {
    Player,
    Computer,
    Tie,
}

#[derive(Debug, Clone, Copy)]
pub enum Response {
    /// Computer attacks or defends with a new card.
    Play(Card),
    /// Computer takes cards.
    Take,
    /// The turn is over.
    EndTurn,
    /// The game is over.
    GameOver(Winner),
}

impl Game {
    pub fn new(ai: AI) -> Game {
        let mut rng = rand::thread_rng();
        let mut deck = Deck::new(&mut rng);
        let player = Hand::new(&mut deck);
        let computer = Hand::new(&mut deck);
        Game {
            ai: ai,
            deck: deck,
            discard: Vec::new(),
            player: player,
            computer: computer,
            players_turn: rng.gen_bool(0.5),
            table: Table::new(),
        }
    }

    pub fn start(&mut self) {
        if !self.players_turn {
            let _ = self.start_attack();
        }
    }

    pub fn player_action(&mut self, action: Action) -> Response {
        if self.players_turn {
            match action {
                Action::Play(card) => self.defend(card),
                Action::EndTurn => self.switch_turn()
            }
        } else {
            match action {
                Action::Play(card) => self.plan_attack(card),
                Action::EndTurn => self.player_took_cards()
            }
        }
    }

    pub fn is_valid_move(&self, card: &Card) -> bool {
        if self.players_turn && (self.table.is_full() || self.computer.cards.is_empty()) {
            return false;
        }
        self.player.acceptable_moves(&self.table, self.deck.trump).contains(card)
    }

    pub fn winner(&self) -> Option<Winner> {
        if self.deck.cards.is_empty() {
            if self.player.cards.is_empty() {
                Some(if self.computer.cards.is_empty() {
                    Winner::Tie
                } else {
                    Winner::Player
                })
            } else if self.computer.cards.is_empty() {
                Some(Winner::Computer)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Start computer attack.
    fn start_attack(&mut self) -> Response {
        let attack = self.ai.plan_attack(self)
            .expect("Attack impossible on first move");
        self.computer.attack_with(attack, &mut self.table);
        Response::Play(attack)
    }

    /// Player attacks us with the provided card, defend.
    fn defend(&mut self, attack: Card) -> Response {
        assert!(self.players_turn);
        assert!(!self.table.is_full());

        self.player.attack_with(attack, &mut self.table);
        let response = match self.ai.plan_defense(self) {
            Some(response) => {
                self.computer.defend_with(response, &mut self.table);
                Response::Play(response)
            },
            None => {
                self.computer.take_from(&mut self.table);
                // Is this ever needed? At least it won't hurt.
                self.computer.draw_from(&mut self.deck);
                self.player.draw_from(&mut self.deck);
                Response::Take
            }
        };

        // We only calculate the winner after ther response to account
        // for the case when both players finish simultaneously.
        if let Some(winner) = self.winner() {
            Response::GameOver(winner)
        } else {
            response
        }
    }

    /// Player finishes the attack, start ours.
    fn switch_turn(&mut self) -> Response {
        assert!(self.players_turn);

        // Order matters here - attacker goes first.
        self.player.draw_from(&mut self.deck);
        self.computer.draw_from(&mut self.deck);

        // Somebody might win after drawing cards.
        if let Some(winner) = self.winner() {
            return Response::GameOver(winner);
        }

        // Clean up
        self.players_turn = false;
        self.discard_table();

        self.start_attack()
    }

    /// Player defended, plan another attack.
    fn plan_attack(&mut self, last_defense: Card) -> Response {
        assert!(!self.players_turn);

        self.player.defend_with(last_defense, &mut self.table);
        // Check if attacking is possible, end turn if not.
        if self.table.is_full() {
            // Order matters here - attacker goes first.
            self.computer.draw_from(&mut self.deck);
            self.player.draw_from(&mut self.deck);

            // Somebody might win after drawing cards.
            if let Some(winner) = self.winner() {
                Response::GameOver(winner)
            } else {
                self.players_turn = true;
                self.discard_table();
                Response::EndTurn
            }
        } else {
            // Whether the defense was the last card in the game.
            if let Some(winner) = self.winner() {
                Response::GameOver(winner)
            } else {
                if let Some(attack) = self.ai.plan_attack(self) {
                    self.computer.attack_with(attack, &mut self.table);
                    Response::Play(attack)
                } else {
                    // No more cards to attack with, yielding.
                    self.players_turn = true;
                    self.discard_table();
                    // Order matters here - attacker goes first.
                    self.computer.draw_from(&mut self.deck);
                    self.player.draw_from(&mut self.deck);
                    Response::EndTurn
                }
            }
        }
    }

    /// Player took cards, start a new attack series.
    fn player_took_cards(&mut self) -> Response {
        assert!(!self.players_turn);

        self.player.take_from(&mut self.table);
        self.computer.draw_from(&mut self.deck);

        // Check for the win.
        if let Some(winner) = self.winner() {
            return Response::GameOver(winner);
        } else {
            self.start_attack()
        }
    }

    fn discard_table(&mut self) {
        for (ac, dc) in self.table.cards.drain(..) {
            self.discard.push(ac);
            if let Some(c) = dc {
                self.discard.push(c);
            }
        }
    }
}
