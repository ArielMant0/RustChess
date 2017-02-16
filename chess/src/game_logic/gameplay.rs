use game_logic::{Board, Colour};
use game_logic::player::Player;

/// Gameplay loop.
pub fn play(mut one: Player, mut two: Player) {
    let mut turn: i8 = 1;
    let mut board = Board::new();

    loop {

        match board.checkmate(&mut one, &mut two) {
            Some((king, over)) =>
                if over == 0 {
                    println!("{} King is in check", king);
                } else {
                    turn = over;
                },
            None => {}
        }

        println!("{}", board);

        turn = match turn {
            1 => if execute_round(&mut board, &mut one, &mut two) {2} else {-1},
            2 => if execute_round(&mut board, &mut two, &mut one) {1} else {-2},
            -1 | -2 => { turn },
            _ => unreachable!()
        };

        if turn == -1 {
            println!("Checkmate, {} Player lost.", one);
            break;
        } else if turn == -2 {
            println!("Checkmate, {} Player lost.", two);
            break;
        }
    }
}

/// Executes a round by getting the move of the current player
fn execute_round(board: &mut Board, active: &mut Player, inactive: &mut Player) -> bool {
    println!("{} Player\'s turn:", active);

    loop {
        let mut name = String::new();
        let mut reverse = false;
        let (before, after) = active.get_move(board, inactive);

        if !board.is_empty(after) {
            name = board.get_figure(after).unwrap().name();
            inactive.capture(after, name.clone());
            reverse = true;
        }
        board.move_figure(before, after);
        active.move_figure(before, after);

        if !board.in_check(active.king(), inactive) {
            return true
        } else {
            println!("You must save your King!");
            if reverse {
                inactive.reverse_capture(&name, after);
            }
            board.move_figure(after, before);
            active.move_figure(after, before);
        }
    }
}
