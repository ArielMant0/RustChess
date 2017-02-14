extern crate clap;

use game_logic::{Colour, Board, BoardField};
use game_logic::player::{PlayerType, Player};
use self::clap::{App, Arg};

macro_rules! get_player {
    ($matcher:expr, $arg_name:expr) => {{
        match $matcher.value_of($arg_name) {
            Some("human") => PlayerType::Human,
            Some("dumb-ai") => PlayerType::Dumb,
            Some("smart-ai") => PlayerType::Smart,
            _ => unreachable!()
        }
    }};
}

/// Create clap-App and read command line arguments.
/// Returns both players.
pub fn parse_commands() -> (Player, Player) {
    let matches = App::new("chess")
                    .version("1.0.0")
                    .about("Chess Game")
                    .author(concat!("Franziska Becker <buecher.apps@gmail.com>,",
                        " René Wanrking <rwarnking@gmail.com>"))
                    .arg(Arg::with_name("player_one")
                        .help("Selects player one.")
                        .possible_values(&["human", "dumb-ai", "smart-ai"])
                        .required(true))
                    .arg(Arg::with_name("player_two")
                        .possible_values(&["human", "dumb-ai", "smart-ai"])
                        .help("Selects player two.")
                        .required(true))
                    .get_matches();

    (Player::new(get_player!(matches, "player_one"), Colour::White),
        Player::new(get_player!(matches, "player_two"), Colour::Black))
}

/// Reads the input position from the human player.
pub fn read_position_input() -> Option<((char, u8), (char, u8))> {
    let mut buffer = String::new();
    ::std::io::stdin()
        .read_line(&mut buffer)
        .expect("something went horribly wrong...");

    // Shorten string to length of 5 (input is sth like: a2 a3)
    if buffer.len() >= 5 {
        //buffer.truncate(5);
        let positions: Vec<char> = buffer.chars()
                                         .filter(|c| !c.is_whitespace())
                                         .collect();
        if positions.len() == 4 {
            let first;
            if let Some(num_one) = positions[1].to_digit(10) {
                first = (positions[0], num_one as u8);
                if let Some(num_two) = positions[3].to_digit(10) {
                    return Some((first, (positions[2], num_two as u8)))
                }
            }
        }
    }
    None
}

/// Reads and returns the move for the human player
pub fn get_human_move(board: &Board) -> ((char, u8), (char, u8)) {
    loop {
        match read_position_input() {
            Some((one, two)) => {
                if !BoardField::is_pos(one.0, one.1) || !BoardField::is_pos(two.0, two.1) {
                    println!("Please enter a valid position (a-h1-8) on the board!");
                } else if !board.is_empty(two) && board.get_figure_color(one) == board.get_figure_color(two) {
                    println!("You cannot capture your own figure!");
                } else if !board.is_move_valid(one, two) {
                    println!("That was not a valid move!");
                } else {
                    return (one, two)
                }
            },
            None => {
                println!("Please enter a valid position (a-h1-8) on the board!");
            }
        }
    }
}
