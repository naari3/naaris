use std::{
    thread,
    time::{Duration, Instant},
};

use tetris::*;

const FRAME_RATE: f32 = 60.0;

fn main() {
    let frame_duration = Duration::from_secs_f32(1.0 / FRAME_RATE);
    let mut game = Game::new();
    loop {
        let frame_start = Instant::now();
        game.update();
        println!("{game}");
        let frame_end = Instant::now();

        if frame_end - frame_start < Duration::from_secs_f32(1.0 / FRAME_RATE) {
            thread::sleep(frame_duration - (frame_end - frame_start));
        }
    }
}
