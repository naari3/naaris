use graphics::{math::Matrix2d, types::Color, Context, Graphics, Text, Transformed};
use piston_window::{G2d, GfxDevice, Glyphs, RenderArgs};
use tetris::{Board, Cell, FallingPiece, GameState, Piece};

use crate::CELL_SIZE;

pub mod game_renderer;
pub mod tgm3master_renderer;

pub trait Renderer {
    fn render(
        &mut self,
        args: &RenderArgs,
        c: Context,
        g2d: &mut G2d,
        d: &mut GfxDevice,
        glyphs: &mut Glyphs,
    );
}

pub trait RenderInner {
    fn render_board_outline<G: Graphics>(&self, transform: Matrix2d, g: &mut G, radius: f64);
    fn render_board<G: Graphics>(&self, transform: Matrix2d, g: &mut G);
    fn render_board_pieces_outline<G: Graphics>(
        &self,
        transform: Matrix2d,
        g: &mut G,
        board: &Board,
    );
    fn render_current_piece<G: Graphics>(
        &self,
        transform: Matrix2d,
        g: &mut G,
        current_piece: &Option<FallingPiece>,
    );
    fn render_hold<G: Graphics>(&self, transform: Matrix2d, g: &mut G, hold: Option<Piece>);
    fn render_piece<G: Graphics>(&self, transform: Matrix2d, g: &mut G, piece: Piece);
    fn render_next<G: Graphics>(&self, transform: Matrix2d, g: &mut G, next: Piece);
    fn render_nexts<G: Graphics>(
        &self,
        transform: Matrix2d,
        g: &mut G,
        next_next: Piece,
        next_next_next: Piece,
    );
    fn render_cell<G: Graphics>(&self, transform: Matrix2d, g: &mut G, x: i32, y: i32, cell: &Cell);
}

trait ToColor {
    fn to_color(&self) -> Color;
}

impl ToColor for [u8; 4] {
    fn to_color(&self) -> [f32; 4] {
        [
            self[0] as f32 / 255.0,
            self[1] as f32 / 255.0,
            self[2] as f32 / 255.0,
            self[3] as f32 / 255.0,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum PieceLine {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PieceLineInfo(isize, isize, PieceLine, (isize, isize));

impl PieceLineInfo {
    fn to_from_to(&self) -> ([f64; 2], [f64; 2]) {
        let &PieceLineInfo(x, y, line, _) = self;
        let x = x as f64;
        let y = y as f64;
        match line {
            PieceLine::Top => (
                [CELL_SIZE * x, CELL_SIZE * y - 1.0],
                [CELL_SIZE * (x + 1.0), CELL_SIZE * y - 1.0],
            ),
            PieceLine::Bottom => (
                [CELL_SIZE * x, CELL_SIZE * (y + 1.0)],
                [CELL_SIZE * (x + 1.0), CELL_SIZE * (y + 1.0)],
            ),
            PieceLine::Left => (
                [CELL_SIZE * x, CELL_SIZE * y],
                [CELL_SIZE * x, CELL_SIZE * (y + 1.0)],
            ),
            PieceLine::Right => (
                [CELL_SIZE * (x + 1.0) + 1.0, CELL_SIZE * y],
                [CELL_SIZE * (x + 1.0) + 1.0, CELL_SIZE * (y + 1.0)],
            ),
            PieceLine::TopLeft => (
                [CELL_SIZE * x - 1.0, CELL_SIZE * y - 1.0],
                [CELL_SIZE * x - 1.0, CELL_SIZE * y - 1.0],
            ),
            PieceLine::TopRight => (
                [CELL_SIZE * (x + 1.0), CELL_SIZE * y - 1.0],
                [CELL_SIZE * (x + 1.0), CELL_SIZE * y - 1.0],
            ),
            PieceLine::BottomLeft => (
                [CELL_SIZE * x - 1.0, CELL_SIZE * (y + 1.0)],
                [CELL_SIZE * x - 1.0, CELL_SIZE * (y + 1.0)],
            ),
            PieceLine::BottomRight => (
                [CELL_SIZE * (x + 1.0), CELL_SIZE * (y + 1.0)],
                [CELL_SIZE * (x + 1.0), CELL_SIZE * (y + 1.0)],
            ),
        }
    }
}

trait GetNeighbor {
    fn exists(&self, x: usize, y: usize) -> Option<bool>;
    fn get_line_infos(&self) -> Vec<PieceLineInfo>;
}

impl GetNeighbor for Board {
    fn exists(&self, x: usize, y: usize) -> Option<bool> {
        if let Some(cells_x) = self.cells.get(y) {
            if let Some(cell_x) = cells_x.get(x) {
                return Some(cell_x.is_some());
            }
        }
        None
    }

    fn get_line_infos(&self) -> Vec<PieceLineInfo> {
        let cell_offset_y = 20;
        let mut line_infos = vec![];
        for x in 0..10isize {
            for y in (0isize + cell_offset_y)..(20 + cell_offset_y) {
                if let Some(has_center) = self.exists(x as _, y as _) {
                    if has_center {
                        continue;
                    }
                    for offset_x in -1..=1 {
                        for offset_y in -1..=1 {
                            let target_x = (x as isize) + offset_x;
                            let target_y = (y as isize) + offset_y;
                            if let Some(exist) = self.exists(target_x as _, target_y as _) {
                                if !exist {
                                    continue;
                                }
                                let kind = match (offset_x, offset_y) {
                                    (-1, -1) => PieceLine::TopLeft,
                                    (0, -1) => PieceLine::Top,
                                    (1, -1) => PieceLine::TopRight,
                                    (-1, 0) => PieceLine::Left,
                                    (1, 0) => PieceLine::Right,
                                    (-1, 1) => PieceLine::BottomLeft,
                                    (0, 1) => PieceLine::Bottom,
                                    (1, 1) => PieceLine::BottomRight,
                                    (_, _) => unreachable!(),
                                };
                                line_infos.push(PieceLineInfo(
                                    x,
                                    y - cell_offset_y,
                                    kind,
                                    (
                                        (x as isize + offset_x),
                                        ((y - cell_offset_y) as isize + offset_y),
                                    ),
                                ))
                            }
                        }
                    }
                }
            }
        }
        line_infos
    }
}

const BLACK: [u8; 4] = [0, 0, 0, 255];
const WHITE: [u8; 4] = [255, 255, 255, 255];
const RED: [u8; 4] = [215, 15, 55, 255];
const ORANGE: [u8; 4] = [227, 91, 2, 255];
const YELLOW: [u8; 4] = [227, 159, 2, 255];
const GREEN: [u8; 4] = [89, 177, 1, 255];
const CYAN: [u8; 4] = [15, 155, 215, 255];
const BLUE: [u8; 4] = [33, 65, 198, 255];
const PURPLE: [u8; 4] = [175, 41, 138, 255];
const GRAY: [u8; 4] = [107, 107, 107, 255];

pub fn standard_render<G: GameState + RenderInner>(
    game: &mut G,
    _args: &RenderArgs,
    c: Context,
    g2d: &mut G2d,
    _d: &mut GfxDevice,
    glyphs: &mut Glyphs,
) {
    game.render_hold(
        c.transform
            .trans(CELL_SIZE * 1.5, CELL_SIZE * 2.5)
            .scale(0.5, 0.5),
        g2d,
        game.get_hold(),
    );
    game.render_next(
        c.transform.trans(CELL_SIZE * 5.0, CELL_SIZE * 2.0),
        g2d,
        game.get_next(),
    );

    game.render_nexts(
        c.transform
            .trans(CELL_SIZE * 6.5, CELL_SIZE * 2.5)
            .scale(0.5, 0.5),
        g2d,
        game.get_next_next(),
        game.get_next_next_next(),
    );

    game.render_board(c.transform.trans(CELL_SIZE * 1.0, CELL_SIZE * 4.0), g2d);
    game.render_board_pieces_outline(
        c.transform.trans(CELL_SIZE * 1.0, CELL_SIZE * 4.0),
        g2d,
        &game.get_board(),
    );
    game.render_board_outline(
        c.transform.trans(CELL_SIZE * 1.0, CELL_SIZE * 4.0),
        g2d,
        1.0,
    );
    game.render_current_piece(
        c.transform.trans(CELL_SIZE * 1.0, CELL_SIZE * 4.0),
        g2d,
        &game.get_current_piece(),
    );

    Text::new_color(WHITE.to_color(), 8)
        .draw(
            "next",
            glyphs,
            &c.draw_state,
            c.transform.trans(16.0, 20.0),
            g2d,
        )
        .unwrap();
    // glyphs.factory.encoder.flush(d);
}
