use nalgebra::Vector2;

use crate::{
    playfield::Playfield,
    tetromino::{Axis, Tetromino, TetrominoKind},
};

pub struct Game;

impl Game {
    /// Run the game
    pub fn run(self) {
        let mut playfield = Playfield::new();

        let tetromino = Tetromino::new(true, TetrominoKind::T, Vector2::new(5, 10));

        playfield.spawn(&tetromino);

        println!("{}", playfield);

        tetromino.move_checked(&mut playfield, Axis::Vertical, 3);

        println!("{}", playfield);
    }
}
