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

    pub fn set_selected(&mut self) {
        /*let field = match (self.mouse_x, self.mouse_y) {
            // TODO select a field an set as selected
            _ => ()
        }

        if from.is_none() {
            from = Some(field);
        } else if to.is_none() {
            to = Some(field)
        } else {
            from = Some(field);
            to = None;
        }*/
    }

    pub fn execute_turn(&self) {
        /*if self.game.board.is_move_valid(from, to) {
            self.game.do_turn(from, to);
        }*/
    }
}
