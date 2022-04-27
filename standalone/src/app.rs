use fps_counter::FPSCounter;
use graphics::{
    clear,
    color::{BLACK, WHITE},
    Text,
};
use piston_window::{
    Button, ButtonArgs, ButtonState, Context, G2d, GfxDevice, Glyphs, RenderArgs, Transformed,
};
use tetris::{FallingPiece, Game, GameState, Input};

use crate::{renderers::Renderer, settings::Settings};

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

    pub fn render(&mut self, args: &RenderArgs, c: Context, g2d: &mut G2d, d: &mut GfxDevice) {
        clear(BLACK, g2d);

        self.game.render(args, c, g2d, d, &mut self.glyphs);

        let fps = self.fps.tick();
        let fps_text = format!("{fps} fps");

        Text::new_color(WHITE, 8)
            .draw(
                &fps_text,
                &mut self.glyphs,
                &c.draw_state,
                c.transform.trans(224.0, 11.0),
                g2d,
            )
            .unwrap();
        self.glyphs.factory.encoder.flush(d);
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
