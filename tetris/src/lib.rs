mod board;
mod cell;
mod game;
mod modes;
mod piece;

pub use board::*;
pub use cell::*;
pub use game::*;
pub use modes::*;
pub use piece::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TetrisError {
    #[error("Out of range")]
    OutOfRange,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    pub left: bool,
    pub right: bool,
    pub hard_drop: bool,
    pub soft_drop: bool,
    pub cw: bool,
    pub ccw: bool,
    pub hold: bool,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Music {}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Sound {
    Bottom,
    Hold,
    Lock,
    Erase,
    Fall,
    PieceI,
    PieceO,
    PieceT,
    PieceL,
    PieceJ,
    PieceS,
    PieceZ,
    RankUp,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum TetrisEvent {
    LineCleared(usize),
    LineShrinked(Vec<usize>),
    PieceSpawned(Piece),
    PieceLocked(FallingPiece),
}
