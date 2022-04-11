extern crate piston_window;

use graphics::{
    clear,
    color::{BLACK, BLUE, CYAN, GRAY, GREEN, PURPLE, RED, WHITE, YELLOW},
    rectangle,
};
use piston_window::{
    Button, ButtonArgs, ButtonEvent, ButtonState, Context, EventLoop, G2d, Graphics, PistonWindow,
    RenderArgs, RenderEvent, Transformed, /*UpdateEvent,*/ WindowSettings,
};
use tetris::{Cell, Game, Input};

const ORANGE: [f32; 4] = [1.0, 0.75, 0.0, 1.0];

pub struct App {
    game: Game,   // Game
    input: Input, // Input
}
const CELL_SIZE: f64 = 16.0;

impl App {
    fn render(&mut self, _args: &RenderArgs, c: Context, g2d: &mut G2d) {
        // let mut board_buffer = RenderBuffer::new((CELL_SIZE * 10.0) as _, (CELL_SIZE * 20.0) as _);
        // let texture = board_buffer
        //     .to_g2d_texture(&mut self.texture_context, &TextureSettings::new())
        //     .unwrap();

        // clear(TRANSPARENT, &mut board_buffer);
        clear(WHITE, g2d);
        self.render_hold(
            c,
            g2d,
            (CELL_SIZE * 2.0) as usize,
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

    fn update(&mut self) {
        self.game.set_input(self.input);
        self.game.update();
    }

    fn input(&mut self, args: &ButtonArgs) {
        use piston_window::Key::*;
        let state = match args.state {
            ButtonState::Press => true,
            ButtonState::Release => false,
        };
        match args.button {
            Button::Keyboard(key) => match key {
                J => {
                    self.input.ccw = state;
                }
                K => {
                    self.input.cw = state;
                }
                Space => {
                    self.input.hold = state;
                }
                D => {
                    self.input.left = state;
                }
                A => {
                    self.input.right = state;
                }
                W => {
                    self.input.hard_drop = state;
                }
                S => {
                    self.input.soft_drop = state;
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Hello Piston!",
        [CELL_SIZE * (10.0 + 6.0), CELL_SIZE * (20.0 + 4.0)],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();
    window.set_max_fps(60);

    let mut app = App {
        game: Game::new(),
        input: Default::default(),
    };
    while let Some(event) = window.next() {
        if let Some(args) = event.render_args() {
            window.draw_2d(&event, |c, g2d, _| {
                app.update();
                app.render(&args, c, g2d);
            });
        }
        // if let Some(_args) = event.update_args() {}

        if let Some(args) = event.button_args() {
            app.input(&args);
        }
    }
}
