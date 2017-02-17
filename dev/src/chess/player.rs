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

use std::collections::HashMap;

use chess::logic::{Color, Board, Figure, Position};

pub const FIGURE_NAMES: &'static [&'static str] = &[
    "king", "queen",
    "bishop", "knight",
    "rook", "pawn"
];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayerType {
    Human,
    Dumb,
    Smart
}

#[derive(Debug)]
pub struct Player {
    ptype: PlayerType,
    color: Color,
    figures: HashMap<String, Vec<Position>>
}

impl Player {
    // Returns an instance of a Player with the given PlayerType
    pub fn new(p: PlayerType, c: Color) -> Self {
        match c {
            Color::Black => Player::create_black_player(p, c),
            _ => Player::create_white_player(p, c),
        }
    }

    fn create_black_player(p: PlayerType, c: Color) -> Self {
        let mut f = HashMap::with_capacity(16);
        let mut pos = Vec::new();
        // Pawns
        for bla in 1..9 {
            pos.push(Position::new(bla, 7))
        }
        f.insert("pawn".to_string(), pos);
        // King
        f.insert("king".to_string(), vec![Position::new(5, 8)]);
        // Queen
        f.insert("queen".to_string(), vec![Position::new(4, 8)]);
        // Bishops
        f.insert("bishop".to_string(), vec![Position::new(3, 8),
                                            Position::new(6, 8)]);
        // Knights
        f.insert("knight".to_string(), vec![Position::new(2, 8),
                                            Position::new(7, 8)]);
        // Rooks
        f.insert("rook".to_string(), vec![Position::new(1, 8),
                                          Position::new(8, 8)]);
        Player { ptype: p, color: c, figures: f }
    }

    fn create_white_player(p: PlayerType, c: Color) -> Self {
        let mut f = HashMap::with_capacity(16);
        let mut pos = Vec::new();
        // Pawns
        for bla in 1..9 {
            pos.push(Position::new(bla, 2))
        }
        f.insert("pawn".to_string(), pos);
        // King
        f.insert("king".to_string(), vec![Position::new(5, 1)]);
        // Queen
        f.insert("queen".to_string(), vec![Position::new(4, 1)]);
        // Bishops
        f.insert("bishop".to_string(), vec![Position::new(3, 1),
                                            Position::new(6, 1)]);
        // Knights
        f.insert("knight".to_string(), vec![Position::new(2, 1),
                                            Position::new(7, 1)]);
        // Rooks
        f.insert("rook".to_string(), vec![Position::new(1, 1),
                                          Position::new(8, 1)]);
        Player { ptype: p, color: c, figures: f }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn ptype(&self) -> PlayerType {
        self.ptype
    }

    pub fn set_ptype(&mut self, p: PlayerType) {
        self.ptype = p;
    }

    pub fn figures(&self) -> HashMap<String, Vec<Position>> {
        self.figures.clone()
    }

    pub fn king(&self) -> Position {
        self.figures.get("king").unwrap()[0]
    }

    // Returns a vector of possible moves for all figures of the player
    pub fn get_possible_moves(&self, board: &Board) -> Vec<(Position, Position)> {
        let mut moves = Vec::new();
        for (_, v) in self.figures.iter() {
            for i in 0..v.len() {
                for outer in 1..9 {
                    for inner in 1..9 {
                        if board.is_move_valid(v[i], Position::new(outer, inner)) {
                            moves.push((v[i], Position::new(outer, inner)));
                        }
                    }
                }
            }
        }
        moves
    }

    // If the player is not a human this returns a move
    pub fn get_ai_move(&mut self, board: &mut Board, other: &mut Player) -> (Position, Position) {
        if self.ptype != PlayerType::Human {
            return super::ai::get_move(board, self, other)
        }
        unreachable!()
    }

    // Move a figure
    pub fn move_figure(&mut self, before: Position, after: Position) {
        for mut v in self.figures.values_mut() {
            for i in 0..v.len() {
                if v[i] == before {
                    v[i] = after;
                    return
                }
            }
        }
    }

    // Capture a figure
    pub fn capture(&mut self, pos: Position, name: String) {
        if let Some(mut positions) = self.figures.get_mut(&name) {
            for i in 0..positions.len() {
                if positions[i] == pos {
                    positions.remove(i);
                    break;
                }
            }
        }
    }

    // Reverse a capture
    pub fn reverse_capture(&mut self, name: &str, pos: Position) {
        if let Some(mut v) = self.figures.get_mut(name) {
                v.push(pos);
        }
    }

    // Returns whether the king can be saved from checkmate in one move
    pub fn can_king_be_saved(&mut self, board: &mut Board, two: &Player) -> bool {
        for &elem in FIGURE_NAMES {
            let v = self.figures.get_mut(elem).unwrap();
            for pos in 0..v.len() {
                let before = v[pos];
                for i in 1..9 {
                    for j in 1..9 {
                        if board.is_move_valid(before, Position::new(i, j)) {
                            let mut tmp = Figure::new();
                            let mut reset = false;
                            if !board.is_empty(Position::new(i, j)) {
                                tmp = board.get_figure(Position::new(i, j))
                                           .unwrap()
                                           .as_figure(two.color());
                                reset = true;
                            }
                            board.move_figure(before, Position::new(i, j));
                            v[pos] = Position::new(i, j);

                            if !board.in_check(Position::new(i, j), two) {
                                board.move_figure(Position::new(i, j), before);
                                if reset {
                                    board.set_figure(Position::new(i, j), tmp);
                                }
                                v[pos] = before;
                                return true
                            } else {
                                board.move_figure(Position::new(i, j), before);
                                if reset {
                                    board.set_figure(Position::new(i, j), tmp);
                                }
                                v[pos] = before;
                            }
                        }
                    }
                }
            }
        }
        false
    }
}

impl ::std::clone::Clone for Player {
    fn clone(&self) -> Self {
        let mut f = HashMap::new();
        for (name, pos) in self.figures.iter() {
            f.insert(name.clone(), pos.clone());
        }
        Player{ figures: f, color: self.color, ptype: self.ptype }
    }

    fn clone_from(&mut self, source: &Self) {
        self.figures.clear();
        self.color = source.color;
        self.ptype = source.ptype;

        for (name, pos) in source.figures.iter() {
            self.figures.insert(name.clone(), pos.clone());
        }
    }
}
