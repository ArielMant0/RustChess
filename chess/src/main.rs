mod game_logic;
mod interface;

use interface::parse_commands;
use game_logic::gameplay::play;

fn main() {
    // get players
    let (player_one, player_two) = parse_commands();
    // start the game
    play(player_one, player_two);
}
