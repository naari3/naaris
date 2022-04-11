extern crate piston_window;

mod app;
mod settings;
mod sound;

use std::fs::read_to_string;

use app::{App, CELL_SIZE};

use piston_window::{ButtonEvent, EventLoop, PistonWindow, RenderEvent, WindowSettings};
use settings::Settings;
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

    let settings_str = read_to_string("./settings.toml").unwrap();
    let settings: Settings = toml::from_str(&settings_str).unwrap();

    let game = Game::from_settings(
        settings.game.gravity,
        settings.game.are,
        settings.game.line_are,
        settings.game.das,
        settings.game.lock_delay,
        settings.game.line_clear_delay,
    );
    let mut app = App::new(game, settings);
    music::start::<Music, Sound, _>(16, || {
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
