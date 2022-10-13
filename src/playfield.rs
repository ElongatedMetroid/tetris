use std::fmt;

use nalgebra::Vector2;

use crate::{tetromino::Tetromino, cell::Cell};

const WIDTH: usize = 10;
const HEIGHT: usize = 24;

pub struct Playfield {
    // 10 cells wide by 24 cells tall 
    pub cells: Vec<Vec<Cell>>,
}

impl fmt::Display for Playfield {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            write!(f, "|").unwrap();
            for cell in row {
                write!(f, "{}", cell).unwrap();
            }
            writeln!(f, "|").unwrap();
        }

        Ok(())
    }
}

impl fmt::Debug for Playfield {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.cells.iter().enumerate() {
            write!(f, "{:2} |", i).unwrap();
            for cell in row {
                write!(f, "{}", cell).unwrap();
            }
            writeln!(f, "|").unwrap();
        }

        writeln!(f, "    0123456789").unwrap();

        Ok(())
    }
}

impl Playfield {
    pub fn new() -> Self {
        let mut cells = Vec::new();

        for y in 0..HEIGHT {
            let mut row = Vec::new();
            for x in 0..WIDTH {
                row.push(Cell::new(' ', Vector2::new(x as isize, y as isize)));
            } 
            cells.push(row);
        }

        Self {
            cells,
        }
    }
    
    pub fn spawn_tetromino(&mut self, t: Tetromino) {
        let y = t.cell_data.main_cell.position.y as usize;
        let x = t.cell_data.main_cell.position.x as usize;

        self.cells[y][x] = t.cell_data.main_cell.clone();

        for cell in t.cell_data.attached_cells {
            // Convert relative coords to playfield coords
            let y = (y as isize + cell.position.y) as usize;
            let x = (x as isize + cell.position.x) as usize;

            let mut cell = cell.clone();

            cell.position.y = y as isize;
            cell.position.x = x as isize;

            self.cells[y][x] = cell;
        }
    }

    /// Applys `physics` to each cell, modifying the cells position
    /// This will return true if all physics were applied
    pub fn apply_falling(&mut self) -> bool {
        true
    }

    pub fn update_positions(&mut self) {
        for real_y in 0..HEIGHT {
            for real_x in 0..WIDTH {
                let cell_y = self.cells[real_y][real_x].position.y as usize;
                let cell_x = self.cells[real_y][real_x].position.x as usize;

                if (real_y, real_x) != (cell_y, cell_x) {
                    let cell = self.cells[real_y][real_x].clone();

                    self.cells[real_y][real_x] = Cell::new(' ', Vector2::new(real_x as isize, real_y as isize));

                    self.cells[cell_y][cell_x] = cell;
                }
            }
        }
    }
}