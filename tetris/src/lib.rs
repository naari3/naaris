mod board;
mod cell;
mod game;
mod piece;

pub use board::*;
pub use cell::*;
pub use game::*;
pub use piece::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TetrisError {
    #[error("Out of range")]
    OutOfRange,
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
