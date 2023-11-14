pub mod game;
pub mod vector2int;
pub mod sdl_context;
pub mod vector2;
pub mod grid_tile;

use game::Game;
use std::time::{Instant, Duration};

const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGHT: u32 = 600;

const FPS: u32 = 15;
const FRAME_DURATION: u64 = 1000 / FPS as u64;

fn main() -> Result<(), String> {
    let mut game: Game = Game::new(SCREEN_WIDTH, SCREEN_HEIGHT, 20)?;

    //let mut last_frame_time = Instant::now();

    while game.running {
        let frame_start = Instant::now();

        game.events();
        game.input();
        game.logic();
        game.render();

        let frame_duration = frame_start.elapsed();

        if frame_duration < Duration::from_millis(FRAME_DURATION) {
            let sleep_time = Duration::from_millis(FRAME_DURATION) - frame_duration;
            std::thread::sleep(sleep_time);
        }

        //last_frame_time = frame_start;
    }
        
    Ok(())
}