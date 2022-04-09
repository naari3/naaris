extern crate piston_window;

use graphics::color::{BLACK, BLUE, CYAN, GRAY, GREEN, PURPLE, RED, WHITE, YELLOW};
use opengl_graphics::GlGraphics;
use piston_window::*;
use tetris::{Cell, Game};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    game: Game,     // Game
}
const CELL_SIZE: f64 = 16.0;

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let board = self.game.get_board();

        let offset_y = 20;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);
            for (y, cells_x) in board.cells.iter().enumerate() {
                if y < offset_y {
                    continue;
                };
                for (x, cell) in cells_x.iter().enumerate() {
                    match cell {
                        Some(cell) => {
                            App::render_cell(c, gl, x, y - offset_y, cell);
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
                        gl,
                        (rel_x + pos.0 as i16) as _,
                        ((-rel_y - offset_y as i16) + pos.1 as i16) as _,
                        &cell,
                    );
                }
            };
        });
    }

    fn render_cell(c: Context, gl: &mut GlGraphics, x: usize, y: usize, cell: &Cell) {
        use tetris::Cell::*;

        const ORANGE: [f32; 4] = [1.0, 0.75, 0.0, 1.0];
        let square = rectangle::square(0.0, 0.0, CELL_SIZE);

        let transform = c
            .transform
            .trans(x as f64 * CELL_SIZE, y as f64 * CELL_SIZE);
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
        rectangle(color, square, transform, gl);
    }

    fn update(&mut self, _: &UpdateArgs) {
        self.game.update()
    }
}

fn main() {
    let opengl = OpenGL::V4_5;
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [CELL_SIZE * 10.0, CELL_SIZE * 20.0])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();
    window.set_max_fps(60);

    let mut app = App {
        gl: GlGraphics::new(opengl),
        game: Game::new(),
    };
    while let Some(event) = window.next() {
        if let Some(args) = event.render_args() {
            app.render(&args);
        }
        if let Some(args) = event.update_args() {
            app.update(&args);
        }

        if let Some(args) = event.button_args() {
            println!("{:?}", args)
        }
    }
}
