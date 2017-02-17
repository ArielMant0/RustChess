// MIT License
//
// Copyright (c) 2017 Franziska Becker, René Warking
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub mod player;
pub mod logic;
pub mod ai;

use self::player::{PlayerType, Player};
use self::logic::{Color, Board, Position};

pub struct ChessGame {
    pub player_one: Player,
    pub player_two: Player,
    pub board: Board,
    pub turn: bool,
    pub gameover: bool,
    pub captured: bool
}

impl ChessGame {
    pub fn new() -> Self {
        ChessGame{ player_one: Player::new(PlayerType::Human, Color::White),
                   player_two: Player::new(PlayerType::Human, Color::Black),
                   board: Board::new(),
                   turn: true,
                   gameover: false,
                   captured: false }
    }

    pub fn was_captured(&self) -> bool {
        self.captured
    }

    pub fn turn_color(&self) -> Color {
        if self.turn {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn do_ai_turn(&mut self) -> Option<(Position, Position)> {

        if !self.gameover {
            if self.board.checkmate(&mut self.player_one, &mut self.player_two) {
                self.gameover = true;
                println!("Game is over");
                return None
            }
            let (mut attack, mut defend) = match self.turn {
                true => (&mut self.player_one, &mut self.player_two),
                false => (&mut self.player_two, &mut self.player_one)
            };


            let (from, to) = {
                if attack.ptype() != PlayerType::Human {
                    attack.get_ai_move(&mut self.board, defend)
                } else {
                    return None
                }
            };

            let mut name = String::new();
            self.captured = false;

            if !self.board.is_empty(to) {
                name = self.board.get_figure(to).unwrap().name();
                defend.capture(to, name.clone());
                self.captured = true;
            }
            self.board.move_figure(from, to);
            attack.move_figure(from, to);

            if !self.board.in_check(attack.king(), defend) {
                self.turn = !self.turn;
                return Some((from, to))
            } else {
                if self.captured {
                    defend.reverse_capture(&name, to);
                    self.captured = false;
                }
                self.board.move_figure(to, from);
                attack.move_figure(to, from);
                return None
            }
        }
        None
    }

    pub fn do_turn(&mut self, from: Position, to: Position) -> bool {

        if !self.gameover {
            if self.board.checkmate(&mut self.player_one, &mut self.player_two) {
                self.gameover = true;
                println!("Game is over");
                return false
            }
            let (mut attack, mut defend) = match self.turn {
                true => (&mut self.player_one, &mut self.player_two),
                false => (&mut self.player_two, &mut self.player_one)
            };

            let mut name = String::new();
            self.captured = false;

            if !self.board.is_empty(to) {
                name = self.board.get_figure(to).unwrap().name();
                defend.capture(to, name.clone());
                self.captured = true;
            }
            self.board.move_figure(from, to);
            attack.move_figure(from, to);

            if !self.board.in_check(attack.king(), defend) {
                self.turn = !self.turn;
                return true
            } else {
                if self.captured {
                    defend.reverse_capture(&name, to);
                    self.captured = false;
                }
                self.board.move_figure(to, from);
                attack.move_figure(to, from);
                return false
            }
        }
        false
    }
}
