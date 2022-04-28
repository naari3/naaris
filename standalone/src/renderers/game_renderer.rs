use graphics::{
    line_from_to, math::Matrix2d, rectangle, rectangle::square, Context, Graphics, Transformed,
};
use piston_window::{G2d, GfxDevice, Glyphs, RenderArgs};
use tetris::{Board, Cell, FallingPiece, Game, GameState, Piece};

use crate::{
    renderers::{ORANGE, RED, YELLOW},
    CELL_SIZE,
};

use super::{
    standard_render, GetNeighbor, RenderInner, Renderer, ToColor, BLACK, BLUE, CYAN, GRAY, GREEN,
    PURPLE, WHITE,
};

impl Renderer for Game {
    fn render(
        &mut self,
        args: &RenderArgs,
        c: Context,
        g2d: &mut G2d,
        d: &mut GfxDevice,
        glyphs: &mut Glyphs,
    ) {
        standard_render(self, args, c, g2d, d, glyphs)
    }
}

impl RenderInner for Game {
    fn render_board_outline<G: Graphics>(&self, transform: Matrix2d, g: &mut G, radius: f64) {
        let color = GRAY.to_color();

        // let left_top = [0.0, 0.0];
        // let right_top = [10.0 * CELL_SIZE, 0.0];
        // let left_bottom = [0.0, 20.0 * CELL_SIZE];
        // let right_bottom = [10.0 * CELL_SIZE, 20.0 * CELL_SIZE];

        let left = [[0.0, 0.0 + radius], [0.0, 20.0 * CELL_SIZE - radius]];
        let right = [
            [10.0 * CELL_SIZE, 0.0 + radius],
            [10.0 * CELL_SIZE, 20.0 * CELL_SIZE - radius],
        ];
        let top = [[0.0 - radius, 0.0], [10.0 * CELL_SIZE + radius, 0.0]];
        let bottom = [
            [0.0 - radius, 20.0 * CELL_SIZE],
            [10.0 * CELL_SIZE + radius, 20.0 * CELL_SIZE],
        ];

        line_from_to(color, radius, left[0], left[1], transform, g);
        line_from_to(color, radius, right[0], right[1], transform, g);
        line_from_to(color, radius, top[0], top[1], transform, g);
        line_from_to(color, radius, bottom[0], bottom[1], transform, g);
    }

    fn render_board<G: Graphics>(&self, transform: Matrix2d, g: &mut G) {
        let square = [0.0, 0.0, 10.0 * CELL_SIZE, 20.0 * CELL_SIZE];
        let background_transform = transform;
        rectangle(BLACK.to_color(), square, background_transform, g);
        let cell_offset_y = 20;
        let board = self.get_board();
        for (y, cells_x) in board.cells.iter().enumerate() {
            if y < cell_offset_y {
                continue;
            };
            for (x, cell) in cells_x.iter().enumerate() {
                match cell {
                    Some(cell) => {
                        self.render_cell(transform, g, x as _, (y - cell_offset_y) as _, cell);
                    }
                    None => {}
                }
            }
        }
        if let Some(locked_piece) = self.get_locked_piece() {
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
        for line_info in board.get_line_infos().iter() {
            let (from, to) = line_info.to_from_to();
            use super::PieceLine::*;
            match line_info.2 {
                Top | Bottom | Left | Right => {
                    line_from_to(WHITE.to_color(), 0.5, from, to, transform, g)
                }
                TopLeft | TopRight | BottomLeft | BottomRight => {
                    let p = square(0.0, 0.0, 1.0);
                    rectangle(WHITE.to_color(), p, transform.trans(from[0], from[1]), g);
                }
            }
        }
    }

    fn render_current_piece<G: Graphics>(
        &self,
        transform: Matrix2d,
        g: &mut G,
        current_piece: &Option<FallingPiece>,
    ) {
        if let Some(current_piece) = current_piece {
            let cell_offset_y = 20;
            let pos = current_piece.piece_position;
            let cell = current_piece.piece_state.get_kind().into();

            for (rel_x, rel_y) in current_piece.piece_state.get_cells().iter() {
                self.render_cell(
                    transform,
                    g,
                    (rel_x + pos.0 as i16) as _,
                    ((-rel_y - cell_offset_y as i16) + pos.1 as i16) as _,
                    &cell,
                );
            }
        };
    }

    fn render_hold<G: Graphics>(&self, transform: Matrix2d, g: &mut G, hold: Option<Piece>) {
        if let Some(hold) = hold {
            self.render_piece(transform, g, hold);
        };
    }

    fn render_piece<G: Graphics>(&self, transform: Matrix2d, g: &mut G, piece: Piece) {
        let cell = piece.into();
        for (rel_x, rel_y) in piece.get_cells().into_iter() {
            self.render_cell(transform, g, rel_x as _, -rel_y as _, &cell);
        }
    }

    fn render_next<G: Graphics>(&self, transform: Matrix2d, g: &mut G, next: Piece) {
        self.render_piece(transform, g, next);
    }

    fn render_nexts<G: Graphics>(
        &self,
        transform: Matrix2d,
        g: &mut G,
        next_next: Piece,
        next_next_next: Piece,
    ) {
        self.render_piece(transform.trans(CELL_SIZE * 4.0, 0.0), g, next_next);
        self.render_piece(transform.trans(CELL_SIZE * 9.0, 0.0), g, next_next_next);
    }

    fn render_cell<G: Graphics>(
        &self,
        transform: Matrix2d,
        g: &mut G,
        x: i32,
        y: i32,
        cell: &Cell,
    ) {
        use tetris::Cell::*;

        let square = rectangle::square(0.0, 0.0, CELL_SIZE - 1.0);
        // let square = rectangle::square(x as f64 * CELL_SIZE, (y as f64) * CELL_SIZE, CELL_SIZE);

        let transform = transform.trans(x as f64 * CELL_SIZE, y as f64 * CELL_SIZE);
        // let transform = IDENTITY;
        let color = match cell {
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
        };
        rectangle(color.to_color(), square, transform, g);
    }
}
