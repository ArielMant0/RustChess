use game_logic::{Colour, Board, OFFSET};
use std::fmt::{Formatter, Display, Error};
use game_logic::ai::get_ai_move;
use interface::get_human_move;
use std::collections::HashMap;
use game_logic::Figure;

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
    color: Colour,
    figures: HashMap<String, Vec<(char, u8)>>
}

impl Player {
    // Returns an instance of a Player with the given PlayerType
    pub fn new(p: PlayerType, c: Colour) -> Self {
        match c {
            Colour::Black => Player::create_black_player(p, c),
            _ => Player::create_white_player(p, c),
        }
    }

    fn create_black_player(p: PlayerType, c: Colour) -> Self {
        let mut f = HashMap::with_capacity(16);
        let mut pos = Vec::new();
        // Pawns
        for bla in 1..9 {
            pos.push((char::from(bla + OFFSET), 7))
        }
        f.insert("pawn".to_string(), pos);
        // King
        f.insert("king".to_string(), vec![('e', 8)]);
        // Queen
        f.insert("queen".to_string(), vec![('d', 8)]);
        // Bishops
        f.insert("bishop".to_string(), vec![('c', 8),
                                            ('f', 8)]);
        // Knights
        f.insert("knight".to_string(), vec![('b', 8),
                                            ('g', 8)]);
        // Rooks
        f.insert("rook".to_string(), vec![('a', 8),
                                          ('h', 8)]);
        Player { ptype: p, color: c, figures: f }
    }

    fn create_white_player(p: PlayerType, c: Colour) -> Self {
        let mut f = HashMap::with_capacity(16);
        let mut pos = Vec::new();
        // Pawns
        for bla in 1..9 {
            pos.push((char::from(bla + OFFSET), 2))
        }
        f.insert("pawn".to_string(), pos);
        // King
        f.insert("king".to_string(), vec![('e', 1)]);
        // Queen
        f.insert("queen".to_string(), vec![('d', 1)]);
        // Bishops
        f.insert("bishop".to_string(), vec![('c', 1),
                                            ('f', 1)]);
        // Knights
        f.insert("knight".to_string(), vec![('b', 1),
                                            ('g', 1)]);
        // Rooks
        f.insert("rook".to_string(), vec![('a', 1),
                                          ('h', 1)]);
        Player { ptype: p, color: c, figures: f }
    }

    pub fn color(&self) -> Colour {
        self.color
    }

    pub fn figures(&self) -> HashMap<String, Vec<(char, u8)>> {
        self.figures.clone()
    }

    pub fn get_move(&self, board: &Board) -> ((char, u8), (char, u8)) {
        match self.ptype {
            PlayerType::Human => get_human_move(board),
            _ => get_ai_move(board, self.ptype)
        }
    }

    pub fn king(&self) -> (char, u8) {
        self.figures.get("king").unwrap()[0]
    }

    pub fn move_figure(&mut self, before: (char, u8), after: (char, u8)) {
        for mut v in self.figures.values_mut() {
            for i in 0..v.len() {
                if v[i] == before {
                    v[i] = after;
                    return
                }
            }
        }
    }

    pub fn capture(&mut self, pos: (char, u8), name: String) {
        if let Some(mut positions) = self.figures.get_mut(&name) {
            for i in 0..positions.len() {
                if positions[i] == pos {
                    positions.remove(i);
                    break;
                }
            }
        }
    }

    pub fn reverse_capture(&mut self, name: &str, pos: (char, u8)) {
        if let Some(mut v) = self.figures.get_mut(name) {
                v.push(pos);
        }
    }

    pub fn can_king_be_saved(&mut self, board: &mut Board, two: &Player) -> bool {
        for &elem in FIGURE_NAMES {
            let v = self.figures.get_mut(elem).unwrap();
            for pos in 0..v.len() {
                let before: (char, u8) = v[pos];
                for i in 1..9 {
                    for j in 1..9 {
                        if board.is_move_valid(before, (char::from(i as u8 + OFFSET), j)) {
                            let mut tmp = Figure::new();
                            let mut reset = false;
                            if !board.is_empty((char::from(i as u8 + OFFSET), j)) {
                                tmp = board.get_figure((char::from(i as u8 + OFFSET), j))
                                           .unwrap()
                                           .as_figure(two.color());
                                reset = true;
                            }
                            board.move_figure(before, (char::from(i as u8 + OFFSET), j));
                            v[pos] = (char::from(i as u8 + OFFSET), j);

                            if !board.in_check((char::from(i as u8 + OFFSET), j), two) {
                                board.move_figure((char::from(i as u8 + OFFSET), j), before);
                                if reset {
                                    board.set_figure((char::from(i as u8 + OFFSET), j), tmp);
                                }
                                v[pos] = before;
                                return true
                            } else {
                                board.move_figure((char::from(i as u8 + OFFSET), j), before);
                                if reset {
                                    board.set_figure((char::from(i as u8 + OFFSET), j), tmp);
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

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.color)
    }
}
