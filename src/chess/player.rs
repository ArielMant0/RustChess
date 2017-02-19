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

use chess::logic::{Color, Board, Position};

/// Types the player can have
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayerType {
    Human,
    Dumb,
    Smart
}

/// The player
#[derive(Debug)]
pub struct Player {
    ptype: PlayerType,
    color: Color,
    castling: [bool; 3],
    pub figures: HashMap<String, Vec<Position>>
}

impl Player {
    /// Returns an instance of a Player with the given PlayerType
    pub fn new(p: PlayerType, c: Color) -> Self {
        match c {
            Color::Black => Player::create_black_player(p, c),
            _ => Player::create_white_player(p, c),
        }
    }

    /// Create a new black player
    fn create_black_player(p: PlayerType, c: Color) -> Self {
        let mut f = HashMap::with_capacity(16);
        let mut pos = Vec::new();
        // Pawns
        for bla in 0..8 {
            pos.push(Position::new(bla, 6))
        }
        f.insert("pawn".to_string(), pos);
        // King
        f.insert("king".to_string(), vec![Position::new(4, 7)]);
        // Queen
        f.insert("queen".to_string(), vec![Position::new(3, 7)]);
        // Bishops
        f.insert("bishop".to_string(), vec![Position::new(2, 7), Position::new(5, 7)]);
        // Knights
        f.insert("knight".to_string(), vec![Position::new(1, 7), Position::new(6, 7)]);
        // Rooks
        f.insert("rook".to_string(), vec![Position::new(0, 7), Position::new(7, 7)]);

        Player { ptype: p, color: c, figures: f, castling: [true, true, true] }
    }

    /// Create a new white player
    fn create_white_player(p: PlayerType, c: Color) -> Self {
        let mut f = HashMap::with_capacity(16);
        let mut pos = Vec::new();
        // Pawns
        for bla in 0..8 {
            pos.push(Position::new(bla, 1))
        }
        f.insert("pawn".to_string(), pos);
        // King
        f.insert("king".to_string(), vec![Position::new(4, 0)]);
        // Queen
        f.insert("queen".to_string(), vec![Position::new(3, 0)]);
        // Bishops
        f.insert("bishop".to_string(), vec![Position::new(2, 0), Position::new(5, 0)]);
        // Knights
        f.insert("knight".to_string(), vec![Position::new(1, 0), Position::new(6, 0)]);
        // Rooks
        f.insert("rook".to_string(), vec![Position::new(0, 0), Position::new(7, 0)]);

        Player { ptype: p, color: c, figures: f, castling: [true, true, true] }
    }

    /// Return player color
    pub fn color(&self) -> Color {
        self.color
    }

    /// Return player type
    pub fn ptype(&self) -> PlayerType {
        self.ptype
    }

    /// Set player type
    pub fn set_ptype(&mut self, p: PlayerType) {
        self.ptype = p;
    }

    pub fn upgrade_pawn(&mut self, pos: Position) {
        self.capture("pawn".to_string(), pos);

        let mut found = false;
        if let Some(mut positions) = self.figures.get_mut("queen") {
            positions.push(pos);
            found = true;
        }

        if !found {
            self.figures.insert("queen".to_string(), vec![pos]);
        }
    }

    /// Return the player's king which should always be there because one
    /// cannot actually 'capture' a king
    pub fn king(&self) -> Position {
        self.figures.get("king").unwrap()[0]
    }

    /// Returns a vector of possible moves for all figures of the player
    pub fn get_possible_moves(&mut self, board: &mut Board, opponent: &mut Player) -> Vec<(Position, Position)> {
        let mut moves = Vec::new();

        for v in self.figures.values() {
            for i in 0..v.len() {
                for outer in 0..8 {
                    for inner in 0..8 {
                        let try = Position::new(inner, outer);
                        if board.is_move_valid(v[i], try, &mut self.clone(), opponent) {
                            moves.push((v[i], try));
                        }
                    }
                }
            }
        }
        moves
    }

    /// If the player is an AI this returns a valid move
    pub fn get_ai_move(&self, board: &Board, other: &Player) -> (Position, Position) {
        return super::ai::get_move(board, self, other);
    }

    /// Move a figure from 'before' to 'after'
    pub fn move_figure(&mut self, before: Position, after: Position) {
        for mut v in self.figures.values_mut() {
            for i in 0..v.len() {
                if v[i] == before {
                    v[i] = after;
                    return
                }
            }
        }

        unreachable!()
    }

    /// Capture a figure
    pub fn capture(&mut self, name: String, pos: Position) {
        let mut delete = false;

        if let Some(mut positions) = self.figures.get_mut(&name) {
            for i in 0..positions.len() {
                if positions[i] == pos && positions.len() > 1 {
                    positions.remove(i);
                    break;
                } else if positions[i] == pos {
                    delete = true;
                    break;
                }
            }
        } else {
            unreachable!()
        }

        if delete {
            self.figures.remove(&name);
        }
    }

    /// Reverse a capture
    pub fn reverse_capture(&mut self, name: String, pos: Position) {
        let mut found = false;
        if let Some(mut v) = self.figures.get_mut(&name) {
            v.push(pos);
            found = true;
        }

        if !found {
            self.figures.insert(name, vec![pos]);
        }
    }

    /// Returns whether the player's king can be saved from checkmate in one move
    pub fn can_king_be_saved(&mut self, board: &mut Board, two: &mut Player) -> bool {
        self.get_possible_moves(board, two).len() > 0
    }
}

impl ::std::fmt::Display for Player {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        for (name, v) in self.figures.iter() {
            try!(write!(f, "{}: {:?}\n", name, v));
        }
        write!(f, "\n")
    }
}

impl ::std::clone::Clone for Player {
    fn clone(&self) -> Self {
        let mut f = HashMap::new();
        for (name, pos) in self.figures.iter() {
            f.insert(name.clone(), pos.clone());
        }
        Player{ figures: f, color: self.color, ptype: self.ptype, castling: self.castling }
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
