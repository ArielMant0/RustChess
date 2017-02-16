pub mod gameplay;
pub mod player;
pub mod ai;

use std::fmt::{Formatter, Display, Error};
use std::collections::HashMap;

use self::Colour::*;
use self::Id::*;
use self::player::Player;

pub const OFFSET: u8 = 96;

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
    pub fn as_figure(&self, c: Colour) -> Figure {
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
    color: Colour,
    figure: Option<Id>
}

impl BoardField {

    pub fn is_pos(a: char, b: u8) -> bool {
        match (a, b) {
            ('a' ... 'h', 1 ... 8) => true,
            _ => false
        }
    }

    pub fn set_occupied(&mut self, f: Id, c: Colour) {
        self.figure = Some(f);
        self.color = c;
    }

    pub fn get_field_color(pos: (char, u8)) -> Colour {
        match pos {
            ('a', _) | ('c', _) | ('e', _) | ('g', _) => {
                if pos.1 % 2 == 0 {
                    White
                } else {
                    Black
                }
            },
            ('b', _) | ('d', _) | ('f', _) | ('h', _) => {
                if pos.1 % 2 == 0 {
                    Black
                } else {
                    White
                }
            },
            _ => unreachable!()
        }
    }

    pub fn set_empty(&mut self, pos: (char, u8)) {
        self.figure = None;
        self.color = BoardField::get_field_color(pos);
    }

    pub fn figure(&self) -> Option<Id> {
        self.figure
    }

    pub fn is_empty(&self) -> bool {
        self.figure.is_none()
    }

    fn padding(&self) -> String {
        let mut s = String::new();
        s.push(' ');
        if self.color == Colour::White {
            return String::from("\u{2588}\u{2588}\u{2588}")
        } else {
            s.push(' ')
        }
        s.push(' ');
        s
    }
}

impl Display for BoardField {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if self.is_empty() {
            write!(f, "{}", self.padding())
        } else {
            write!(f, "{}", self.figure.unwrap().as_figure(self.color))
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Colour {
    Black,
    White
}

impl Colour {
    pub fn short(&self) -> char {
        match *self{
            Colour::Black => 'B',
            _ => 'W',
        }
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let c = match *self{
            Colour::Black => "Black",
            _ => "White",
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
pub struct Figure {
    color: Colour,
    name: &'static str,
}

impl Figure {

    pub fn new() -> Self {
        Figure{ name: "pawn", color: White}
    }

    pub fn valid_move(&self, board: &Board, from: (char, u8), to: (char, u8)) -> bool {
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

    fn straight(board: &Board, from: (char, u8), to: (char, u8)) -> bool {
        from.0 == to.0 && Figure::way_is_clear(board, 0, from, to)
    }

    fn diagonal(board: &Board, from: (char, u8), to: (char, u8)) -> bool {
        (1u8..9).any(|x| {
            // right and up or down
            Figure::way_is_clear(board, 1, from, to) &&
            ((from.0 as u8 + x == to.0 as u8 &&
            (from.1 + x == to.1 || from.1.checked_sub(x).is_some() && from.1 - x == to.1)) ||
            // left and up or down
            ((from.0 as u8).checked_sub(x).is_some() && from.0 as u8 - x == to.0 as u8 &&
            (from.1 + x == to.1 || from.1.checked_sub(x).is_some() && from.1 - x == to.1)))
        })
    }

    fn sideways(board: &Board, from: (char, u8), to: (char, u8)) -> bool {
        from.1 == to.1 && Figure::way_is_clear(board, 2, from, to)
    }

    fn way_is_clear(board: &Board, direction: u8, from: (char, u8), to: (char, u8)) -> bool {
        match direction {
            0 => {
                if to.1 > from.1 {
                    (from.1+1..to.1).all(|x| board.is_empty((from.0, x)))
                } else if to.1 < from.1 {
                    (to.1+1..from.1).all(|x| board.is_empty((from.0, x)))
                } else {
                    false
                }
            },
            1 => {
                if to.0 > from.0 && to.1 > from.1 {
                    let diff = to.0 as u8 - from.0 as u8;
                    (1..diff).all(|x| board.is_empty((char::from(from.0 as u8 + x), from.1 + x)))
                } else if to.0 > from.0 && to.1 < from.1 {
                    let diff = to.0 as u8 - from.0 as u8;
                    (1..diff).all(|x| board.is_empty((char::from(from.0 as u8 + x), from.1 - x)))
                } else if to.0 < from.0 && to.1 > from.1 {
                    let diff = from.0 as u8 - to.0 as u8;
                    (1..diff).all(|x| board.is_empty((char::from(from.0 as u8 - x), from.1 + x)))
                } else if to.0 < from.0 && to.1 < from.1 {
                    let diff = from.0 as u8 - to.0 as u8;
                    (1..diff).all(|x| board.is_empty((char::from(from.0 as u8 - x), from.1 - x)))
                } else {
                    false
                }
            },
            2 => {
                if to.0 as u8 > from.0 as u8 {
                    ((from.0 as u8 + 1)..(to.0 as u8)).all(|x| board.is_empty((char::from(x), from.1)))
                } else {
                    ((to.0 as u8 + 1)..(from.0 as u8)).all(|x| board.is_empty((char::from(x), from.1)))
                }
            },
            _ => unreachable!()
        }
    }

    fn pawn_move(&self, board: &Board, from: (char, u8), to: (char, u8)) -> bool {
        if board.is_empty(to) {
            return match self.color {
                Colour::Black => {
                    match from {
                        (_, 7) => {
                            Figure::straight(board, from, to) &&
                            (5..7).any(|x| x == to.1)
                        },
                        _ => {
                            Figure::straight(board, from, to) &&
                            from.1.checked_sub(1).is_some() &&
                            from.1 - 1 == to.1
                        }
                    }
                },
                _ => {
                    match from {
                        (_, 2) => {
                            Figure::straight(board, from, to) &&
                            (3..5).any(|x| x == to.1)
                        },
                        _ => {
                            Figure::straight(board, from, to) &&
                            from.1 + 1 == to.1
                        }
                    }
                }
            }
        } else {
            board.get_figure_color(to).unwrap() != self.color &&
            (from.0 as u8 - 1 == to.0 as u8 || from.0 as u8 + 1 == to.0 as u8) &&
            match self.color {
                Colour::Black => from.1 - 1 == to.1,
                _ => from.1 + 1 == to.1
            }
        }
    }

    fn bishop_move(&self, board: &Board, from: (char, u8), to: (char, u8)) -> bool {
        let mut clash = true;
        if let Some(f) = board.get_figure_color(to){
            clash = f != self.color;
        }

        clash && Figure::diagonal(board, from, to)
    }

    fn rook_move(&self, board: &Board, from: (char, u8), to: (char, u8)) -> bool {
        let mut clash = true;
        if let Some(f) = board.get_figure_color(to) {
            clash = f != self.color;
        }

        clash && (Figure::straight(board, from, to) || Figure::sideways(board, from, to))
    }

    fn knight_move(&self, board: &Board, from: (char, u8), to: (char, u8)) -> bool {
        let mut clash = true;
        if let Some(f) = board.get_figure_color(to) {
            clash = f != self.color;
        }

        let straight = {
            if from.1 + 2 == to.1 || (from.1.checked_sub(2).is_some() && from.1 - 2 == to.1) {
                from.0 as u8 + 1 == to.0 as u8 || ((from.0 as u8).checked_sub(1).is_some() &&
                                                    from.0 as u8 - 1 == to.0 as u8)
            } else if from.1 + 1 == to.1 || (from.1.checked_sub(1).is_some() &&
                                             from.1 - 1 == to.1)
            {
                from.0 as u8 + 2 == to.0 as u8 || ((from.0 as u8).checked_sub(2).is_some() &&
                                                    from.0 as u8 - 2 == to.0 as u8)
            } else {
                false
            }
        };

        let sideways = {
            if from.0 as u8 + 2 == to.0 as u8 || ((from.0 as u8).checked_sub(2).is_some() &&
                                                   from.0 as u8 - 2 == to.0 as u8)
            {
                from.1 + 1 == to.1 || (from.1.checked_sub(1).is_some() && from.1 - 1 == to.1)
            } else if from.0 as u8 + 1 == to.0 as u8 || ((from.0 as u8).checked_sub(1).is_some() &&
                                                          from.0 as u8 - 1 == to.0 as u8)
            {
                from.1 + 2 == to.1 || (from.1.checked_sub(2).is_some() && from.1 - 2 == to.1)
            } else {
                false
            }
        };

        clash && (straight || sideways)
    }

    fn king_move(&self, board: &Board, from: (char, u8), to: (char, u8)) -> bool {
        let mut clash = true;
        if let Some(f) = board.get_figure_color(to) {
            clash = f != self.color;
        }

        let direction = {
            //  forward or backward
            if from.0 as u8  == to.0 as u8 {
                from.1 + 1 == to.1 || (from.1.checked_sub(1).is_some() && from.1 - 1 == to.1)
            //  right or left
            } else if from.1 == to.1 {
                from.0 as u8 + 1 == to.0 as u8 || ((from.0 as u8).checked_sub(1).is_some() &&
                                                    from.0 as u8 - 1 == to.0 as u8)
            // diagonal right
            } else if from.0 as u8 + 1 == to.0 as u8 {
                from.1 + 1 == to.1 || (from.1.checked_sub(1).is_some() && from.1 - 1 == to.1)
            // diagonal left
            } else if (from.0 as u8).checked_sub(1).is_some() && from.0 as u8 - 1 == to.0 as u8 {
                from.1 + 1 == to.1 || (from.1.checked_sub(1).is_some() && from.1 - 1 == to.1)
            } else {
                false
            }
        };

        clash && direction
    }

    fn queen_move(&self, board: &Board, from: (char, u8), to: (char, u8)) -> bool {
        let mut clash = true;
        if let Some(f) = board.get_figure_color(to) {
            clash = f != self.color;
        }

        clash && (Figure::straight(board, from, to) ||
        Figure::sideways(board, from, to) || Figure::diagonal(board, from, to))
    }
}

impl Display for Figure {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let ftype = match self.name {
            "pawn" => "Pa",
            "bishop" => "Bi",
            "knight" => "Kn",
            "rook" => "Ro",
            "king" => "Ki",
            "queen" => "Qu",
            _ => unreachable!()
        };
        write!(f, "{}{}", self.color.short(), ftype)
    }
}

impl PartialEq for Figure {
    fn eq(&self, other: &Figure) -> bool {
        self.color == other.color && self.name != other.name
    }
}

#[derive(Debug)]
pub struct Board {
    pub fields: HashMap<(char, u8), BoardField>
}

impl Board {
    /// return new board where every field is empty
    pub fn new() -> Self {
        let mut f = HashMap::with_capacity(64);
        for outer in 1..9 {
            for inner in 1..9 {
                let _ = match (char::from(outer + OFFSET), inner) {
                    a @ (_, 2) => f.insert(a, BoardField{ color: White, figure: Some(Pawn)}),
                    a @ ('a', 1) => f.insert(a, BoardField{ color: White, figure: Some(Rook)}),
                    a @ ('h', 1) => f.insert(a, BoardField{ color: White, figure: Some(Rook)}),
                    a @ ('b', 1) => f.insert(a, BoardField{ color: White, figure: Some(Knight)}),
                    a @ ('g', 1) => f.insert(a, BoardField{ color: White, figure: Some(Knight)}),
                    a @ ('c', 1) => f.insert(a, BoardField{ color: White, figure: Some(Bishop)}),
                    a @ ('f', 1) => f.insert(a, BoardField{ color: White, figure: Some(Bishop)}),
                    a @ ('e', 1) => f.insert(a, BoardField{ color: White, figure: Some(King)}),
                    a @ ('d', 1) => f.insert(a, BoardField{ color: White, figure: Some(Queen)}),
                    // Black figures
                    a @ (_, 7) => f.insert(a, BoardField{ color: Black, figure: Some(Pawn)}),
                    a @ ('a', 8) => f.insert(a, BoardField{ color: Black, figure: Some(Rook)}),
                    a @ ('h', 8) => f.insert(a, BoardField{ color: Black, figure: Some(Rook)}),
                    a @ ('b', 8) => f.insert(a, BoardField{ color: Black, figure: Some(Knight)}),
                    a @ ('g', 8) => f.insert(a, BoardField{ color: Black, figure: Some(Knight)}),
                    a @ ('c', 8) => f.insert(a, BoardField{ color: Black, figure: Some(Bishop)}),
                    a @ ('f', 8) => f.insert(a, BoardField{ color: Black, figure: Some(Bishop)}),
                    a @ ('e', 8) => f.insert(a, BoardField{ color: Black, figure: Some(King)}),
                    a @ ('d', 8) => f.insert(a, BoardField{ color: Black, figure: Some(Queen)}),
                    a @ _ => {
                        let c = {
                            if outer % 2 == 1 && inner % 2 == 0 || outer % 2 == 0 && inner % 2 == 1 {
                                White
                            } else {
                                Black
                            }
                        };
                        f.insert(a, BoardField{ color: c, figure: None })
                    }
                };
            }
        }
        Board{ fields: f }
    }

    /// get the symbol of the board at position 'pos'
    pub fn get_figure(&self, pos: (char, u8)) -> Option<Id> {
        if let Some(f) = self.fields.get(&pos) {
            return f.figure
        }
        unreachable!()
    }

    pub fn is_move_valid(&self, from: (char, u8), to: (char, u8)) -> bool {
        if let Some(pos) = self.fields.get(&from) {
            if let Some(fig) = pos.figure() {
                return fig.as_figure(self.get_figure_color(from).unwrap())
                          .valid_move(self, from, to)
            }
        }
        unreachable!()
    }

    pub fn set_figure(&mut self, pos: (char, u8), fig: Figure) {
        self.fields.insert(pos,
            BoardField{ color: fig.color, figure: Some(Id::from_fig(fig.name)) });
    }

    /// get the colour of the figure at position 'pos'
    pub fn get_figure_color(&self, pos: (char, u8)) -> Option<Colour> {
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
    pub fn move_figure(&mut self, before: (char, u8), after: (char, u8)) {
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
    pub fn is_empty(&self, pos: (char, u8)) -> bool {
        self.fields.get(&pos).unwrap().is_empty()
    }

    pub fn in_check(&self, king: (char, u8), opponent: &Player) -> bool {
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
    pub fn checkmate(&mut self, one: &mut Player, two: &mut Player) -> Option<(Colour, i8)> {
        if self.in_check(one.king(), two) {
            if one.can_king_be_saved(self, two) {
                return Some((one.color(), 0))
            } else {
                return Some((one.color(), -1))
            }
        }

        if self.in_check(two.king(), one) {
            if two.can_king_be_saved(self, one) {
                return Some((two.color(), 0))
            } else {
                return Some((two.color(), -2))
            }
        }

        None
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

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        try!(write!(f, "  | a | b | c | d | e | f | g | h |\n"));
        try!(write!(f, "--|---|---|---|---|---|---|---|---|--\n"));
        for outer in (1..9).rev() {
            try!(write!(f, "{} |{}|{}|{}|{}|{}|{}|{}|{}|\n", outer,
                *self.fields.get(&('a', outer)).unwrap(),
                *self.fields.get(&('b', outer)).unwrap(),
                *self.fields.get(&('c', outer)).unwrap(),
                *self.fields.get(&('d', outer)).unwrap(),
                *self.fields.get(&('e', outer)).unwrap(),
                *self.fields.get(&('f', outer)).unwrap(),
                *self.fields.get(&('g', outer)).unwrap(),
                *self.fields.get(&('h', outer)).unwrap()));
        }
        write!(f, "--|---|---|---|---|---|---|---|---|--\n")
    }
}
