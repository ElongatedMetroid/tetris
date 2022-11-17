use core::fmt;
use std::{cell::RefCell, rc::Rc};

use crate::{
    cell::Cell,
    grid::Grid,
    tetromino::Tetromino,
};

const PLAYFIELD_WIDTH: usize = 10;
const PLAYFIELD_HEIGHT: usize = 24;

pub struct Playfield {
    /// The grid will hold refrences to Cell's
    grid: Grid<Option<Rc<RefCell<Cell>>>>,
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
            self.grid.grid[tetromino_cell.borrow().position.y]
                [tetromino_cell.borrow().position.x] = Some(Rc::clone(&tetromino_cell));
        }
    }

    /// Update the positions of Cell's on the grid, match there position field to the corresponding position on the grid (the positions will
    /// only be changed if the can_move field is true)
    pub fn update_positions(&mut self) {
        for real_y in (0..PLAYFIELD_HEIGHT).rev() {
            for real_x in 0..PLAYFIELD_WIDTH {
                let cell = match &self.grid.grid[real_y][real_x] {
                    Some(cell) => cell,
                    None => continue,
                };

                if cell.borrow().can_move {
                    let cell_y = cell.borrow().position.y;
                    let cell_x = cell.borrow().position.x;

                    if (real_y, real_x) != (cell_y, cell_x) {
                        let cell = self.grid.grid[real_y][real_x].take();

                        self.grid.grid[cell_y][cell_x] = cell;
                    }
                }
            }
        }
    }
    // A tetris will
}
