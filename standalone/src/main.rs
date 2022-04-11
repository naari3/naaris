extern crate piston_window;

mod app;
mod sound;

use app::{App, CELL_SIZE};

use piston_window::{ButtonEvent, EventLoop, PistonWindow, RenderEvent, WindowSettings};
use tetris::{Game, Music, Sound};

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Hello Piston!",
        [CELL_SIZE * (10.0 + 6.0), CELL_SIZE * (20.0 + 4.0)],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();
    window.set_max_fps(60);

    let mut app = App::new(Game::new());
    music::start::<Music, Sound, _>(8, || {
        sound::init();
        music::set_volume(0.5);
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
    });
}
