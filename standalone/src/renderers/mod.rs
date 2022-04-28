use graphics::{types::Color, Context};
use piston_window::{G2d, GfxDevice, Glyphs, RenderArgs};
use tetris::Board;

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
struct PieceLineInfo(usize, usize, PieceLine);

impl PieceLineInfo {
    fn to_from_to(&self) -> ([f64; 2], [f64; 2]) {
        let &PieceLineInfo(x, y, line) = self;
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
        for x in 0..10usize {
            for y in (0usize + cell_offset_y)..(20 + cell_offset_y) {
                if let Some(has_center) = self.exists(x, y) {
                    if has_center {
                        continue;
                    }
                    for offset_x in -1..=1 {
                        for offset_y in -1..=1 {
                            let target_x = (x as isize).saturating_add(offset_x) as usize;
                            let target_y = (y as isize).saturating_add(offset_y) as usize;
                            if let Some(exist) = self.exists(target_x, target_y) {
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
                                line_infos.push(PieceLineInfo(x, y - cell_offset_y, kind))
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
