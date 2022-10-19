use std::{process, thread, time::Duration};

use device_query::{DeviceState, DeviceQuery, Keycode};
use nalgebra::Vector2;
use rand::prelude::SliceRandom;

use crate::{
    playfield::{Direction, Playfield},
    tetromino::{Tetromino, TetrominoKind},
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
    pub fn run(&mut self) {
        let pieces = Tetromino::all(Vector2::new(5, 10));
        let mut rng = rand::thread_rng();
        let device_state = DeviceState::new();

        loop {
            let mut piece: Tetromino = pieces.choose(&mut rng).unwrap().clone();

            self.playfield.spawn_tetromino(&mut piece);

            loop {
                print!("\x1B[2J\x1B[1;1H");
                self.print_playfield();

                thread::sleep(Duration::from_millis(500));

                let direction = match device_state.get_keys().last() {
                    Some(&Keycode::Left) => Some(Direction::Left),
                    Some(&Keycode::Right) => Some(Direction::Right),
                    _ => None,
                };

                if let Some(direction) = direction {
                    self.playfield.move_tetromino(direction, &mut piece);
                }

                if self.playfield.move_tetromino(Direction::Down, &mut piece) {
                    break;
                }
            }
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
