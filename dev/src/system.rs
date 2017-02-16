// MIT License
//
// Copyright (c) 2017 Franziska Becker, René Warking
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use chess::logic::{Color, Position, BoardField};
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
        let at = Position::new(pos.0, pos.1);

        if !self.game.board.is_empty(at) && self.game.board.get_figure_color(at).unwrap() == self.game.turn_color() {
            match (self.from, self.to) {
                (None, None) => self.from = Some(at),
                (Some(_), None) => self.to = Some(at),
                (Some(_), Some(_)) => {
                    self.from = Some(at);
                    self.to = None;
                },
                _ => unreachable!()
            }
        }
    }

    pub fn check_ready_and_play(&mut self)
        -> Option<((Color, Position, Position), Option<(Color, Position)>)>
    {
        if self.from.is_some() && self.to.is_some() {
            self.execute_turn();
            if self.game.was_captured() {
                let one = self.game.turn_color();
                let two = if one == Color::Black {Color::White} else {Color::Black};

                return Some(((one, self.from.unwrap(), self.to.unwrap()), Some((two, self.to.unwrap()))))
            } else {
                let one = self.game.turn_color();
                return Some(((one, self.from.unwrap(), self.to.unwrap()), None))
            }
        }
        None
    }

    pub fn execute_turn(&mut self) {
        if self.game.board.is_move_valid(self.from.unwrap(), self.to.unwrap()) {
            self.game.do_turn(self.from.unwrap(), self.to.unwrap());
        }
    }

    pub fn from_position(pos: &Position) -> ::cgmath::Point3<f32> {
        ::cgmath::Point3::new(pos.x as f32 - 5.0, 0.0, pos.y as f32 + 5.0)
    }
}
