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

use self::Color::*;
use self::Figure::*;
use chess::player::Player;

/// Positions on the Board
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8
}

impl Position {
    /// Construct a new position, may only be a in range from 0 to 7 (inclusive)
    pub fn new(a: u8, b: u8) -> Self {
        if Position::is_pos(a, b) {
            return Position{ x: a, y: b }
        }

        unreachable!()
    }

    /// Returns whether 'p' is a valid position
    pub fn is_pos(a: u8, b: u8) -> bool {
        match (a, b) {
            (0...7, 0...7) => true,
            _ => false
        }
    }
}

impl ::std::fmt::Display for Position {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        // Needs match if more errors are possible
        write!(f, "({}{})", self.x, self.x)
    }
}

/// Figures for all possible figures
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Figure {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn
}

impl ::std::fmt::Display for Figure {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.short())
    }
}

impl Figure {

    /// Check whether move is valid according to figure type
    pub fn valid_move(&self, board: &Board, from: Position, to: Position, color: &Color) -> bool {
        match *self {
            Pawn => self.pawn_move(board, from, to, color),
            Bishop => self.bishop_move(board, from, to, color),
            Knight => self.knight_move(board, from, to, color),
            Rook => self.rook_move(board, from, to, color),
            King => self.king_move(board, from, to, color),
            Queen => self.queen_move(board, from, to, color)
        }
    }

    /// Returns whether the move's direction is straight
    fn straight(board: &Board, from: Position, to: Position) -> bool {
        from.x == to.x && Figure::way_is_clear(board, 0, from, to)
    }

    /// Returns whether the move's direction is diagonal
    fn diagonal(board: &Board, from: Position, to: Position) -> bool {
        (0u8..8).any(|x| {
            // right and up or down
            ((from.x + x == to.x &&
            (from.y + x == to.y || from.y.checked_sub(x).is_some() && from.y - x == to.y)) ||
            // left and up or down
            (from.x.checked_sub(x).is_some() && from.x - x == to.x &&
            (from.y + x == to.y || from.y.checked_sub(x).is_some() && from.y - x == to.y)))
        }) && Figure::way_is_clear(board, 1, from, to)
    }

    /// Returns whether the move's direction is sideways
    fn sideways(board: &Board, from: Position, to: Position) -> bool {
        from.y == to.y && Figure::way_is_clear(board, 2, from, to)
    }

    /// Returns whether there are no other figures in the way
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
                    (1..diff).all(|x| from.y.checked_sub(x).is_some() && board.is_empty(Position::new(from.x + x, from.y - x)))
                } else if to.x < from.x && to.y > from.y {
                    let diff = from.x - to.x;
                    (1..diff).all(|x| from.x.checked_sub(x).is_some() && board.is_empty(Position::new(from.x - x, from.y + x)))
                } else if to.x < from.x && to.y < from.y {
                    let diff = from.x - to.x;
                    (1..diff).all(|x| from.x.checked_sub(x).is_some() && board.is_empty(Position::new(from.x - x, from.y - x)))
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

    /// Tests if move is valid for a pawn
    fn pawn_move(&self, board: &Board, from: Position, to: Position, color: &Color) -> bool {
        if board.is_empty(to) {
            match *color {
                Black => {
                    match (from.x, from.y) {
                        (_, 6) => {
                            Figure::straight(board, from, to) &&
                            (4..6).any(|x| x == to.y)
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
                        (_, 1) => {
                            Figure::straight(board, from, to) &&
                            (2..4).any(|x| x == to.y)
                        },
                        _ => {
                            Figure::straight(board, from, to) &&
                            from.y + 1 == to.y
                        }
                    }
                }
            }
        } else {
            board.get_figure_color(to).unwrap() != *color &&
            (from.x + 1 == to.x || (from.x.checked_sub(1).is_some() && from.x - 1 == to.x)) &&
            match *color {
                Black => from.y.checked_sub(1).is_some() && from.y - 1 == to.y,
                _ => from.y + 1 == to.y
            }
        }
    }

    /// Tests if move is valid for a bishop
    fn bishop_move(&self, board: &Board, from: Position, to: Position, color: &Color) -> bool {
        let mut opposite = true;
        if let Some(f) = board.get_figure_color(to){
            opposite = f != *color;
        }

        opposite && Figure::diagonal(board, from, to)
    }

    /// Tests if move is valid for a rook
    fn rook_move(&self, board: &Board, from: Position, to: Position, color: &Color) -> bool {
        let mut opposite = true;
        if let Some(f) = board.get_figure_color(to) {
            opposite = f != *color;
        }

        opposite && ((Figure::straight(board, from, to) && !Figure::sideways(board, from, to))
        || (!Figure::straight(board, from, to) && Figure::sideways(board, from, to)))
    }

    /// Tests if move is valid for a knight
    fn knight_move(&self, board: &Board, from: Position, to: Position, color: &Color) -> bool {
        let mut opposite = true;
        if let Some(f) = board.get_figure_color(to) {
            opposite = f != *color;
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

        opposite && (straight || sideways)
    }

    /// Tests if move is valid for a king
    fn king_move(&self, board: &Board, from: Position, to: Position, color: &Color) -> bool {
        let mut opposite = true;
        if let Some(f) = board.get_figure_color(to) {
            opposite = f != *color;
        }

        let direction = {
            // forward or backward
            if from.x == to.x {
                from.y + 1 == to.y || (from.y.checked_sub(1).is_some() && from.y - 1 == to.y)
            // right or left
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

        opposite && direction
    }

    /// Tests if move is valid for a queen
    fn queen_move(&self, board: &Board, from: Position, to: Position, color: &Color) -> bool {
        let mut opposite = true;
        if let Some(f) = board.get_figure_color(to) {
            opposite = f != *color;
        }

        // Check only ONE of the possible directions is valid (TODO XOR in rust?)
        opposite && ((Figure::straight(board, from, to) && !(Figure::sideways(board, from, to) || Figure::diagonal(board, from, to)))
        || (Figure::sideways(board, from, to) && !(Figure::straight(board, from, to) || Figure::diagonal(board, from, to)))
        || (Figure::diagonal(board, from, to) && !(Figure::straight(board, from, to) || Figure::sideways(board, from, to))))
    }

    /// Constructs a figure from a name
    pub fn from_name(n: &str) -> Self {
        match n {
            "rook" => Rook,
            "queen" => Queen,
            "king" => King,
            "pawn" => Pawn,
            "knight" => Knight,
            "bishop" => Bishop,
            _ => unreachable!()
        }
    }

    /// Returns the figure's name
    pub fn name(&self) -> String {
        match *self {
            King => String::from("king"),
            Queen => String::from("queen"),
            Rook => String::from("rook"),
            Knight => String::from("knight"),
            Bishop => String::from("bishop"),
            Pawn => String::from("pawn")
        }
    }

    /// Return the short version of a figure's name
    fn short(&self) -> String {
        match *self {
            King => String::from("Ki"),
            Queen => String::from("Qu"),
            Rook => String::from("Ro"),
            Knight => String::from("Kn"),
            Bishop => String::from("Bi"),
            Pawn => String::from("Pa")
        }
    }
}

/// Field on the Board
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Field {
    color: Color,
    figure: Option<Figure>
}

/// Print a field, for debug purposes
impl ::std::fmt::Display for Field {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        if self.is_empty() {
            write!(f, "   ")
        } else {
            let c = if self.color == Color::White {'W'} else {'B'};
            write!(f, "{}{}", c, self.figure.unwrap())
        }
    }
}

impl Field {
    /// Set a figure on this field
    pub fn set_occupied(&mut self, f: Figure, c: Color) {
        self.figure = Some(f);
        self.color = c;
    }

    /// Get correct color for an empty field
    pub fn get_field_color(pos: Position) -> Color {
        if pos.x % 2 == 0 {
            if pos.y % 2 == 1 {
                White
            } else {
                Black
            }
        } else {
            if pos.y % 2 == 0 {
                White
            } else {
                Black
            }
        }
    }

    /// Remove figure from this field and update color
    pub fn set_empty(&mut self, pos: Position) {
        self.figure = None;
        self.color = Field::get_field_color(pos);
    }

    /// Get this field's figure
    pub fn get_figure(&self) -> Option<Figure> {
        self.figure
    }

    /// Return whether this field is empty
    pub fn is_empty(&self) -> bool {
        self.figure.is_none()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Black,
    White
}

/// Overload '!' operator for comfort
impl ::std::ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self {
        if self == White {
            Black
        } else {
            White
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    fields: Vec<Vec<Field>>
}

impl Board {
    /// Construct new board with standar figure positions
    pub fn new() -> Self {
        let mut f = Vec::new();
        for outer in 0u8..8 {
            let mut nested = Vec::new();
            for inner in 0u8..8 {
                match (inner, outer) {
                    // White Figures
                    (_, 1) => nested.push(Field{ color: White, figure: Some(Pawn) }),
                    (0, 0) => nested.push(Field{ color: White, figure: Some(Rook) }),
                    (7, 0) => nested.push(Field{ color: White, figure: Some(Rook) }),
                    (1, 0) => nested.push(Field{ color: White, figure: Some(Knight) }),
                    (6, 0) => nested.push(Field{ color: White, figure: Some(Knight) }),
                    (2, 0) => nested.push(Field{ color: White, figure: Some(Bishop) }),
                    (5, 0) => nested.push(Field{ color: White, figure: Some(Bishop) }),
                    (4, 0) => nested.push(Field{ color: White, figure: Some(King) }),
                    (3, 0) => nested.push(Field{ color: White, figure: Some(Queen) }),
                    // Black Figures
                    (_, 6) => nested.push(Field{ color: Black, figure: Some(Pawn) }),
                    (7, 7) => nested.push(Field{ color: Black, figure: Some(Rook) }),
                    (0, 7) => nested.push(Field{ color: Black, figure: Some(Rook) }),
                    (1, 7) => nested.push(Field{ color: Black, figure: Some(Knight) }),
                    (6, 7) => nested.push(Field{ color: Black, figure: Some(Knight) }),
                    (2, 7) => nested.push(Field{ color: Black, figure: Some(Bishop) }),
                    (5, 7) => nested.push(Field{ color: Black, figure: Some(Bishop) }),
                    (4, 7) => nested.push(Field{ color: Black, figure: Some(King) }),
                    (3, 7) => nested.push(Field{ color: Black, figure: Some(Queen) }),
                    // Empty Fields
                    _ => nested.push(Field{ color: Field::get_field_color(Position::new(outer, inner)), figure: None }),
                }
            }
            f.push(nested);
        }

        Board{ fields: f }
    }

    /// Get the figure at position 'pos'
    pub fn get_figure(&self, pos: Position) -> Option<Figure> {
        self[pos].get_figure()
    }

    /// Check if move from 'from' to 'to' is valid
    pub fn is_move_valid(&mut self, from: Position, to: Position, active: &mut Player, inactive: &mut Player) -> bool {
        if let Some(fig) = self[from].get_figure() {
            return fig.valid_move(self, from, to, &self[from].color) &&
                   !self.simulate_check(from, to, active, inactive, true)
        }

        // If we got here there was no figure at 'from' which should not be the case
        unreachable!()
    }

    pub fn simulate_check(&mut self, from: Position, to: Position, active: &mut Player, inactive: &mut Player, king: bool) -> bool {
        let mut reverse = false;
        let mut name = String::new();

        // Check if there is another figure at 'to' and capture it if there is
        if !self.is_empty(to) {
            name =  self.get_figure(to).unwrap().name();
            inactive.capture(name.clone(), to);
            reverse = true;
        }
        // Move figure in board and active player
        self.move_figure(from, to);
        active.move_figure(from, to);

        // If the active player's king/figure is not in check return true
        let result = if king {self.in_check(active.king(), inactive)} else {self.in_check(to, inactive)};

        // Reverse move
        self.move_figure(to, from);
        active.move_figure(to, from);
        // Reverse capture if it happened
        if reverse {
            inactive.reverse_capture(name.clone(), to);
            self.set_figure(to, Figure::from_name(&name), inactive.color());
        }

        result
    }

    /// Set a figure at position on the board
    pub fn set_figure(&mut self, pos: Position, fig: Figure, col: Color) {
        self[pos] = Field{ color: col, figure: Some(fig) };
    }

    /// Get the color of the figure at position 'pos'
    pub fn get_figure_color(&self, pos: Position) -> Option<Color> {
        if self.is_empty(pos) {
            None
        } else {
            return Some(self[pos].color)
        }
    }

    /// Tests whether a move results in a capture
    pub fn is_capture_move(&self, from: Position, to: Position) -> bool {
        !self[to].is_empty() && self[to].color != self[from].color
    }

    /// Move a figure on the board
    pub fn move_figure(&mut self, before: Position, after: Position) {
        let col = self[before].color;
        let tmp = self[before].get_figure().unwrap();

        // Set new positions on the board
        self[before].set_empty(before);
        self[after].set_occupied(tmp, col);
    }

    /// Return whether field at position 'pos' is empty or not
    pub fn is_empty(&self, pos: Position) -> bool {
        self[pos].is_empty()
    }

    /// Return whether King at Position 'king' is in check
    pub fn in_check(&self, king: Position, opponent: &Player) -> bool {

        // For all figures of the opponent check whether
        // they can make a valid move to the king's position
        opponent.figures
                .values()
                .any(|pos| {
                    pos.iter().any(|&x|
                        if let Some(tmp) = self[x].get_figure() {
                            tmp.valid_move(self, x, king, &opponent.color())
                        } else {
                            unreachable!()
                        })
                })
    }

    /// Return wether a king is in checkmate
    pub fn checkmate(&mut self, one: &mut Player, two: &mut Player) -> bool {
        // Check if first king is in checkmate
        if self.in_check(one.king(), two) {
            return !one.can_king_be_saved(self, two)
        }

        // Check if second king is in checkmate
        if self.in_check(two.king(), one) {
            return !two.can_king_be_saved(self, one)
        }

        false
    }
}

/// Implement indexing with positions for ease of use
impl ::std::ops::Index<Position> for Board {
    type Output = Field;

    fn index(&self, pos: Position) -> &Self::Output {
        &self.fields[pos.y as usize][pos.x as usize]
    }
}

impl ::std::ops::IndexMut<Position> for Board {

    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        &mut self.fields[pos.y as usize][pos.x as usize]
    }
}

/// Implement indexing with u8-tuples for ease of use
impl ::std::ops::Index<(u8, u8)> for Board {
    type Output = Field;

    fn index(&self, pos: (u8, u8)) -> &Self::Output {
        &self.fields[pos.1 as usize][pos.0 as usize]
    }
}

impl ::std::ops::IndexMut<(u8, u8)> for Board {

    fn index_mut(&mut self, pos: (u8, u8)) -> &mut Self::Output {
        &mut self.fields[pos.1 as usize][pos.0 as usize]
    }
}

impl ::std::fmt::Display for Board {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        try!(write!(f, "\n  | a | b | c | d | e | f | g | h |\n"));
        try!(write!(f, "--|---|---|---|---|---|---|---|---|--\n"));
        for outer in (0u8..8).rev() {
            try!(write!(f, "{} |{}|{}|{}|{}|{}|{}|{}|{}| \n", outer + 1,
                self[(0, outer)],
                self[(1, outer)],
                self[(2, outer)],
                self[(3, outer)],
                self[(4, outer)],
                self[(5, outer)],
                self[(6, outer)],
                self[(7, outer)]));
        }
        write!(f, "--|---|---|---|---|---|---|---|---|--\n")
    }
}
