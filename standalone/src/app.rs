use fps_counter::FPSCounter;
use graphics::{
    clear, line_from_to, math::Matrix2d, rectangle, rectangle::square, types::Color, Text,
};
use piston_window::{
    Button, ButtonArgs, ButtonState, Context, G2d, GfxDevice, Glyphs, Graphics, RenderArgs,
    Transformed,
};
use tetris::{Board, Cell, FallingPiece, Game, Input, Piece};

use crate::settings::Settings;

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

pub struct App {
    fps: FPSCounter,
    glyphs: Glyphs,
    game: Game,   // Game
    input: Input, // Input
    settings: Settings,
    locked_piece: Option<FallingPiece>,
}
pub const CELL_SIZE: f64 = 16.0;

impl App {
    pub fn new(game: Game, settings: Settings, glyphs: Glyphs) -> Self {
        Self {
            fps: FPSCounter::default(),
            glyphs,
            game,
            input: Default::default(),
            settings,
            locked_piece: None,
        }
    }

    pub fn render(&mut self, _args: &RenderArgs, c: Context, g2d: &mut G2d, d: &mut GfxDevice) {
        clear(BLACK.to_color(), g2d);

        App::render_hold(
            c.transform
                .trans(CELL_SIZE * 1.5, CELL_SIZE * 2.5)
                .scale(0.5, 0.5),
            g2d,
            self.game.get_hold(),
        );
        App::render_next(
            c.transform.trans(CELL_SIZE * 5.0, CELL_SIZE * 2.0),
            g2d,
            self.game.get_next(),
        );

        App::render_nexts(
            c.transform
                .trans(CELL_SIZE * 6.5, CELL_SIZE * 2.5)
                .scale(0.5, 0.5),
            g2d,
            self.game.get_next_next(),
            self.game.get_next_next_next(),
        );

        self.render_board(c.transform.trans(CELL_SIZE * 1.0, CELL_SIZE * 4.0), g2d);
        App::render_board_pieces_outline(
            c.transform.trans(CELL_SIZE * 1.0, CELL_SIZE * 4.0),
            g2d,
            &self.game.get_board(),
        );
        App::render_board_outline(
            c.transform.trans(CELL_SIZE * 1.0, CELL_SIZE * 4.0),
            g2d,
            1.0,
        );
        App::render_current_piece(
            c.transform.trans(CELL_SIZE * 1.0, CELL_SIZE * 4.0),
            g2d,
            &self.game.current_piece,
        );

        let fps = self.fps.tick();
        let fps_text = format!("{fps} fps");

        Text::new_color(WHITE.to_color(), 8)
            .draw(
                &fps_text,
                &mut self.glyphs,
                &c.draw_state,
                c.transform.trans(224.0, 11.0),
                g2d,
            )
            .unwrap();

        Text::new_color(WHITE.to_color(), 8)
            .draw(
                "next",
                &mut self.glyphs,
                &c.draw_state,
                c.transform.trans(16.0, 20.0),
                g2d,
            )
            .unwrap();
        self.glyphs.factory.encoder.flush(d);
    }

    fn render_board_outline<G: Graphics>(transform: Matrix2d, g: &mut G, radius: f64) {
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

    fn render_board<G: Graphics>(&mut self, transform: Matrix2d, g: &mut G) {
        let square = [0.0, 0.0, 10.0 * CELL_SIZE, 20.0 * CELL_SIZE];
        let background_transform = transform;
        rectangle(BLACK.to_color(), square, background_transform, g);
        let cell_offset_y = 20;
        let board = self.game.get_board();
        for (y, cells_x) in board.cells.iter().enumerate() {
            if y < cell_offset_y {
                continue;
            };
            for (x, cell) in cells_x.iter().enumerate() {
                match cell {
                    Some(cell) => {
                        App::render_cell(transform, g, x as _, (y - cell_offset_y) as _, cell);
                    }
                    None => {}
                }
            }
        }
        if let Some(locked_piece) = self.locked_piece {
            let cell_offset_y = 20;
            let pos = locked_piece.piece_position;
            let cell = Cell::White;

            for (rel_x, rel_y) in locked_piece.piece_state.get_cells().iter() {
                let exist = board.cells[(-rel_y + pos.1 as i16) as usize]
                    [(rel_x + pos.0 as i16) as usize]
                    .is_some();
                if exist {
                    App::render_cell(
                        transform,
                        g,
                        (rel_x + pos.0 as i16) as _,
                        ((-rel_y - cell_offset_y as i16) + pos.1 as i16) as _,
                        &cell,
                    );
                }
            }
            self.locked_piece = None
        }
    }

    fn render_board_pieces_outline<G: Graphics>(transform: Matrix2d, g: &mut G, board: &Board) {
        for line_info in board.get_line_infos().iter() {
            let (from, to) = line_info.to_from_to();
            use PieceLine::*;
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
        transform: Matrix2d,
        g: &mut G,
        current_piece: &Option<FallingPiece>,
    ) {
        if let Some(current_piece) = current_piece {
            let cell_offset_y = 20;
            let pos = current_piece.piece_position;
            let cell = current_piece.piece_state.get_kind().into();

            for (rel_x, rel_y) in current_piece.piece_state.get_cells().iter() {
                App::render_cell(
                    transform,
                    g,
                    (rel_x + pos.0 as i16) as _,
                    ((-rel_y - cell_offset_y as i16) + pos.1 as i16) as _,
                    &cell,
                );
            }
        };
    }

    fn render_hold<G: Graphics>(transform: Matrix2d, g: &mut G, hold: Option<Piece>) {
        if let Some(hold) = hold {
            App::render_piece(transform, g, hold);
        };
    }

    fn render_piece<G: Graphics>(transform: Matrix2d, g: &mut G, piece: Piece) {
        let cell = piece.into();
        for (rel_x, rel_y) in piece.get_cells().into_iter() {
            App::render_cell(transform, g, rel_x as _, -rel_y as _, &cell);
        }
    }

    fn render_next<G: Graphics>(transform: Matrix2d, g: &mut G, next: Piece) {
        App::render_piece(transform, g, next);
    }

    fn render_nexts<G: Graphics>(
        transform: Matrix2d,
        g: &mut G,
        next_next: Piece,
        next_next_next: Piece,
    ) {
        App::render_piece(transform.trans(CELL_SIZE * 4.0, 0.0), g, next_next);
        App::render_piece(transform.trans(CELL_SIZE * 9.0, 0.0), g, next_next_next);
    }

    fn render_cell<G: Graphics>(transform: Matrix2d, g: &mut G, x: i32, y: i32, cell: &Cell) {
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

    pub fn update(&mut self) {
        self.game.set_input(self.input);
        self.game.update();
        {
            let sound_queue = self.game.get_sound_queue();
            while sound_queue.len() > 0 {
                if let Some(sound) = sound_queue.pop() {
                    music::play_sound(&sound, music::Repeat::Times(0), 0.25);
                };
            }
        }
        {
            let event_queue = self.game.get_event_queue();
            while event_queue.len() > 0 {
                if let Some(event) = event_queue.pop() {
                    match event {
                        tetris::TetrisEvent::PieceLocked(piece) => {
                            self.locked_piece = Some(piece);
                        }
                        _ => unimplemented!(),
                    }
                };
            }
        }
    }

    pub fn input(&mut self, args: &ButtonArgs) {
        let state = match args.state {
            ButtonState::Press => true,
            ButtonState::Release => false,
        };
        match args.button {
            Button::Keyboard(key) => match key {
                _ => {
                    if self.settings.key.left == key.code() as _ {
                        self.input.left = state;
                    }
                    if self.settings.key.right == key.code() as _ {
                        self.input.right = state;
                    }
                    if self.settings.key.hard_drop == key.code() as _ {
                        self.input.hard_drop = state;
                    }
                    if self.settings.key.soft_drop == key.code() as _ {
                        self.input.soft_drop = state;
                    }
                    if self.settings.key.cw == key.code() as _ {
                        self.input.cw = state;
                    }
                    if self.settings.key.ccw == key.code() as _ {
                        self.input.ccw = state;
                    }
                    if self.settings.key.hold == key.code() as _ {
                        self.input.hold = state;
                    }
                    if self.settings.key.restart == key.code() as _ && !state {
                        self.game = Game::from_settings(
                            self.settings.game.gravity,
                            self.settings.game.are,
                            self.settings.game.line_are,
                            self.settings.game.das,
                            self.settings.game.lock_delay,
                            self.settings.game.line_clear_delay,
                        );
                    }

                    println!("key.code(): {}", key.code());
                    // self.settings.key_config
                }
            },
            _ => {}
        }
    }
}
