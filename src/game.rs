use nalgebra::Vector2;

use crate::{
    playfield::{self, Playfield},
    tetromino::{self, Tetromino, TetrominoKind},
};

pub struct Game;

impl Game {
    /// Run the game
    pub fn run(self) {
        let mut playfield = Playfield::new();

        let mut tetromino = Tetromino::new(TetrominoKind::I, Vector2::new(5, 10));

        playfield.spawn(&tetromino);

        println!("{}", playfield);

        println!("{}", playfield);
    }
}
