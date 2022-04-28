use std::fmt::Display;

use arrayvec::ArrayVec;
use rand::{prelude::SliceRandom, thread_rng};

use crate::{Cell, FallingPiece, Piece, PieceState, TetrisError};

#[derive(Debug, Clone)]
pub struct Board {
    pub cells: ArrayVec<ArrayVec<Option<Cell>, 10>, 40>,
    pub next_pieces: Vec<Piece>,
    bag: Vec<Piece>,
    pub hold_piece: Option<Piece>,
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
            hold_piece: None,
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
    pub fn set_piece(&mut self, piece: &FallingPiece) -> Result<(), TetrisError> {
        let (x, y) = piece.piece_position;
        for (rel_x, rel_y) in piece.piece_state.get_cells().into_iter() {
            if let Some(cells_x) = self.cells.get_mut((-rel_y + y as i16) as usize) {
                if let Some(cell) = cells_x.get_mut((rel_x + x as i16) as usize) {
                    *cell = Some(piece.piece_state.get_kind().into());
                } else {
                    return Err(TetrisError::OutOfRange);
                }
            } else {
                return Err(TetrisError::OutOfRange);
            }
        }
        Ok(())
    }

    pub fn check_collision(&self, piece: PieceState, x: usize, y: usize) -> bool {
        for (rel_x, rel_y) in piece.get_cells().into_iter() {
            if let Some(cells_x) = self.cells.clone().get_mut((-rel_y + y as i16) as usize) {
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

    pub fn line_clear(&mut self) -> Option<usize> {
        let mut cleared_lines = 0;
        for cells_x in self.cells.iter_mut() {
            if cells_x.iter().all(|d| d.is_some()) {
                *cells_x = ArrayVec::from([None; 10]);
                cleared_lines += 1;
            }
        }
        if cleared_lines != 0 {
            Some(cleared_lines)
        } else {
            None
        }
    }

    pub fn line_shrink(&mut self) -> Vec<usize> {
        let mut lines = vec![];
        for (y, cells_x) in self.cells.iter_mut().enumerate() {
            if cells_x.iter().all(|d| d.is_none()) {
                lines.push(y);
            }
        }
        for y in lines.iter().rev() {
            self.cells.remove(*y);
        }
        for _ in 0..lines.len() {
            self.cells.insert(0, ArrayVec::from([None; 10]));
        }
        lines
    }

    pub fn swap_hold_piece(&mut self, piece: Piece) -> Option<Piece> {
        let original = self.hold_piece;
        self.hold_piece = Some(piece);
        original
    }
}
