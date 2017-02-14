use game_logic::Board;
use game_logic::player::PlayerType;


/// Returns a dumb move that just looks for an empty field
/// in a line that no other marks.
fn get_dumb_move(board: &Board) -> ((char, u8), (char, u8)) {
    if board.is_empty(('a', 2)) {
        println!("empty");
    }
    (('a', 2), ('a', 3))
}

/// Returns a move for the AI, depending on which one it is.
pub fn get_ai_move(board: &Board, p: PlayerType) -> ((char, u8), (char, u8)) {
    // if AI is stupid
    if p == PlayerType::Dumb {
        get_dumb_move(board)
    // if AI is smart
    } else {
        get_dumb_move(board)
    }
}
