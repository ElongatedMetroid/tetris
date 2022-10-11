use core::fmt;
use std::borrow::Borrow;

use nalgebra::Vector2;

use crate::{tetromino::Tetromino, cell::Cell};

pub struct Playfield {
    // 10 cells wide by 24 cells tall 
    cells: Vec<Vec<Cell>>
}

impl fmt::Display for Playfield {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
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
        let mut cells = Vec::new();

        for _ in 0..24 {
            let mut row = Vec::new();
            for _ in 0..10 {
                row.push(Cell::new(' ', Vector2::new(0, 0)));
            }
            cells.push(row);
        }

        Self {
            cells
        }
    }
    
    pub fn spawn_tetromino(&mut self, t: Tetromino) {
        let y = t.cell_data.main_cell.position.y as usize;
        let x = t.cell_data.main_cell.position.x as usize;

        self.cells[y][x] = t.cell_data.main_cell;

        for cell in t.cell_data.attached_cells {
            let y = (y as isize + cell.position.y) as usize;
            let x = (x as isize + cell.position.x) as usize;

            self.cells[y][x] = cell;
        }
    }
}