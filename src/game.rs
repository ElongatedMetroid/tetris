use std::{thread, time::Duration, process};

use nalgebra::Vector2;
use rand::prelude::SliceRandom;

use crate::{
    playfield::Playfield,
    tetromino::{Tetromino, TetrominoKind},
};

pub struct Game {
    // (Temporarily public for testing purposes  )
    pub playfield: Playfield,
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

                if self.playfield.apply_falling(&mut piece) {
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
