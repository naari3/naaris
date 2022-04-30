use graphics::{
    line_from_to, math::Matrix2d, rectangle, rectangle::square, Context, Graphics, Text,
    Transformed,
};
use piston_window::{G2d, GfxDevice, Glyphs, RenderArgs};
use tetris::{Board, Cell, GameState, Roll, Status, TGM3Master};

use crate::{
    renderers::{PieceLineInfo, BLACK, BLUE, CYAN, GRAY, GREEN, ORANGE, PURPLE, RED, YELLOW},
    sound::StandaloneSound,
    CELL_SIZE,
};

use super::{standard_render, GetNeighbor, RenderInner, Renderer, ToColor, WHITE};

impl Renderer for TGM3Master {
    fn render(
        &mut self,
        args: &RenderArgs,
        c: Context,
        g2d: &mut G2d,
        d: &mut GfxDevice,
        glyphs: &mut Glyphs,
    ) {
        standard_render(self, args, c, g2d, d, glyphs);
        {
            let event_queue = self.get_tgm3events();
            while event_queue.len() > 0 {
                if let Some(event) = event_queue.pop() {
                    match event {
                        // tetris::TetrisEvent::PieceLocked() => {}
                        _ => {}
                    }
                };
            }
        }
        {
            let sound_queue = self.get_tgm3sounds();
            while sound_queue.len() > 0 {
                if let Some(sound) = sound_queue.pop() {
                    music::play_sound::<StandaloneSound>(
                        &sound.into(),
                        music::Repeat::Times(0),
                        0.25,
                    );
                };
            }
        }

        Text::new_color(WHITE.to_color(), 8)
            .draw(
                &format!("{:0>3}", self.get_level()),
                glyphs,
                &c.draw_state,
                c.transform.trans(208.0, 336.0),
                g2d,
            )
            .unwrap();

        let rank = if self.get_level() > 899 {
            999
        } else {
            self.get_level() / 100 * 100 + 100
        };
        Text::new_color(WHITE.to_color(), 8)
            .draw(
                &format!("{: >3}", rank),
                glyphs,
                &c.draw_state,
                c.transform.trans(208.0, 352.0),
                g2d,
            )
            .unwrap();

        if let Status::End = self.get_status() {
            rectangle(
                BLACK.to_color(),
                [0.0, 0.0, CELL_SIZE * 9.0, CELL_SIZE * 3.0],
                c.transform.trans(CELL_SIZE * 1.5, CELL_SIZE * 12.0),
                g2d,
            );
            let grade = [
                "9", "8", "7", "6", "5", "4", "3", "2", "1", "S1", "S2", "S3", "S4", "S5", "S6",
                "S7", "S8", "S9", "m1", "m2", "m3", "m4", "m5", "m6", "m7", "m8", "m9", "M", "MK",
                "MV", "MO", "MM", "GM",
            ][self.get_aggregate_grade()];
            Text::new_color(WHITE.to_color(), 8)
                .draw(
                    &format!("congrats!: {grade}"),
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(CELL_SIZE * 2.0, CELL_SIZE * 13.0),
                    g2d,
                )
                .unwrap();
        }
    }
}

impl RenderInner for TGM3Master {
    fn render_board_outline<G: Graphics>(&self, transform: Matrix2d, g: &mut G, radius: f64) {
        self.inner.render_board_outline(transform, g, radius);
    }

    fn render_board<G: Graphics>(&self, transform: Matrix2d, g: &mut G) {
        let square = [0.0, 0.0, 10.0 * CELL_SIZE, 20.0 * CELL_SIZE];
        let background_transform = transform;
        rectangle(BLACK.to_color(), square, background_transform, g);
        let cell_offset_y = 20;
        let board = self.inner.get_board();
        for (y, cells_x) in board.cells.iter().enumerate() {
            if y < cell_offset_y {
                continue;
            };
            for (x, cell) in cells_x.iter().enumerate() {
                match cell {
                    Some(cell) => {
                        let cell_offset_y = 20;
                        let opacity_timers = self.get_opacity_timers();
                        let timer = opacity_timers.get(y as usize).map_or(None, |timers_x| {
                            timers_x.get(x as usize).clone().unwrap_or(&None).as_ref()
                        });
                        let timer = timer.map(|t| *t);

                        match self.get_status() {
                            Status::Roll(roll) => {
                                tgm3_roll_render_cell(
                                    transform,
                                    g,
                                    x as _,
                                    (y - cell_offset_y) as _,
                                    cell,
                                    timer,
                                    roll,
                                );
                            }
                            _ => {
                                self.render_cell(
                                    transform,
                                    g,
                                    x as _,
                                    (y - cell_offset_y) as _,
                                    cell,
                                );
                            }
                        };
                    }
                    None => {}
                }
            }
        }
        if let Some(locked_piece) = self.inner.get_locked_piece() {
            let cell_offset_y = 20;
            let pos = locked_piece.piece_position;
            let cell = Cell::White;

            for (rel_x, rel_y) in locked_piece.piece_state.get_cells().iter() {
                let exist = board.cells[(-rel_y + pos.1 as i16) as usize]
                    [(rel_x + pos.0 as i16) as usize]
                    .is_some();
                if exist {
                    self.render_cell(
                        transform,
                        g,
                        (rel_x + pos.0 as i16) as _,
                        ((-rel_y - cell_offset_y as i16) + pos.1 as i16) as _,
                        &cell,
                    );
                }
            }
        }
    }

    fn render_board_pieces_outline<G: Graphics>(
        &self,
        transform: Matrix2d,
        g: &mut G,
        board: &Board,
    ) {
        let timers = self.get_opacity_timers();
        let cell_offset_y = 20;

        for line_info in board.get_line_infos().iter() {
            let &PieceLineInfo(_, _, _, (target_x, target_y)) = line_info;
            let mut color = WHITE.to_color();
            if target_y > 50 {
                println!("target_y: {target_y}");
            }
            if let Status::Roll(roll) = self.get_status() {
                let timer = if target_y >= 0 {
                    timers
                        .get((target_y + cell_offset_y as isize) as usize)
                        .map_or(None, |timers_x| timers_x.get(target_x as usize))
                        .unwrap_or(&None)
                        .as_ref()
                        .map(|&t| t)
                } else {
                    None
                };
                color[3] = timer_to_opacity(timer, roll);
            }
            let (from, to) = line_info.to_from_to();
            use super::PieceLine::*;
            match line_info.2 {
                Top | Bottom | Left | Right => line_from_to(color, 0.5, from, to, transform, g),
                TopLeft | TopRight | BottomLeft | BottomRight => {
                    let p = square(0.0, 0.0, 1.0);
                    rectangle(color, p, transform.trans(from[0], from[1]), g);
                }
            }
        }
    }

    fn render_current_piece<G: Graphics>(
        &self,
        transform: Matrix2d,
        g: &mut G,
        current_piece: &Option<tetris::FallingPiece>,
    ) {
        self.inner.render_current_piece(transform, g, current_piece);
    }

    fn render_hold<G: Graphics>(
        &self,
        transform: Matrix2d,
        g: &mut G,
        hold: Option<tetris::Piece>,
    ) {
        self.inner.render_hold(transform, g, hold);
    }

    fn render_piece<G: Graphics>(&self, transform: Matrix2d, g: &mut G, piece: tetris::Piece) {
        self.inner.render_piece(transform, g, piece);
    }

    fn render_next<G: Graphics>(&self, transform: Matrix2d, g: &mut G, next: tetris::Piece) {
        self.inner.render_next(transform, g, next);
    }

    fn render_nexts<G: Graphics>(
        &self,
        transform: Matrix2d,
        g: &mut G,
        next_next: tetris::Piece,
        next_next_next: tetris::Piece,
    ) {
        self.inner
            .render_nexts(transform, g, next_next, next_next_next);
    }

    fn render_cell<G: Graphics>(
        &self,
        transform: Matrix2d,
        g: &mut G,
        x: i32,
        y: i32,
        cell: &Cell,
    ) {
        self.inner.render_cell(transform, g, x, y, cell);
    }
}

fn tgm3_roll_render_cell<G: Graphics>(
    transform: Matrix2d,
    g: &mut G,
    x: i32,
    y: i32,
    cell: &Cell,
    timer: Option<usize>,
    roll: Roll,
) {
    use tetris::Cell::*;

    let square = rectangle::square(0.0, 0.0, CELL_SIZE - 1.0);

    let transform = transform.trans(x as f64 * CELL_SIZE, y as f64 * CELL_SIZE);
    let mut color = match cell {
        Black => BLACK,
        White => WHITE,
        Red => RED,
        Orange => ORANGE,
        Yellow => YELLOW,
        Green => GREEN,
        Cyan => CYAN,
        Blue => BLUE,
        Purple => PURPLE,
        Glay => GRAY,
    }
    .to_color();

    color[3] = timer_to_opacity(timer, roll);
    rectangle(color, square, transform, g);
}

fn timer_to_opacity(timer: Option<usize>, roll: Roll) -> f32 {
    match roll {
        Roll::Normal => {
            if let Some(timer) = timer {
                if timer <= 60 {
                    return timer as f32 / 60.0;
                }
            }
            1.0
        }
        Roll::Invisible => {
            if let Some(timer) = timer {
                if timer <= 4 {
                    return timer as f32 / 4.0;
                }
            }
            1.0
        }
    }
}
