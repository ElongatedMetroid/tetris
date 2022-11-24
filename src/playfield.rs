use core::fmt;
use std::{cell::RefCell, rc::Rc};

use crate::{cell::Cell, grid::Grid, tetromino::Tetromino};

pub const PLAYFIELD_WIDTH: usize = 10;
pub const PLAYFIELD_HEIGHT: usize = 24;

pub struct Playfield {
    /// The grid will hold refrences to Cell's
    pub(crate) grid: Grid<Option<Rc<RefCell<Cell>>>>,
}

impl fmt::Display for Playfield {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid.grid {
            write!(f, "|").unwrap();
            for (i, cell) in row.iter().enumerate() {
                write!(
                    f,
                    "{}{}",
                    if i == 0 { "" } else { "|" },
                    match cell {
                        Some(cell) => cell.borrow().character,
                        None => ' ',
                    }
                )
                .unwrap();
            }
            writeln!(f, "|").unwrap();
        }

        Ok(())
    }
}

impl Playfield {
    /// Create a new Playfield instance
    pub fn new() -> Self {
        let mut playfield = Self {
            grid: Grid::<Option<Rc<RefCell<Cell>>>>::new(),
        };

        for _ in 0..PLAYFIELD_HEIGHT {
            let mut row = Vec::new();
            for _ in 0..PLAYFIELD_WIDTH {
                row.push(None);
            }
            playfield.grid.grid.push(row);
        }

        playfield
    }

    /// Spawn a tetromino
    pub fn spawn(&mut self, tetromino: &Tetromino) {
        for tetromino_cell in &tetromino.cells {
            self.grid.grid[tetromino_cell.borrow().position.y as usize]
                [tetromino_cell.borrow().position.x as usize] = Some(Rc::clone(tetromino_cell));
        }
    }

    // A tetris will
}
