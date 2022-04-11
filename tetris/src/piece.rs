use num_enum::IntoPrimitive;
use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::Board;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FallingPiece {
    pub piece_state: PieceState,
    pub piece_position: (usize, usize),
    pub previous_lock_y: usize,
}

impl FallingPiece {
    pub fn from_piece_state(piece_state: PieceState) -> Self {
        let piece_position = piece_state.get_initial_position();
        Self {
            piece_state,
            piece_position,
            previous_lock_y: 0,
        }
    }
    pub fn shift(&mut self, board: &Board, x: i32, y: i32) -> bool {
        if self.check_shift_collision(board, x, y) {
            false
        } else {
            let new_position = (
                (self.piece_position.0 as i32 + x) as usize,
                (self.piece_position.1 as i32 + y) as usize,
            );
            self.piece_position = new_position;

            true
        }
    }
    pub fn check_shift_collision(&self, board: &Board, x: i32, y: i32) -> bool {
        let new_position = (
            (self.piece_position.0 as i32 + x) as usize,
            (self.piece_position.1 as i32 + y) as usize,
        );
        if board.check_collision(self.piece_state, new_position.0, new_position.1) {
            true
        } else {
            false
        }
    }
    pub fn cw(&mut self, board: &Board) -> bool {
        let initial_offsets = self.piece_state.kicks();
        self.piece_state.cw();
        let target_offsets = self.piece_state.kicks();
        let kicks = initial_offsets
            .iter()
            .zip(target_offsets.iter())
            .map(|(&(x1, y1), &(x2, y2))| (x1 - x2, y1 - y2));

        for (dx, dy) in kicks {
            if !self.check_shift_collision(board, dx, dy) {
                self.shift(board, dx, dy);
                return true;
            };
        }
        self.piece_state.ccw();

        false
    }
    pub fn ccw(&mut self, board: &Board) -> bool {
        let initial_offsets = self.piece_state.kicks();
        self.piece_state.ccw();
        let target_offsets = self.piece_state.kicks();
        let kicks = initial_offsets
            .iter()
            .zip(target_offsets.iter())
            .map(|(&(x1, y1), &(x2, y2))| (x1 - x2, y1 - y2));

        for (dx, dy) in kicks {
            if !self.check_shift_collision(board, dx, dy) {
                self.shift(board, dx, dy);
                return true;
            };
        }
        self.piece_state.cw();
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rotation {
    North,
    East,
    South,
    West,
}

impl Rotation {
    pub fn cw(&mut self) {
        use Rotation::*;
        match self {
            North => *self = East,
            East => *self = South,
            South => *self = West,
            West => *self = North,
        }
    }

    pub fn ccw(&mut self) {
        use Rotation::*;
        match self {
            North => *self = West,
            East => *self = North,
            South => *self = East,
            West => *self = South,
        }
    }
}

const STANDS: [[(i16, i16); 4]; 7] = [
    [(-1, 0), (0, 0), (1, 0), (2, 0)],  // I
    [(0, 0), (1, 0), (0, 1), (1, 1)],   // O
    [(-1, 0), (0, 0), (1, 0), (0, 1)],  // T
    [(-1, 0), (0, 0), (1, 0), (1, 1)],  // L
    [(-1, 0), (0, 0), (1, 0), (-1, 1)], // J
    [(-1, 0), (0, 0), (0, 1), (1, 1)],  // S
    [(-1, 1), (0, 1), (0, 0), (1, 0)],  // Z
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, IntoPrimitive)]
#[repr(u8)]
pub enum Piece {
    I,
    O,
    T,
    L,
    J,
    S,
    Z,
}

impl Piece {
    pub fn get_cells(&self) -> Vec<(i16, i16)> {
        STANDS[*self as usize].to_vec()
    }
}

impl Distribution<Piece> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Piece
    where
        R: Rng,
    {
        use Piece::*;
        match rng.gen_range(0..=6) {
            0 => I,
            1 => O,
            2 => T,
            3 => L,
            4 => J,
            5 => S,
            6 => Z,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PieceState {
    kind: Piece,
    rotation: Rotation,
}

impl Distribution<PieceState> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceState
    where
        R: Rng,
    {
        PieceState {
            kind: rng.gen(),
            rotation: Rotation::North,
        }
    }
}

impl PieceState {
    pub fn from_piece(piece: Piece) -> Self {
        Self {
            kind: piece,
            rotation: Rotation::North,
        }
    }
    pub fn get_cells(&self) -> Vec<(i16, i16)> {
        let stand = self.kind.get_cells();

        let rot_matrix = match self.rotation {
            Rotation::North => vec![(1, 0), (0, 1)],
            Rotation::East => vec![(0, 1), (-1, 0)],
            Rotation::South => vec![(-1, 0), (0, -1)],
            Rotation::West => vec![(0, -1), (1, 0)],
        };
        let rotated = stand
            .iter()
            .map(|c| {
                (
                    rot_matrix[0].0 * c.0 + rot_matrix[0].1 * c.1,
                    rot_matrix[1].0 * c.0 + rot_matrix[1].1 * c.1,
                )
            })
            .collect::<Vec<_>>();
        rotated
    }

    pub fn get_kind(&self) -> Piece {
        self.kind.clone()
    }

    pub fn get_initial_position(&self) -> (usize, usize) {
        match self.kind {
            Piece::I => (4, 20),
            _ => (4, 21),
        }
    }

    pub fn cw(&mut self) {
        self.rotation.cw()
    }

    pub fn ccw(&mut self) {
        self.rotation.ccw()
    }

    pub fn kicks(&self) -> [(i32, i32); 5] {
        use Piece::*;
        use Rotation::*;

        match (self.kind, self.rotation) {
            // (O, North) => [(0, 0); 5],
            // (O, East) => [(0, -1); 5],
            // (O, South) => [(-1, -1); 5],
            // (O, West) => [(-1, 0); 5],

            // (I, North) => [(0, 0), (-1, 0), (2, 0), (-1, 0), (2, 0)],
            // (I, East) => [(-1, 0), (0, 0), (0, 0), (0, 1), (0, -2)],
            // (I, South) => [(-1, 1), (1, 1), (-2, 1), (1, 0), (-2, 0)],
            // (I, West) => [(0, 1), (0, 1), (0, 1), (0, -1), (0, 2)],

            // (_, North) => [(0, 0); 5],
            // (_, East) => [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
            // (_, South) => [(0, 0); 5],
            // (_, West) => [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
            (O, North) => [(0, 0); 5],
            (O, East) => [(0, 1); 5],
            (O, South) => [(-1, 1); 5],
            (O, West) => [(-1, 0); 5],

            (I, North) => [(0, 0), (-1, 0), (2, 0), (-1, 0), (2, 0)],
            (I, East) => [(-1, 0), (0, 0), (0, 0), (0, 1), (0, 2)],
            (I, South) => [(-1, -1), (1, -1), (-2, -1), (1, 0), (-2, 0)],
            (I, West) => [(0, -1), (0, -1), (0, -1), (0, 1), (0, -2)],

            (_, North) => [(0, 0); 5],
            (_, East) => [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
            (_, South) => [(0, 0); 5],
            (_, West) => [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
        }
    }
}
