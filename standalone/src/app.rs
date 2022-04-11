use graphics::{clear, line_from_to, rectangle, types::Color};
use piston_window::{
    Button, ButtonArgs, ButtonState, Context, G2d, Graphics, RenderArgs, Transformed,
};
use tetris::{Cell, Game, Input};

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
    game: Game,   // Game
    input: Input, // Input
    settings: Settings,
}
pub const CELL_SIZE: f64 = 16.0;

impl App {
    pub fn new(game: Game, settings: Settings) -> Self {
        Self {
            game,
            input: Default::default(),
            settings,
        }
    }

    pub fn render(&mut self, _args: &RenderArgs, c: Context, g2d: &mut G2d) {
        // let mut board_buffer = RenderBuffer::new((CELL_SIZE * 10.0) as _, (CELL_SIZE * 20.0) as _);
        // let texture = board_buffer
        //     .to_g2d_texture(&mut self.texture_context, &TextureSettings::new())
        //     .unwrap();

        // clear(TRANSPARENT, &mut board_buffer);
        clear(BLACK.to_color(), g2d);
        self.render_hold(
            c,
            g2d,
            (CELL_SIZE * 1.0) as usize,
            (CELL_SIZE * 2.0) as usize,
        );
        self.render_next(
            c,
            g2d,
            (CELL_SIZE * 6.0) as usize,
            (CELL_SIZE * 2.0) as usize,
        );
        self.render_board_outline(
            c,
            g2d,
            2.0,
            (CELL_SIZE * 1.0) as usize,
            (CELL_SIZE * 4.0) as usize,
        );
        self.render_board(
            c,
            g2d,
            (CELL_SIZE * 1.0) as usize,
            (CELL_SIZE * 4.0) as usize,
        );

        // let texture =
        //     opengl_graphics::Texture::from_image(&*board_buffer, &TextureSettings::new());
        // image(&texture, c.transform.trans(10.0, 0.0), g2d);
    }

    fn render_board_outline<G: Graphics>(
        &mut self,
        c: Context,
        g: &mut G,
        radius: f64,
        offset_x: usize,
        offset_y: usize,
    ) {
        let color = GRAY.to_color();
        let left = [
            [0.0 + offset_x as f64, 0.0 + offset_y as f64 + radius],
            [
                0.0 + offset_x as f64,
                20.0 * CELL_SIZE + offset_y as f64 - radius,
            ],
        ];
        let right = [
            [
                10.0 * CELL_SIZE + offset_x as f64,
                0.0 + offset_y as f64 + radius,
            ],
            [
                10.0 * CELL_SIZE + offset_x as f64,
                20.0 * CELL_SIZE + offset_y as f64 - radius,
            ],
        ];
        let top = [
            [0.0 + offset_x as f64 - radius, 0.0 + offset_y as f64],
            [
                10.0 * CELL_SIZE + offset_x as f64 + radius,
                0.0 + offset_y as f64,
            ],
        ];
        let bottom = [
            [
                0.0 + offset_x as f64 - radius,
                20.0 * CELL_SIZE + offset_y as f64,
            ],
            [
                10.0 * CELL_SIZE + offset_x as f64 + radius,
                20.0 * CELL_SIZE + offset_y as f64,
            ],
        ];
        // let left_top = [0.0 + offset_x as f64, 0.0 + offset_y as f64];
        // let right_top = [10.0 * CELL_SIZE + offset_x as f64, 0.0 + offset_y as f64];
        // let left_bottom = [0.0 + offset_x as f64, 20.0 * CELL_SIZE + offset_y as f64];
        // let right_bottom = [
        //     10.0 * CELL_SIZE + offset_x as f64,
        //     20.0 * CELL_SIZE + offset_y as f64,
        // ];

        line_from_to(color, radius, left[0], left[1], c.transform, g);
        line_from_to(color, radius, right[0], right[1], c.transform, g);
        line_from_to(color, radius, top[0], top[1], c.transform, g);
        line_from_to(color, radius, bottom[0], bottom[1], c.transform, g);
    }

    fn render_board<G: Graphics>(
        &mut self,
        c: Context,
        g: &mut G,
        offset_x: usize,
        offset_y: usize,
    ) {
        let square = [0.0, 0.0, 10.0 * CELL_SIZE, 20.0 * CELL_SIZE];
        let transform = c.transform.trans(offset_x as f64, offset_y as f64);
        rectangle(BLACK.to_color(), square, transform, g);
        let cell_offset_y = 20;
        for (y, cells_x) in self.game.get_board().cells.iter().enumerate() {
            if y < cell_offset_y {
                continue;
            };
            for (x, cell) in cells_x.iter().enumerate() {
                match cell {
                    Some(cell) => {
                        App::render_cell(
                            c,
                            g,
                            x as _,
                            (y - cell_offset_y) as _,
                            offset_x,
                            offset_y,
                            cell,
                        );
                    }
                    None => {}
                }
            }
        }

        if let Some(current_piece) = self.game.current_piece {
            let pos = current_piece.piece_position;
            let cell = current_piece.piece_state.get_kind().into();

            for (rel_x, rel_y) in current_piece.piece_state.get_cells().iter() {
                App::render_cell(
                    c,
                    g,
                    (rel_x + pos.0 as i16) as _,
                    ((-rel_y - cell_offset_y as i16) + pos.1 as i16) as _,
                    offset_x,
                    offset_y,
                    &cell,
                );
            }
        };
    }

    fn render_hold<G: Graphics>(
        &mut self,
        c: Context,
        g: &mut G,
        offset_x: usize,
        offset_y: usize,
    ) {
        if let Some(hold) = self.game.get_hold() {
            let cell = hold.into();
            for (rel_x, rel_y) in hold.get_cells().into_iter() {
                App::render_cell(c, g, rel_x as _, -rel_y as _, offset_x, offset_y, &cell);
            }
        };
    }

    fn render_next<G: Graphics>(
        &mut self,
        c: Context,
        g: &mut G,
        offset_x: usize,
        offset_y: usize,
    ) {
        let next = self.game.get_next();
        let cell = next.into();
        for (rel_x, rel_y) in next.get_cells().into_iter() {
            App::render_cell(c, g, rel_x as _, -rel_y as _, offset_x, offset_y, &cell);
        }

        let next_next = self.game.get_next_next();
        let cell = next_next.into();
        for (rel_x, rel_y) in next_next.get_cells().into_iter() {
            App::render_cell(
                c,
                g,
                (rel_x + 4) as _,
                -rel_y as _,
                offset_x,
                offset_y,
                &cell,
            );
        }

        let next_next_next = self.game.get_next_next_next();
        let cell = next_next_next.into();
        for (rel_x, rel_y) in next_next_next.get_cells().into_iter() {
            App::render_cell(
                c,
                g,
                (rel_x + 8) as _,
                -rel_y as _,
                offset_x,
                offset_y,
                &cell,
            );
        }
    }

    fn render_cell<G: Graphics>(
        c: Context,
        g: &mut G,
        x: i32,
        y: i32,
        offset_x: usize,
        offset_y: usize,
        cell: &Cell,
    ) {
        use tetris::Cell::*;

        let square = rectangle::square(0.0, 0.0, CELL_SIZE);
        // let square = rectangle::square(x as f64 * CELL_SIZE, (y as f64) * CELL_SIZE, CELL_SIZE);

        let transform = c.transform.trans(
            (x as f64 * CELL_SIZE) + offset_x as f64,
            (y as f64 * CELL_SIZE) + offset_y as f64,
        );
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