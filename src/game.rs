use device_query::{DeviceQuery, DeviceState, Keycode};
use nalgebra::Vector2;
use rand::seq::SliceRandom;

use crate::{
    playfield::Playfield,
    tetromino::{Axis, Tetromino, TetrominoKind},
};

pub struct Game;

impl Game {
    /// Run the game
    pub fn run(self) {
        // Thread controling player movement
        // Thread that moves the piece down and when its at the bottom generate a new piece
        // Thread that prints the board
        // Thread that checks for line clears
        let device_state = DeviceState::new();
        let mut playfield = Playfield::new();
        let tetrominos = Tetromino::all(true, Vector2::new(5, 10));
        let mut tetromino = tetrominos.choose(&mut rand::thread_rng()).unwrap();

        playfield.spawn(&tetromino);

        // Last recorded time
        let mut last_time = std::time::Instant::now();
        let mut falling_timer = 0;
        let mut print_timer = 0;
        let mut input_timer = 0;

        let print_every = 5000000;
        let fall_every = 1000000000;
        let input_every = 70000000;
        loop {
            // Current time
            let time = std::time::Instant::now();
            // Time elapsed since last loop
            let time_elapsed = time.duration_since(last_time).as_nanos();
            last_time = time;

            print_timer += time_elapsed;
            while print_timer >= print_every {
                print!("\x1B[2J\x1B[1;1H");
                println!("{}", playfield);

                print_timer -= print_every;
            }
            // Update the timer with the amount of time that has passed
            falling_timer += time_elapsed;
            // Every `fall_every` make the piece fall one cell
            while falling_timer >= fall_every {
                // If the piece is done falling
                if !tetromino.down(&mut playfield) {
                    tetromino.kill(&mut playfield);
                    // Generate a new piece
                    tetromino = tetrominos.choose(&mut rand::thread_rng()).unwrap();
                    playfield.spawn(&tetromino);
                }

                falling_timer -= fall_every;
            }

            input_timer += time_elapsed;
            while input_timer >= input_every {
                let keys = device_state.get_keys();
                if keys.contains(&Keycode::Up) {
                    tetromino.up(&mut playfield);
                }
                if keys.contains(&Keycode::Down) {
                    tetromino.down(&mut playfield);
                }
                if keys.contains(&Keycode::Left) {
                    tetromino.left(&mut playfield);
                }
                if keys.contains(&Keycode::Right) {
                    tetromino.right(&mut playfield);
                }

                input_timer -= input_every;
            }
        }
    }
}
