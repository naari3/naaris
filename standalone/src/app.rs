use fps_counter::FPSCounter;
use graphics::{clear, line_from_to, math::Matrix2d, rectangle, types::Color, Text};
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

        App::render_board_outline(
            c.transform.trans(CELL_SIZE * 1.0, CELL_SIZE * 4.0),
            g2d,
            2.0,
        );
        App::render_board(
            c.transform.trans(CELL_SIZE * 1.0, CELL_SIZE * 4.0),
            g2d,
            &self.game.get_board(),
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

    fn render_board<G: Graphics>(transform: Matrix2d, g: &mut G, board: &Board) {
        let square = [0.0, 0.0, 10.0 * CELL_SIZE, 20.0 * CELL_SIZE];
        let background_transform = transform;
        rectangle(BLACK.to_color(), square, background_transform, g);
        let cell_offset_y = 20;
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

        let square = rectangle::square(0.0, 0.0, CELL_SIZE);
        // let square = rectangle::square(x as f64 * CELL_SIZE, (y as f64) * CELL_SIZE, CELL_SIZE);

        let transform = transform
            .trans((x as f64 * CELL_SIZE) + 0.0, (y as f64 * CELL_SIZE) + 0.0)
            .scale(0.9, 0.9);
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
        let sound_queue = self.game.get_sound_queue();
        while sound_queue.len() > 0 {
            if let Some(sound) = sound_queue.pop() {
                music::play_sound(&sound, music::Repeat::Times(0), 0.25);
            };
        }
    }

    pub fn input(&mut self, args: &ButtonArgs) {
        let state = match args.state {
            ButtonState::Press => true,
            ButtonState::Release => false,
        };
        match args.button {
            Button::Keyboard(key) => match key {
                // J => {
                //     self.input.ccw = state;
                // }
                // K => {
                //     self.input.cw = state;
                // }
                // Space => {
                //     self.input.hold = state;
                // }
                // D => {
                //     self.input.left = state;
                // }
                // A => {
                //     self.input.right = state;
                // }
                // W => {
                //     self.input.hard_drop = state;
                // }
                // S => {
                //     self.input.soft_drop = state;
                // }
                // R => {
                //     self.game = Game::new();
                // }
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
