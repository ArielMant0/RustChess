extern crate rand;

use chess::logic::{Id, Board, Position};
use chess::player::{Player, PlayerType};

use self::rand::{thread_rng, Rng};

/// Returns a move for the AI, depending on which one it is.
pub fn get_move(board: &mut Board, me: &mut Player, other: &mut Player) -> (Position, Position) {
    // If AI is stupid
    if me.ptype() != PlayerType::Smart {
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

// freesounds.org

fn random_move(moves: &Vec<(Position, Position)>) -> (Position, Position) {
    let mut rng = thread_rng();
    let index = rng.gen_range(0, moves.len());
    moves[index]
}

/// Returns a dumb move.
fn get_dumb_move(board: &mut Board, me: &mut Player, other: &mut Player) -> (Position, Position) {
    let my_moves = me.get_possible_moves(board);
    let mut tmp_board = board.clone();
    let mut tmp_other = other.clone();

    if let Some(at) = my_moves.iter()
                     .filter(|&x| move_is_good(x.clone(), &mut tmp_board, me, &mut tmp_other))
                     .max_by_key(|x| {
                        if !board.is_empty(x.1) &&
                            board.get_figure_color(x.1).unwrap() == other.color()
                        {
                            (figure_value(board.get_figure(x.1).unwrap()))
                        } else {
                            0
                        }})
    {

        if board.is_empty(at.1) {
            return random_move(&my_moves)
        } else {
            return *at
        }
    } else {
        unreachable!()
    }
}

fn move_is_good(pos: (Position, Position), board: &mut Board, me: &mut Player, other: &mut Player) -> bool {
    let from = pos.0;
    let to = pos.1;

    let mut name = String::new();
    let mut reverse = false;

    if !board.is_empty(to) {
        name = board.get_figure(to).unwrap().name();
        other.capture(to, name.clone());
        reverse = true;
    }
    board.move_figure(from, to);
    me.move_figure(from, to);

    if !board.in_check(me.king(), other) {
        if reverse {
            other.reverse_capture(&name, to);
        }
        board.move_figure(to, from);
        me.move_figure(to, from);
        return true
    } else {
        if reverse {
            other.reverse_capture(&name, to);
        }
        board.move_figure(to, from);
        me.move_figure(to, from);
        return false
    }
}

#[allow(dead_code, unused_variables)]
fn get_smart_move(board: Board, me: Player, other: Player) -> (Position, Position) {
    //let my_moves = me.get_possible_moves();
    //let opp_moves = other.get_possible_moves();

    (Position::new(1, 1), Position::new(1, 2))
}
