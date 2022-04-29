extern crate piston_window;

mod app;
mod renderers;
mod settings;
mod sound;

use std::fs::read_to_string;

use app::App;

pub const CELL_SIZE: f64 = 16.0;

use piston_window::{ButtonEvent, EventLoop, PistonWindow, RenderEvent, WindowSettings};
use renderers::Renderer;
use settings::{GameMode, Settings};
use sound::StandaloneSound;
use tetris::{Game, GameState, Music, Sound, TGM3Master};

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "naaris",
        [CELL_SIZE * (10.0 + 6.0), CELL_SIZE * (20.0 + 5.0)],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();
    window.set_max_fps(60);

    let settings_str = read_to_string("./settings.toml").unwrap();
    let settings: Settings = toml::from_str(&settings_str).unwrap();

    // Pixeloid Sans
    // This font family are licensed under the SIL Open Font License, Version 1.1.
    // https://ggbot.itch.io/pixeloid-font
    let font_path = "./assets/PixeloidSans.ttf";
    let glyphs = window.load_font(font_path).unwrap();

    match settings.mode {
        GameMode::Free => {
            let game = Game::from_settings(
                settings.game.gravity,
                settings.game.are,
                settings.game.line_are,
                settings.game.das,
                settings.game.lock_delay,
                settings.game.line_clear_delay,
            );
            main_loop(
                window,
                App::new(
                    game,
                    settings.key,
                    glyphs,
                    Box::new(move || {
                        Game::from_settings(
                            settings.game.gravity,
                            settings.game.are,
                            settings.game.line_are,
                            settings.game.das,
                            settings.game.lock_delay,
                            settings.game.line_clear_delay,
                        )
                    }),
                ),
            )
        }
        GameMode::TGM3Master => {
            let game = TGM3Master::new();
            main_loop(
                window,
                App::new(game, settings.key, glyphs, || TGM3Master::new()),
            )
        }
    };
}

fn main_loop<G: GameState + Renderer, R: FnMut() -> G>(
    mut window: PistonWindow,
    mut app: App<G, R>,
) {
    music::start::<Music, StandaloneSound, _>(256, || {
        sound::init();
        music::set_volume(0.5);
        while let Some(event) = window.next() {
            if let Some(args) = event.render_args() {
                window.draw_2d(&event, |c, g2d, d| {
                    app.update();
                    app.render(&args, c, g2d, d);
                });
            }
            // if let Some(_args) = event.update_args() {}

            if let Some(args) = event.button_args() {
                app.input(&args);
            }
        }
    });
}
