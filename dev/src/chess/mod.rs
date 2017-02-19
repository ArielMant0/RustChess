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
    pub white_player: Player,
    pub black_player: Player,
    pub board: Board,
    pub turn: bool,
    pub gameover: bool
}

impl ChessGame {
    pub fn new() -> Self {
        ChessGame{ white_player: Player::new(PlayerType::Human, Color::White),
                   black_player: Player::new(PlayerType::Human, Color::Black),
                   board: Board::new(),
                   turn: true,
                   gameover: false }
    }

    /// Returns the color of the player whose turn it is
    pub fn turn_color(&self) -> Color {
        if self.turn {
            Color::White
        } else {
            Color::Black
        }
    }

    /// Makes the move from 'from' to 'to' and return whether a figure was captured
    fn make_move(&mut self, from: Position, to: Position) -> bool {
        let mut captured = false;
        if self.turn {
            // If a figure is at 'to' capture it and set flag
            if !self.board.is_empty(to) {
                let name = self.board[to].get_figure().unwrap().name();
                self.black_player.capture(name.clone(), to);
                captured = true;
            }
            // Move figure(s) in board and player
            self.board.move_figure(from, to);
            self.white_player.move_figure(from, to);
        } else {
            // If a figure is at 'to' capture it and set flag
            if !self.board.is_empty(to) {
                let name = self.board[to].get_figure().unwrap().name();
                self.white_player.capture(name.clone(), to);
                captured = true;
            }
            // Move figure(s) in board and player
            self.board.move_figure(from, to);
            self.black_player.move_figure(from, to);
        }
        self.turn = !self.turn;
        captured
    }

    /// Makes a turn using the AI
    pub fn do_ai_turn(&mut self) -> Option<((Position, Position), bool)> {

        if !self.gameover {
            if self.board.checkmate(&mut self.white_player, &mut self.black_player) {
                self.gameover = true;
                println!("Game is over");
                return None
            }

            let (from , to) = match self.turn {
                true => {
                    if self.white_player.ptype() != PlayerType::Human {
                        self.white_player.get_ai_move(&self.board, &self.black_player)
                    } else {
                        return None
                    }
                },
                false => {
                    if self.black_player.ptype() != PlayerType::Human {
                        self.black_player.get_ai_move(&self.board, &self.white_player)
                    } else {
                        return None
                    }
                }
            };

            return Some(((from, to), self.make_move(from , to)))
        }
        None
    }

    /// Makes a turn based on player input
    pub fn do_turn(&mut self, from: Position, to: Position) -> i8 {

        if !self.gameover {
            if self.board.checkmate(&mut self.white_player, &mut self.black_player) {
                self.gameover = true;
                println!("Game is over");
                return -1
            }

            let result = match self.turn {
                true => self.board.is_move_valid(from, to, &mut self.white_player, &mut self.black_player),
                false => self.board.is_move_valid(from, to, &mut self.black_player, &mut self.white_player)
            };

            if result {
                if self.make_move(from, to) {
                    return 1
                } else {
                    return 0
                }
            } else {
                return -1
            }
        }
        -1
    }
}
