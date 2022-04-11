use std::fs::read_to_string;

use graphics::{
    clear,
    color::{BLACK, BLUE, CYAN, GRAY, GREEN, PURPLE, RED, WHITE, YELLOW},
    rectangle,
};
use piston_window::{
    Button, ButtonArgs, ButtonState, Context, G2d, Graphics, RenderArgs, Transformed,
};
use serde_derive::Deserialize;
use tetris::{Cell, Game, Input};

const ORANGE: [f32; 4] = [1.0, 0.75, 0.0, 1.0];

#[derive(Deserialize)]
pub struct Settings {
    key_config: KeyConfig,
}

#[derive(Deserialize)]
pub struct KeyConfig {
    left: usize,
    right: usize,
    soft_drop: usize,
    hard_drop: usize,
    cw: usize,
    ccw: usize,
    hold: usize,
    restart: usize,
}

pub struct App {
    game: Game,   // Game
    input: Input, // Input
    settings: Settings,
}
pub const CELL_SIZE: f64 = 16.0;

impl App {
    pub fn new(game: Game) -> Self {
        let settings_str = read_to_string("./settings.toml").unwrap();
        let settings = toml::from_str(&settings_str).unwrap();
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
        clear(WHITE, g2d);
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

    fn render_board<G: Graphics>(
        &mut self,
        c: Context,
        g: &mut G,
        offset_x: usize,
        offset_y: usize,
    ) {
        let square = [0.0, 0.0, 10.0 * CELL_SIZE, 20.0 * CELL_SIZE];
        let transform = c.transform.trans(offset_x as f64, offset_y as f64);
        rectangle(GRAY, square, transform, g);
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
        rectangle(color, square, transform, g);
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
                    if self.settings.key_config.left == key.code() as _ {
                        self.input.left = state;
                    }
                    if self.settings.key_config.right == key.code() as _ {
                        self.input.right = state;
                    }
                    if self.settings.key_config.hard_drop == key.code() as _ {
                        self.input.hard_drop = state;
                    }
                    if self.settings.key_config.soft_drop == key.code() as _ {
                        self.input.soft_drop = state;
                    }
                    if self.settings.key_config.cw == key.code() as _ {
                        self.input.cw = state;
                    }
                    if self.settings.key_config.ccw == key.code() as _ {
                        self.input.ccw = state;
                    }
                    if self.settings.key_config.hold == key.code() as _ {
                        self.input.hold = state;
                    }
                    if self.settings.key_config.restart == key.code() as _ && !state {
                        self.game = Game::new();
                    }

                    println!("key.code(): {}", key.code());
                    // self.settings.key_config
                }
            },
            _ => {}
        }
    }
}
