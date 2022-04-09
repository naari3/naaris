use std::fmt::Display;

mod board;
mod cell;
mod piece;

pub use board::*;
pub use cell::*;
pub use piece::*;
use rand::random;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TetrisError {
    #[error("Out of range")]
    OutOfRange,
}

pub struct Game {
    board: Board,
    current_piece: Option<PieceState>,
    piece_position: (usize, usize),
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    {:?}", self.board.next_pieces)?;
        writeln!(f, "    0 1 2 3 4 5 6 7 8 9")?;
        writeln!(f, "    -------------------")?;
        let mut render_board = self.board.clone();
        let (x, y) = self.piece_position;
        if let Some(current_piece) = self.current_piece {
            render_board.set_piece(&current_piece, x, y).unwrap();
        }

        let offset_y = 20;
        for (y, cells_x) in render_board.cells.iter().enumerate() {
            if y < offset_y {
                continue;
            };
            write!(f, "{y:02}| ")?;
            for cell in cells_x {
                match cell {
                    Some(cell) => write!(f, "{cell} ")?,
                    None => write!(f, "  ")?,
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Game {
    pub fn new() -> Self {
        let mut board = Board::default();
        let next = board.pop_next();
        let current_piece = PieceState::from_piece(next);
        let piece_position = current_piece.get_initial_position();
        Self {
            board,
            current_piece: Some(current_piece),
            piece_position,
        }
    }

    pub fn update(&mut self) {
        if let Some(piece_state) = self.current_piece {
            let (x, y) = self.piece_position;
            let new_piece_position = (x, y + 1);
            if self
                .board
                .check_collision(piece_state, new_piece_position.0, new_piece_position.1)
            {
                self.board.set_piece(&piece_state, x, y).unwrap();
                self.current_piece = None;
            } else {
                self.piece_position = new_piece_position;
            }
        } else {
            let next_piece = PieceState::from_piece(self.board.pop_next());
            self.piece_position = next_piece.get_initial_position();
            self.current_piece = next_piece.into();
        }
    }
}

pub struct Input {
    left: bool,
    right: bool,
    hard_drop: bool,
    soft_drop: bool,
    cw: bool,
    ccw: bool,
    hold: bool,
}
