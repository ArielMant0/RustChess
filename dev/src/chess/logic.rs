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

use std::hash::{Hash, Hasher};
use std::collections::HashMap;

use self::Color::*;
use self::Id::*;
use chess::player::Player;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8
}

impl Position {
    pub fn new(a: u8, b: u8) -> Self {
        Position{ x: a, y: b }
    }
}

impl ::std::cmp::Eq for Position {}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Id {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn
}

impl Id {

    pub fn as_figure(&self, c: Color) -> Figure {
        match *self {
            Id::King => Figure{ color: c, name: "king" },
            Id::Queen => Figure{ color: c, name: "queen" },
            Id::Rook => Figure{ color: c, name: "rook" },
            Id::Knight => Figure{ color: c, name: "knight" },
            Id::Bishop => Figure{ color: c, name: "bishop" },
            Id::Pawn => Figure{ color: c, name: "pawn" }
        }
    }

    pub fn from_fig(n: &str) -> Self {
        match n {
            "king" => King,
            "queen" => Queen,
            "rook" => Rook,
            "bishop" => Bishop,
            "knight" => Knight,
            "pawn" => Pawn,
            _ => unreachable!()
        }
    }

    pub fn name(&self) -> String {
        match *self {
            Id::King => String::from("king"),
            Id::Queen => String::from("queen"),
            Id::Rook => String::from("rook"),
            Id::Knight => String::from("knight"),
            Id::Bishop => String::from("bishop"),
            Id::Pawn => String::from("pawn")
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BoardField {
    color: Color,
    figure: Option<Id>
}

impl BoardField {

    pub fn set_occupied(&mut self, f: Id, c: Color) {
        self.figure = Some(f);
        self.color = c;
    }

    pub fn get_field_color(pos: Position) -> Color {
        match (pos.x, pos.y) {
            (first @ 1...8, 1...8) if first % 2 == 1 => {
                if pos.y % 2 == 0 {
                    White
                } else {
                    Black
                }
            },
            (first @ 1...8, 1...8) if first % 2 == 0 => {
                if pos.y % 2 == 0 {
                    Black
                } else {
                    White
                }
            },
            _ => unreachable!()
        }
    }

    pub fn set_empty(&mut self, pos: Position) {
        self.figure = None;
        self.color = BoardField::get_field_color(pos);
    }

    pub fn figure(&self) -> Option<Id> {
        self.figure
    }

    pub fn is_empty(&self) -> bool {
        self.figure.is_none()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Black,
    White
}

#[derive(Debug)]
pub struct Figure {
    color: Color,
    name: &'static str,
}

impl Figure {

    pub fn new() -> Self {
        Figure{ name: "pawn", color: White}
    }

    pub fn valid_move(&self, board: &Board, from: Position, to: Position) -> bool {
        match self.name {
            "pawn" => self.pawn_move(board, from, to),
            "bishop" => self.bishop_move(board, from, to),
            "knight" => self.knight_move(board, from, to),
            "rook" => self.rook_move(board, from, to),
            "king" => self.king_move(board, from, to),
            "queen" => self.queen_move(board, from, to),
            _ => unreachable!(),
        }
    }

    fn straight(board: &Board, from: Position, to: Position) -> bool {
        from.x == to.x && Figure::way_is_clear(board, 0, from, to)
    }

    fn diagonal(board: &Board, from: Position, to: Position) -> bool {
        (1u8..9).any(|x| {
            // right and up or down
            Figure::way_is_clear(board, 1, from, to) &&
            ((from.x + x == to.x &&
            (from.y + x == to.y || from.y.checked_sub(x).is_some() && from.y - x == to.y)) ||
            // left and up or down
            (from.x.checked_sub(x).is_some() && from.x - x == to.x &&
            (from.y + x == to.y || from.y.checked_sub(x).is_some() && from.y - x == to.y)))
        })
    }

    fn sideways(board: &Board, from: Position, to: Position) -> bool {
        from.y == to.y && Figure::way_is_clear(board, 2, from, to)
    }

    fn way_is_clear(board: &Board, direction: u8, from: Position, to: Position) -> bool {
        match direction {
            0 => {
                if to.y > from.y {
                    (from.y+1..to.y).all(|x| board.is_empty(Position::new(from.x, x)))
                } else if to.y < from.y {
                    (to.y+1..from.y).all(|x| board.is_empty(Position::new(from.x, x)))
                } else {
                    false
                }
            },
            1 => {
                if to.x > from.x && to.y > from.y {
                    let diff = to.x - from.x;
                    (1..diff).all(|x| board.is_empty(Position::new(from.x + x, from.y + x)))
                } else if to.x > from.x && to.y < from.y {
                    let diff = to.x - from.x;
                    (1..diff).all(|x| board.is_empty(Position::new(from.x + x, from.y - x)))
                } else if to.x < from.x && to.y > from.y {
                    let diff = from.x - to.x;
                    (1..diff).all(|x| board.is_empty(Position::new(from.x - x, from.y + x)))
                } else if to.x < from.x && to.y < from.y {
                    let diff = from.x - to.x;
                    (1..diff).all(|x| board.is_empty(Position::new(from.x - x, from.y - x)))
                } else {
                    false
                }
            },
            2 => {
                if to.x > from.x {
                    (from.x+1..to.x).all(|x| board.is_empty(Position::new(x, from.y)))
                } else if to.x < from.x {
                    (to.x+1..from.x).all(|x| board.is_empty(Position::new(x, from.y)))
                } else {
                    false
                }
            },
            _ => unreachable!()
        }
    }

    fn pawn_move(&self, board: &Board, from: Position, to: Position) -> bool {
        if board.is_empty(to) {
            return match self.color {
                Color::Black => {
                    match (from.x, from.y) {
                        (_, 7) => {
                            Figure::straight(board, from, to) &&
                            (5..7).any(|x| x == to.y)
                        },
                        _ => {
                            Figure::straight(board, from, to) &&
                            from.y.checked_sub(1).is_some() &&
                            from.y - 1 == to.y
                        }
                    }
                },
                _ => {
                    match (from.x, from.y) {
                        (_, 2) => {
                            Figure::straight(board, from, to) &&
                            (3..5).any(|x| x == to.y)
                        },
                        _ => {
                            Figure::straight(board, from, to) &&
                            from.y + 1 == to.y
                        }
                    }
                }
            }
        } else {
            board.get_figure_color(to).unwrap() != self.color &&
            (from.x - 1 == to.x || from.x + 1 == to.x) &&
            match self.color {
                Color::Black => from.y - 1 == to.y,
                _ => from.y + 1 == to.y
            }
        }
    }

    fn bishop_move(&self, board: &Board, from: Position, to: Position) -> bool {
        let mut clash = true;
        if let Some(f) = board.get_figure_color(to){
            clash = f != self.color;
        }

        clash && Figure::diagonal(board, from, to)
    }

    fn rook_move(&self, board: &Board, from: Position, to: Position) -> bool {
        let mut clash = true;
        if let Some(f) = board.get_figure_color(to) {
            clash = f != self.color;
        }

        clash && (Figure::straight(board, from, to) || Figure::sideways(board, from, to))
    }

    fn knight_move(&self, board: &Board, from: Position, to: Position) -> bool {
        let mut clash = true;
        if let Some(f) = board.get_figure_color(to) {
            clash = f != self.color;
        }

        let straight = {
            if from.y + 2 == to.y || (from.y.checked_sub(2).is_some() && from.y - 2 == to.y) {
               from.x + 1 == to.x || (from.x.checked_sub(1).is_some() && from.x - 1 == to.x)
            } else if from.y + 1 == to.y || (from.y.checked_sub(1).is_some() && from.y - 1 == to.y) {
                from.x + 2 == to.x || (from.x.checked_sub(2).is_some() && from.x - 2 == to.x)
            } else {
                false
            }
        };

        let sideways = {
            if from.x + 2 == to.x || (from.x.checked_sub(2).is_some() && from.x - 2 == to.x) {
                from.y + 1 == to.y || (from.y.checked_sub(1).is_some() && from.y - 1 == to.y)
            } else if from.x + 1 == to.x || (from.x.checked_sub(1).is_some() && from.x - 1 == to.x) {
                from.y + 2 == to.y || (from.y.checked_sub(2).is_some() && from.y - 2 == to.y)
            } else {
                false
            }
        };

        clash && (straight || sideways)
    }

    fn king_move(&self, board: &Board, from: Position, to: Position) -> bool {
        let mut clash = true;
        if let Some(f) = board.get_figure_color(to) {
            clash = f != self.color;
        }

        let direction = {
            //  forward or backward
            if from.x == to.x {
                from.y + 1 == to.y || (from.y.checked_sub(1).is_some() && from.y - 1 == to.y)
            //  right or left
            } else if from.y == to.y {
                from.x + 1 == to.x || (from.x.checked_sub(1).is_some() && from.x - 1 == to.x)
            // diagonal right
            } else if from.x + 1 == to.x {
                from.y + 1 == to.y || (from.y.checked_sub(1).is_some() && from.y - 1 == to.y)
            // diagonal left
            } else if from.x.checked_sub(1).is_some() && from.x - 1 == to.x {
                from.y + 1 == to.y || (from.y.checked_sub(1).is_some() && from.y - 1 == to.y)
            } else {
                false
            }
        };

        clash && direction
    }

    fn queen_move(&self, board: &Board, from: Position, to: Position) -> bool {
        let mut clash = true;
        if let Some(f) = board.get_figure_color(to) {
            clash = f != self.color;
        }

        clash && (Figure::straight(board, from, to) && !(Figure::sideways(board, from, to) || Figure::diagonal(board, from, to)))
        && (Figure::sideways(board, from, to) && !(Figure::straight(board, from, to) || Figure::diagonal(board, from, to)))
        && (Figure::diagonal(board, from, to) && !(Figure::straight(board, from, to) || Figure::sideways(board, from, to)))
    }
}

impl PartialEq for Figure {
    fn eq(&self, other: &Figure) -> bool {
        self.color == other.color && self.name == other.name
    }
}

#[derive(Debug)]
pub struct Board {
    fields: HashMap<Position, BoardField>
}

impl Board {
    /// return new board where every field is empty
    pub fn new() -> Self {
        let mut f = HashMap::with_capacity(64);
        for outer in 1..9 {
            for inner in 1..9 {
                let _ = match (outer, inner) {
                    a @ (_, 2) => f.insert(Position::new(a.0, a.1), BoardField{ color: White, figure: Some(Pawn)}),
                    a @ (1, 1) => f.insert(Position::new(a.0, a.1), BoardField{ color: White, figure: Some(Rook)}),
                    a @ (8, 1) => f.insert(Position::new(a.0, a.1), BoardField{ color: White, figure: Some(Rook)}),
                    a @ (2, 1) => f.insert(Position::new(a.0, a.1), BoardField{ color: White, figure: Some(Knight)}),
                    a @ (7, 1) => f.insert(Position::new(a.0, a.1), BoardField{ color: White, figure: Some(Knight)}),
                    a @ (3, 1) => f.insert(Position::new(a.0, a.1), BoardField{ color: White, figure: Some(Bishop)}),
                    a @ (6, 1) => f.insert(Position::new(a.0, a.1), BoardField{ color: White, figure: Some(Bishop)}),
                    a @ (5, 1) => f.insert(Position::new(a.0, a.1), BoardField{ color: White, figure: Some(King)}),
                    a @ (4, 1) => f.insert(Position::new(a.0, a.1), BoardField{ color: White, figure: Some(Queen)}),
                    // Black figures
                    a @ (_, 7) => f.insert(Position::new(a.0, a.1), BoardField{ color: Black, figure: Some(Pawn)}),
                    a @ (1, 8) => f.insert(Position::new(a.0, a.1), BoardField{ color: Black, figure: Some(Rook)}),
                    a @ (8, 8) => f.insert(Position::new(a.0, a.1), BoardField{ color: Black, figure: Some(Rook)}),
                    a @ (2, 8) => f.insert(Position::new(a.0, a.1), BoardField{ color: Black, figure: Some(Knight)}),
                    a @ (7, 8) => f.insert(Position::new(a.0, a.1), BoardField{ color: Black, figure: Some(Knight)}),
                    a @ (3, 8) => f.insert(Position::new(a.0, a.1), BoardField{ color: Black, figure: Some(Bishop)}),
                    a @ (6, 8) => f.insert(Position::new(a.0, a.1), BoardField{ color: Black, figure: Some(Bishop)}),
                    a @ (5, 8) => f.insert(Position::new(a.0, a.1), BoardField{ color: Black, figure: Some(King)}),
                    a @ (4, 8) => f.insert(Position::new(a.0, a.1), BoardField{ color: Black, figure: Some(Queen)}),
                    _ => {
                        let c = {
                            if outer % 2 == 1 && inner % 2 == 0 || outer % 2 == 0 && inner % 2 == 1 {
                                White
                            } else {
                                Black
                            }
                        };
                        f.insert(Position::new(outer, inner), BoardField{ color: c, figure: None })
                    }
                };
            }
        }
        Board{ fields: f }
    }

    /// get the symbol of the board at position 'pos'
    pub fn get_figure(&self, pos: Position) -> Option<Id> {
        if let Some(f) = self.fields.get(&pos) {
            return f.figure
        }
        unreachable!()
    }

    pub fn is_move_valid(&self, from: Position, to: Position) -> bool {
        if let Some(pos) = self.fields.get(&from) {
            if let Some(fig) = pos.figure() {
                return fig.as_figure(self.get_figure_color(from).unwrap())
                          .valid_move(self, from, to)
            }
        }
        unreachable!()
    }

    pub fn set_figure(&mut self, pos: Position, fig: Figure) {
        self.fields.insert(pos,
            BoardField{ color: fig.color, figure: Some(Id::from_fig(fig.name)) });
    }

    /// get the Color of the figure at position 'pos'
    pub fn get_figure_color(&self, pos: Position) -> Option<Color> {
        if let Some(f) = self.fields.get(&pos) {
            if self.is_empty(pos) {
                return None
            } else {
                return Some(f.color)
            }
        }
        unreachable!()
    }

    /// move a figure on the board
    pub fn move_figure(&mut self, before: Position, after: Position) {
        let col = self.fields.get(&before).unwrap().color;
        let tmp = self.fields.get(&before).unwrap().figure.unwrap();

        if let Some(f) = self.fields.get_mut(&before) {
            f.set_empty(before);
        }

        if let Some(x) = self.fields.get_mut(&after) {
            x.set_occupied(tmp, col);
        }
    }

    /// return whether field at pos is empty or not
    pub fn is_empty(&self, pos: Position) -> bool {
        self.fields.get(&pos).unwrap().is_empty()
    }

    pub fn in_check(&self, king: Position, opponent: &Player) -> bool {
        if opponent.figures().values().any(|pos|
        {
            for p in pos {
                if self.fields.get(&p).unwrap()
                                      .figure
                                      .unwrap()
                                      .as_figure(opponent.color())
                                      .valid_move(self, *p, king)
                {
                    return true
                }
            }
            false
        }) { true } else { false}
    }

    /// return wether one if the players' has won or a king is just in check
    pub fn checkmate(&mut self, one: &mut Player, two: &mut Player) -> bool {
        if self.in_check(one.king(), two) {
            return !one.can_king_be_saved(self, two)
        }

        if self.in_check(two.king(), one) {
            return !two.can_king_be_saved(self, one)
        }

        false
    }
}

impl ::std::clone::Clone for Board {
    fn clone(&self) -> Self {
        let mut f = HashMap::new();
        for (&pos, field) in self.fields.iter() {
            f.insert(pos, BoardField{ color: field.color, figure: field.figure });
        }
        Board{ fields: f }
    }

    fn clone_from(&mut self, source: &Self) {
        self.fields.clear();
        for (&pos, field) in source.fields.iter() {
            self.fields.insert(pos, BoardField{ color: field.color, figure: field.figure });
        }
    }
}
