use std::fmt::Display;

use arrayvec::ArrayVec;

mod cell;
mod piece;

pub use cell::*;
pub use piece::*;
use rand::{prelude::SliceRandom, random, thread_rng};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TetrisError {
    #[error("Out of range")]
    OutOfRange,
}

#[derive(Debug, Clone)]
pub struct Board {
    cells: ArrayVec<ArrayVec<Option<Cell>, 10>, 40>,
    next_pieces: Vec<Piece>,
    bag: Vec<Piece>,
}

impl Default for Board {
    fn default() -> Self {
        let mut cells = ArrayVec::new();
        for _ in 0..40 {
            cells.push(ArrayVec::from([None; 10]));
        }
        let mut rng = thread_rng();
        let mut bag = vec![
            Piece::I,
            Piece::O,
            Piece::T,
            Piece::L,
            Piece::J,
            Piece::S,
            Piece::Z,
        ];
        bag.shuffle(&mut rng);
        let mut next_pieces = bag.clone();
        next_pieces.shuffle(&mut rng);
        Self {
            cells,
            next_pieces,
            bag,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    0 1 2 3 4 5 6 7 8 9")?;
        writeln!(f, "    -------------------")?;

        let offset_y = 20;
        for (y, cells_x) in self.cells.iter().enumerate() {
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

impl Board {
    pub fn set_piece(&mut self, piece: &PieceState, x: usize, y: usize) -> Result<(), TetrisError> {
        for (rel_x, rel_y) in piece.get_cells().into_iter() {
            if let Some(cells_x) = self.cells.get_mut((-rel_y + y as i16) as usize) {
                if let Some(cell) = cells_x.get_mut((rel_x + x as i16) as usize) {
                    *cell = Some(match piece.get_kind() {
                        Piece::I => Cell::Cyan,
                        Piece::O => Cell::Yellow,
                        Piece::T => Cell::Purple,
                        Piece::L => Cell::Blue,
                        Piece::J => Cell::Orange,
                        Piece::S => Cell::Green,
                        Piece::Z => Cell::Red,
                    });
                } else {
                    return Err(TetrisError::OutOfRange);
                }
            } else {
                return Err(TetrisError::OutOfRange);
            }
        }
        Ok(())
    }

    pub fn check_collision(&mut self, piece: PieceState, x: usize, y: usize) -> bool {
        for (rel_x, rel_y) in piece.get_cells().into_iter() {
            if let Some(cells_x) = self.cells.get_mut((-rel_y + y as i16) as usize) {
                if let Some(cell) = cells_x.get_mut((rel_x + x as i16) as usize) {
                    if let Some(_) = *cell {
                        return true;
                    }
                } else {
                    return true;
                }
            } else {
                return true;
            }
        }
        return false;
    }

    pub fn pop_next(&mut self) -> Piece {
        let next = self.next_pieces.remove(0);
        self.next_pieces.push(self.bag.remove(0));
        if self.bag.len() == 0 {
            let mut rng = thread_rng();
            let mut bag = vec![
                Piece::I,
                Piece::O,
                Piece::T,
                Piece::L,
                Piece::J,
                Piece::S,
                Piece::Z,
            ];
            bag.shuffle(&mut rng);
            self.bag = bag;
        }

        next
    }
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
        render_board
            .set_piece(&self.current_piece.unwrap(), x, y)
            .unwrap();

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
                let new_piece_state: PieceState = random();
                self.piece_position = new_piece_state.get_initial_position();
                self.current_piece = Some(new_piece_state);
            } else {
                self.piece_position = new_piece_position;
            }
        } else {
            self.current_piece = random();
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
