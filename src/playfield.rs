use core::fmt;

use crate::{tetromino::Tetromino, cell::Cell};

pub struct Playfield {
    // 10 cells wide by 24 cells tall 
    cells: [[Cell; 10]; 24]
}

impl fmt::Display for Playfield {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.cells {
            write!(f, "|").unwrap();
            for cell in row {
                write!(f, "{}", cell).unwrap();
            }
            write!(f, "|\n").unwrap();
        }

        Ok(())
    }
}

impl Playfield {
    pub fn new() -> Self {
        Self {
            cells: [[' '; 10]; 24]
        }
    }
    
    pub fn spawn_tetrimino(&mut self, t: Tetromino) {
        
    }
}