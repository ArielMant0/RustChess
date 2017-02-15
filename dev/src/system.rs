use chess::logic::{Position, BoardField};
use chess::ChessGame;

pub struct System {
    // Mouse Coordinates
    mouse_x: i32,
    mouse_y: i32,
    // Selected Pieces/BoardFields
    from: Option<Position>,
    to: Option<Position>,
    // Holds Board and Players
    game: ChessGame
}

impl System {
    pub fn new() -> Self {
        System {
            mouse_x: 0,
            mouse_y: 0,
            from: None,
            to: None,
            game: ChessGame::new()
        }
    }

    pub fn mouse(&self) -> (i32, i32) {
        (self.mouse_x, self.mouse_y)
    }

    pub fn set_mouse_coordinates(&mut self, x: i32, y: i32) {
        self.mouse_x = x;
        self.mouse_y = y;
    }

    pub fn set_selected(&mut self, pos: (u8, u8)) {
        println!("What system gets: {}, {}", pos.0, pos.1);

        if self.from.is_none() {
            self.from = Some(Position::new(pos.0, pos.1));
        } else if self.to.is_none() {
            self.to = Some(Position::new(pos.0, pos.1));
        } else {
            self.from = Some(Position::new(pos.0, pos.1));
            self.to = None;
        }
    }

    pub fn check_ready_and_play(&mut self) {
        if self.from.is_some() && self.to.is_some() {
            self.execute_turn();
        }
    }

    pub fn execute_turn(&mut self) {
        if self.game.board.is_move_valid(self.from.unwrap(), self.to.unwrap()) {
            self.game.do_turn(self.from.unwrap(), self.to.unwrap());
        }
    }
}

