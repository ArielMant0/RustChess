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

use chess::logic::{Color, Position};
use chess::player::PlayerType;
use chess::ChessGame;

/// Handles interaction between game logic and visualization
pub struct System {
    // Mouse Coordinates
    mouse_x: i32,
    mouse_y: i32,
    // Selected Pieces/BoardFields
    from: Option<Position>,
    to: Option<Position>,
    // Holds Board and Players
    game: ChessGame,
    ai: bool
}

impl System {
    pub fn new() -> Self {
        System {
            mouse_x: 0,
            mouse_y: 0,
            from: None,
            to: None,
            game: ChessGame::new(),
            ai: false
        }
    }

    /// Returns current mouse coordinates
    pub fn mouse(&self) -> (i32, i32) {
        (self.mouse_x, self.mouse_y)
    }

    /// Sets mouse coordinates
    pub fn set_mouse_coordinates(&mut self, x: i32, y: i32) {
        self.mouse_x = x;
        self.mouse_y = y;
    }

    /// Updates player's figure selection
    pub fn set_selected(&mut self, pos: (u8, u8)) {
        let at = if Position::is_pos(pos.0, pos.1) {Position::new(pos.0, pos.1)} else {return};

        match (self.from, self.to) {
            // First field on which the figure that should be moved is
            (None, None) => {
                if !self.game.board.is_empty(at) && self.game.board.get_figure_color(at).unwrap() == self.game.turn_color() {
                    self.from = Some(at);
                }
            },
            // Second field to which shall be moved
            (Some(_), None) => {
                if self.game.board.is_empty(at) {
                    self.to = Some(at);
                } else if self.game.board.get_figure_color(at).unwrap() != self.game.turn_color() {
                    self.to = Some(at);
                }
            },
            // Reset and set first selection again
            (Some(_), Some(_)) => {
                if !self.game.board.is_empty(at) && self.game.board.get_figure_color(at).unwrap() == self.game.turn_color() {
                    self.from = Some(at);
                }
                self.to = None;
            },
            _ => unreachable!()
        }
    }

    /// If two fields have been selected execute a turn an return the positions which need to be updates visually
    pub fn check_ready_and_play(&mut self) -> Option<((Color, Position, Position), bool)> {
        if self.from.is_some() && self.to.is_some() {
            let result = self.game.do_turn(self.from.unwrap(), self.to.unwrap());
            if result >= 0 {
                // We need to take the opposite color of the one who's turn it is now
                // because our turn has already been made
                let turn_color = !self.game.turn_color();

                let before = self.from.unwrap();
                let after = self.to.unwrap();
                self.reset_selection();

                return Some(((turn_color, before, after), result == 1))
            } else {
                return None
            }
        }
        None
    }

    /// Reset field selections
    pub fn reset_selection(&mut self) {
        self.from = None;
        self.to = None;
    }

    /// Execute a turn for the AI
    pub fn execute_ai_turn(&mut self) -> Option<((Color, Position, Position), bool)> {
        if let Some(((before, after), captured)) = self.game.do_ai_turn() {
            // We need to take the opposite color of the one who's turn it is now
            // because our turn has already been made
            let turn_color = !self.game.turn_color();

            return Some(((turn_color, before, after), captured))
        }
        None
    }

    /// Returns whether an AI is active
    pub fn has_ai(&self) -> bool {
        self.ai
    }

    /// Transforms a board position to a field position in the world
    pub fn from_position(pos: &Position) -> ::cgmath::Point3<f32> {
        ::cgmath::Point3::new(3.5 - pos.x as f32, 0.1, 3.5 - pos.y as f32)
    }

    /// Toggles player type from AI to Human or the other way round and resets the field selections
    pub fn toggle_player_ai(&mut self, which: bool) {
        if which {
            if self.game.white_player.ptype() == PlayerType::Human {
                self.game.white_player.set_ptype(PlayerType::Dumb);
                self.ai = true;
            } else {
                self.game.white_player.set_ptype(PlayerType::Human);
                self.ai = self.game.black_player.ptype() != PlayerType::Human;
            }
        } else {
            if self.game.black_player.ptype() == PlayerType::Human {
                self.game.black_player.set_ptype(PlayerType::Dumb);
                self.ai = true;
            } else {
                self.game.black_player.set_ptype(PlayerType::Human);
                self.ai = self.game.white_player.ptype() != PlayerType::Human;
            }
        }
        self.reset_selection();
    }
}
