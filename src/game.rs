use std::{process, thread, time::Duration, sync::{Arc, Mutex}};

use device_query::{DeviceState, DeviceQuery, Keycode};
use nalgebra::Vector2;
use rand::prelude::SliceRandom;

use crate::{
    playfield::{Direction, Playfield},
    tetromino::Tetromino,
};

pub struct Game {
    playfield: Playfield,
}

impl Game {
    pub fn new() -> Self {
        Self {
            playfield: Playfield::new(),
        }
    }
    pub fn run(self) {
        let pieces = Tetromino::all(Vector2::new(5, 10));
        let mut piece = Arc::new(Mutex::new(pieces[0].clone()));
        let game = Arc::new(Mutex::new(self));

        let piece_clone = Arc::clone(&piece);
        let game_clone = Arc::clone(&game);
        // Keyboard input thread
        thread::spawn(move || {
            let device_state = DeviceState::new();

            loop {
                let direction = match device_state.get_keys().first() {
                    Some(&Keycode::Left) => Some(Direction::Left),
                    Some(&Keycode::Right) => Some(Direction::Right),
                    Some(&Keycode::Down) => Some(Direction::Down),
                    _ => None,
                };

                if let Some(direction) = direction {
                    game_clone.lock().unwrap().playfield.move_tetromino(direction, &mut piece_clone.lock().unwrap());
                }

                thread::sleep(Duration::from_millis(100));
            }
        });
        let piece_clone = Arc::clone(&piece);
        let game_clone = Arc::clone(&game);
        // Falling tetromino thread
        thread::spawn(move || {
            let mut rng = rand::thread_rng();

            loop {
                *piece_clone.lock().unwrap() = pieces.choose(&mut rng).unwrap().clone();
                game_clone.lock().unwrap().playfield.spawn_tetromino(&mut piece.lock().unwrap());
    
                loop {
                    thread::sleep(Duration::from_millis(500));
    
                    if game_clone.lock().unwrap().playfield.move_tetromino(Direction::Down, &mut piece_clone.lock().unwrap()) {
                        // Tetromino is now grounded, request a new tetromino
                        break;
                    }
                }
            }
        });

        loop {
            print!("\x1B[2J\x1B[1;1H");
            game.lock().unwrap().print_playfield();

            thread::sleep(Duration::from_millis(100));
        }
    }
    pub fn print_playfield(&self) {
        if cfg!(debug_assertions) {
            print!("{:?}", self.playfield);
        } else {
            print!("{}", self.playfield);
        }
    }
}
