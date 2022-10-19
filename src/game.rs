use std::{process, thread, time::Duration};

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

        loop {
            let mut piece: Tetromino = pieces.choose(&mut rng).unwrap().clone();

            self.playfield.spawn_tetromino(&mut piece);

            loop {
                print!("\x1B[2J\x1B[1;1H");
                self.print_playfield();

                thread::sleep(Duration::from_millis(500));

                if self.playfield.move_tetromino(Direction::Left, &mut piece) {
                    process::exit(1);
                }
                self.playfield.update_positions();

                if self.playfield.move_tetromino(Direction::Down, &mut piece) {
                    break;
                }
                self.playfield.update_positions();
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
