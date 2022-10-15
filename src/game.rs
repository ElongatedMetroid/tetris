use std::{thread, time::Duration};

use nalgebra::Vector2;

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
        let i = Tetromino::new(TetrominoKind::I, Vector2::new(5, 10), 0);

        let mut tetromino = self.playfield.spawn_tetromino(i);

        loop {
            print!("\x1B[2J\x1B[1;1H");
            self.print_playfield();

            thread::sleep(Duration::from_millis(500));

            self.playfield.apply_falling(&mut tetromino);
            self.playfield.update_positions();
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
