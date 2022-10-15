use std::{thread, time::Duration};

use nalgebra::Vector2;
use tetris::{tetromino::{Tetromino, TetrominoKind}, game::Game};

fn main() {
    let mut game = Game::new();

    let i = Tetromino::new(TetrominoKind::I, Vector2::new(5, 10), 0);

    let tetromino = game.playfield.spawn_tetromino(i);

    game.print_playfield();

    thread::sleep(Duration::from_secs(1));
    
    game.playfield.apply_falling(&tetromino);

    game.playfield.update_positions();

    game.print_playfield();
}
