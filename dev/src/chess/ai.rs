extern crate rand;

use chess::logic::{Figure, Board, Position};
use chess::player::{Player, PlayerType};

use self::rand::{thread_rng, Rng};

/// Returns a move for the AI, depending on which one it is
pub fn get_move(board: &Board, me: &Player, other: &Player) -> (Position, Position) {

    // If AI is stupid
    if me.ptype() != PlayerType::Smart {
        get_dumb_move(&mut board.clone(), &mut me.clone(), &mut other.clone())
    // If AI is smart
    } else {
        get_smart_move(board.clone(), me.clone(), other.clone())
    }
}

/// Returns the measure of a figure's value
fn figure_value(fig: &Figure) -> u8 {
    match *fig {
        Figure::King => 100,
        Figure::Queen => 80,
        Figure::Bishop | Figure::Knight => 50,
        Figure::Rook => 25,
        Figure::Pawn => 10
    }
}

/// Returns a random move in 'moves'
fn random_move(moves: &Vec<(Position, Position)>) -> (Position, Position) {
    let mut rng = thread_rng();
    let index = rng.gen_range(0, moves.len());

    moves[index]
}

/// Returns a dumb move
fn get_dumb_move(board: &mut Board, me: &mut Player, other: &mut Player) -> (Position, Position) {
    let my_moves = me.get_possible_moves(board, other);

    if let Some(at) = my_moves.iter().max_by_key(|x| capture_and_evade(board, x)) {
        if board.is_empty(at.1) {
            return random_move(&my_moves)
        } else {
            return *at
        }
    }

    // If we got here than there is no valid move to make, which should not happen
    // because then this function should not be called in the first place
    unreachable!()
}


/// Return a measure that tries to capture opponent figures and evade being captured
fn capture_and_evade(board: &Board, pos: &(Position, Position)) -> u8 {
    if board.is_capture_move(pos.0, pos.1) {
        figure_value(&board.get_figure(pos.1).unwrap())
    } else {
        0
    }

    // TODO: implement evasion
    /*let ev = {
        let b = board.clone();

    };*/
}

/// Chooses a smart AI move
#[allow(dead_code, unused_variables)]
fn get_smart_move(board: Board, me: Player, other: Player) -> (Position, Position) {
    // TODO: implement this (minimax?)

    (Position::new(1, 1), Position::new(1, 2))
}

