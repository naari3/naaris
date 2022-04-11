use std::fmt::Display;

use crate::{Board, FallingPiece, Input, Piece, PieceState, Sound};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DasState {
    Left,
    Right,
    None,
}

impl Default for DasState {
    fn default() -> Self {
        DasState::Left
    }
}

pub struct Game {
    board: Board,
    pub current_piece: Option<FallingPiece>,
    gravity: f64,
    shift_down_counter: f64,
    lock_delay: usize,
    lock_counter: usize,
    input: Input,
    previous_input: Input,
    sound_queue: Vec<Sound>,
    das: usize,
    das_counter: usize,
    das_state: DasState,
    are: usize,
    line_are: usize,
    are_counter: Option<usize>,
    line_clear_lock: usize,
    line_clear_lock_timer: Option<usize>,
    hold_used: bool,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    {:?}", self.board.hold_piece)?;
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
        // Self {
        //     board,
        //     current_piece: Some(current_piece),
        //     gravity: 1024.0 / 65536.0,
        //     // gravity: 1.0 / 2.0,
        //     // gravity: 20.0,
        //     shift_down_counter: 0.0,
        //     lock_delay: 30,
        //     lock_counter: 0,
        //     input: Default::default(),
        //     previous_input: Default::default(),
        //     sound_queue: vec![],
        //     das: 14,
        //     das_counter: 0,
        //     das_state: Default::default(),
        //     are: 25,
        //     line_are: 25,
        //     are_counter: None,
        //     line_clear_lock: 40,
        //     line_clear_lock_timer: None,
        //     hold_used: false,
        // }
        Self {
            board,
            current_piece: Some(current_piece),
            gravity: 20.0,
            shift_down_counter: 0.0,
            lock_delay: 18,
            lock_counter: 0,
            input: Default::default(),
            previous_input: Default::default(),
            sound_queue: vec![],
            das: 8,
            das_counter: 0,
            das_state: Default::default(),
            are: 10,
            line_are: 6,
            are_counter: None,
            line_clear_lock: 6,
            line_clear_lock_timer: None,
            hold_used: false,
        }
    }

    pub fn update(&mut self) {
        if let None = self.current_piece {
            if let Some(line_clear_lock_timer) = self.line_clear_lock_timer.as_mut() {
                *line_clear_lock_timer -= 1;
                if *line_clear_lock_timer <= 0 {
                    self.sound_queue.push(Sound::Fall);
                    self.line_clear_lock_timer = None;
                    self.board.line_shrink();
                }
            } else {
                if let Some(are_counter) = self.are_counter.as_mut() {
                    *are_counter -= 1;
                    if *are_counter <= 0 {
                        let next_piece = PieceState::from_piece(self.board.pop_next());
                        self.current_piece = FallingPiece::from_piece_state(next_piece).into();

                        let sound = self.get_next_sound();
                        self.sound_queue.push(sound);

                        self.hold_used = false;
                        self.are_counter = None;
                    }
                }
            }
        }
        self.handle_hold();
        self.handle_rotate();
        self.handle_hard_drop();
        self.apply_gravity();
        self.handle_shift();
        self.apply_line_clear();

        self.previous_input = self.input;
    }

    fn apply_gravity(&mut self) {
        if let Some(mut current_piece) = self.current_piece.clone() {
            if current_piece.check_shift_collision(&self.board, 0, 1) {
                if current_piece.piece_position.1 > current_piece.previous_lock_y {
                    self.sound_queue.push(Sound::Bottom);
                }
                current_piece.previous_lock_y = current_piece.piece_position.1;
                self.lock_counter += 1;
                if self.lock_counter >= self.lock_delay {
                    self.board.set_piece(&current_piece).unwrap();
                    self.current_piece = None;
                    self.are_counter = Some(self.are);
                    self.lock_counter = 0;
                    self.sound_queue.push(Sound::Lock);
                    return;
                }
            }

            self.shift_down_counter += if self.input.soft_drop && self.gravity < 1.0 {
                1.0
            } else {
                self.gravity
            };
            if self.shift_down_counter >= 1.0 {
                let fall_size = self.shift_down_counter as i32;
                self.shift_down_counter = 0.0;

                if current_piece.shift(&self.board, 0, fall_size) {
                    // falling down successful
                } else {
                    for i in 0..fall_size {
                        if current_piece.shift(&self.board, 0, fall_size - i) {
                            break;
                        };
                    }
                }
            }
            // let sound = self.get_next_sound();
            // self.sound_queue.push(sound);
            self.current_piece = Some(current_piece);
        }
    }

    fn apply_line_clear(&mut self) {
        if let Some(_lines) = self.board.line_clear() {
            self.sound_queue.push(Sound::Erase);
            self.line_clear_lock_timer = Some(self.line_clear_lock);
            self.are_counter = Some(self.line_are);
        };
    }

    fn handle_rotate(&mut self) {
        if !self.previous_input.cw && self.input.cw {
            if let Some(current_piece) = self.current_piece.as_mut() {
                if current_piece.cw(&self.board) {
                    self.lock_counter = 0;
                };
            }
        } else if !self.previous_input.ccw && self.input.ccw {
            if let Some(current_piece) = self.current_piece.as_mut() {
                if current_piece.ccw(&self.board) {
                    self.lock_counter = 0;
                };
            }
        }
    }

    fn handle_hold(&mut self) {
        if !self.hold_used && self.input.hold {
            let sound = self.get_next_sound();
            if let Some(current_piece) = self.current_piece.as_mut() {
                let swapped = self
                    .board
                    .swap_hold_piece(current_piece.piece_state.get_kind());
                let new_piece = if let Some(swapped) = swapped {
                    FallingPiece::from_piece_state(PieceState::from_piece(swapped))
                } else {
                    FallingPiece::from_piece_state(PieceState::from_piece(self.board.pop_next()))
                };
                self.sound_queue.push(sound);
                *current_piece = new_piece;
                self.hold_used = true;
                self.sound_queue.push(Sound::Hold);
            }
        }
    }

    fn handle_hard_drop(&mut self) {
        if !self.previous_input.hard_drop && self.input.hard_drop {
            if let Some(current_piece) = self.current_piece.as_mut() {
                self.sound_queue.push(Sound::Bottom);

                for i in 0..20 {
                    if current_piece.check_shift_collision(&self.board, 0, i) {
                        current_piece.shift(&self.board, 0, i - 1);
                        self.board.set_piece(&current_piece).unwrap();
                        self.current_piece = None;
                        self.lock_counter = 0;
                        self.are_counter = Some(self.are);
                        break;
                    };
                }
            }
        }
    }

    fn handle_shift(&mut self) {
        if self.input.left {
            if self.das_state != DasState::Left {
                self.das_state = DasState::Left;
                self.das_counter = 0
            }
            if self.das_counter == 0 {
                self.das_counter += 1;
                if let Some(current_piece) = self.current_piece.as_mut() {
                    if current_piece.shift(&self.board, 1, 0) {
                        self.lock_counter = 0;
                    };
                }
            }
            if self.input.left == self.previous_input.left {
                self.das_counter += 1;
                if self.das_counter >= self.das {
                    if let Some(current_piece) = self.current_piece.as_mut() {
                        if current_piece.shift(&self.board, 1, 0) {
                            self.lock_counter = 0;
                        };
                    }
                }
            }
        } else if self.input.right {
            if self.das_state != DasState::Right {
                self.das_state = DasState::Right;
                self.das_counter = 0
            }
            if self.das_counter == 0 {
                self.das_counter += 1;
                if let Some(current_piece) = self.current_piece.as_mut() {
                    if current_piece.shift(&self.board, -1, 0) {
                        self.lock_counter = 0;
                    };
                }
            }
            if self.input.right == self.previous_input.right {
                self.das_counter += 1;
                if self.das_counter >= self.das {
                    if let Some(current_piece) = self.current_piece.as_mut() {
                        if current_piece.shift(&self.board, -1, 0) {
                            self.lock_counter = 0;
                        };
                    }
                }
            }
        }
        if !self.input.left && !self.input.right {
            self.das_state = DasState::None;
            self.das_counter = 0
        }
    }

    pub fn get_board(&self) -> Board {
        self.board.clone()
    }

    pub fn get_hold(&self) -> Option<Piece> {
        self.board.hold_piece.clone()
    }

    pub fn get_next(&self) -> Piece {
        self.board.next_pieces[0]
    }

    pub fn get_next_next(&self) -> Piece {
        self.board.next_pieces[1]
    }

    pub fn get_next_next_next(&self) -> Piece {
        self.board.next_pieces[2]
    }

    pub fn get_sound_queue(&mut self) -> &mut Vec<Sound> {
        self.sound_queue.as_mut()
    }

    pub fn get_next_sound(&self) -> Sound {
        match self.get_next() {
            Piece::I => Sound::PieceI,
            Piece::O => Sound::PieceO,
            Piece::T => Sound::PieceT,
            Piece::L => Sound::PieceL,
            Piece::J => Sound::PieceJ,
            Piece::S => Sound::PieceS,
            Piece::Z => Sound::PieceZ,
        }
    }

    pub fn set_input(&mut self, input: Input) {
        self.input = input;
    }
}
