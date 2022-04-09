use std::fmt::Display;

use crate::{Board, FallingPiece, PieceState};

pub struct Game {
    board: Board,
    pub current_piece: Option<FallingPiece>,
    gravity: f64,
    shift_down_counter: f64,
    lock_delay: usize,
    lock_counter: usize,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    {:?}", self.board.next_pieces)?;
        writeln!(f, "    0 1 2 3 4 5 6 7 8 9")?;
        writeln!(f, "    -------------------")?;
        let mut render_board = self.board.clone();
        if let Some(current_piece) = self.current_piece {
            render_board.set_piece(&current_piece).unwrap();
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
        let current_piece = FallingPiece::from_piece_state(PieceState::from_piece(next));
        Self {
            board,
            current_piece: Some(current_piece),
            gravity: 1024.0 / 65536.0,
            shift_down_counter: 0.0,
            lock_delay: 30,
            lock_counter: 0,
        }
    }

    pub fn update(&mut self) {
        if let None = self.current_piece {
            let next_piece = PieceState::from_piece(self.board.pop_next());
            self.current_piece = FallingPiece::from_piece_state(next_piece).into();
        }
        self.apply_gravity();
    }

    fn apply_gravity(&mut self) {
        if let Some(mut current_piece) = self.current_piece.clone() {
            if current_piece.check_shift_collision(&self.board, 0, 1) {
                self.lock_counter += 1;
                println!("{}", self.lock_counter);
                if self.lock_counter >= self.lock_delay {
                    self.board.set_piece(&current_piece).unwrap();
                    self.current_piece = None;
                    self.lock_counter = 0;
                    return;
                }
            }

            self.shift_down_counter += self.gravity;
            if self.shift_down_counter >= 1.0 {
                let fall_size = self.shift_down_counter as i32;
                self.shift_down_counter = 0.0;

                if current_piece.shift(&self.board, 0, fall_size) {
                    // falling down successful
                }
            }
            self.current_piece = Some(current_piece);
        }
    }

    fn apply_lock_delay(&mut self) {}

    pub fn get_board(&self) -> Board {
        self.board.clone()
    }
}
