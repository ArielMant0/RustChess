extern crate rand;

use chess::logic::{Id, Board, Position};
use chess::player::{Player, PlayerType};

use self::rand::{thread_rng, Rng};

/// Returns a move for the AI, depending on which one it is.
pub fn get_move(board: &Board, me: &Player, other: &Player) -> (Position, Position) {
    // If AI is stupid
    if me.ptype() == PlayerType::Dumb {
        get_dumb_move(board, me, other)
    // If AI is smart
    } else {
        get_smart_move(board.clone(), me.clone(), other.clone())
    }
}

fn figure_value(fig: Id) -> u8 {
    match fig {
        Id::King => 100,
        Id::Queen => 80,
        Id::Bishop | Id::Knight => 50,
        Id::Rook => 25,
        Id::Pawn => 10
    }
}

fn random_move(moves: &Vec<(Position, Position)>) -> (Position, Position) {
    let mut rng = thread_rng();
    let index = rng.gen_range(0, moves.len());
    moves[index]
}

/// Returns a dumb move.
fn get_dumb_move(board: &Board, me: &Player, other: &Player) -> (Position, Position) {
    let my_moves = me.get_possible_moves(board);
    let at = my_moves.iter()
                     .max_by_key(|x| {
                        if !board.is_empty(x.1) &&
                            board.get_figure_color(x.1).unwrap() == other.color()
                        {
                            (figure_value(board.get_figure(x.1).unwrap()))
                        } else {
                            0
                        }})
                     .unwrap();

    if board.is_empty(at.1) {
        random_move(&my_moves)
    } else {
        *at
    }
}

fn get_smart_move(board: Board, me: Player, other: Player) -> (Position, Position) {
    //let my_moves = me.get_possible_moves();
    //let opp_moves = other.get_possible_moves();

    (Position::new(1, 1), Position::new(1, 2))
}
