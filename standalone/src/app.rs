use fps_counter::FPSCounter;
use graphics::{
    clear,
    color::{BLACK, WHITE},
    rectangle, Text,
};
use piston_window::{
    Button, ButtonArgs, ButtonState, Context, G2d, GfxDevice, Glyphs, RenderArgs, Transformed,
};
use tetris::{GameState, Input};

use crate::{renderers::Renderer, settings::KeyConfig, sound::StandaloneSound, CELL_SIZE};

pub struct App<G: GameState + Renderer, R: FnMut() -> G> {
    fps: FPSCounter,
    glyphs: Glyphs,
    game: G,      // Game
    input: Input, // Input
    key_config: KeyConfig,
    reset: R,
    pause: bool,
}

impl<G: GameState + Renderer, R: FnMut() -> G> App<G, R> {
    pub fn new(game: G, key_config: KeyConfig, glyphs: Glyphs, reset: R) -> Self {
        Self {
            fps: FPSCounter::default(),
            glyphs,
            game,
            input: Default::default(),
            key_config,
            reset,
            pause: false,
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
        if self.pause {
            rectangle(
                BLACK,
                [0.0, 0.0, CELL_SIZE * 9.0, CELL_SIZE * 1.0],
                c.transform.trans(CELL_SIZE * 1.5, CELL_SIZE * 12.5),
                g2d,
            );
            Text::new_color(WHITE, 8)
                .draw(
                    &format!("pause"),
                    &mut self.glyphs,
                    &c.draw_state,
                    c.transform.trans(CELL_SIZE * 2.0, CELL_SIZE * 13.0),
                    g2d,
                )
                .unwrap();
        }
        self.glyphs.factory.encoder.flush(d);
    }

    pub fn update(&mut self) {
        self.game.set_input(self.input);
        if !self.pause {
            self.game.update();
        }
        {
            let sound_queue = self.game.get_sound_queue();
            while sound_queue.len() > 0 {
                if let Some(sound) = sound_queue.pop() {
                    music::play_sound::<StandaloneSound>(
                        &sound.into(),
                        music::Repeat::Times(0),
                        0.25,
                    );
                };
            }
        }
        {
            let event_queue = self.game.get_event_queue();
            while event_queue.len() > 0 {
                if let Some(event) = event_queue.pop() {
                    match event {
                        // tetris::TetrisEvent::PieceLocked() => {}
                        _ => {}
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
                    if self.key_config.left == key.code() as _ {
                        self.input.left = state;
                    }
                    if self.key_config.right == key.code() as _ {
                        self.input.right = state;
                    }
                    if self.key_config.hard_drop == key.code() as _ {
                        self.input.hard_drop = state;
                    }
                    if self.key_config.soft_drop == key.code() as _ {
                        self.input.soft_drop = state;
                    }
                    if self.key_config.cw == key.code() as _ {
                        self.input.cw = state;
                    }
                    if self.key_config.ccw == key.code() as _ {
                        self.input.ccw = state;
                    }
                    if self.key_config.hold == key.code() as _ {
                        self.input.hold = state;
                    }
                    if self.key_config.restart == key.code() as _ && !state {
                        self.game = (self.reset)();
                    }
                    if self.key_config.pause == key.code() as _ && !state {
                        self.pause = !self.pause;
                    }

                    println!("key.code(): {}", key.code());
                    // self.key_config_config
                }
            },
            _ => {}
        }
    }
}
