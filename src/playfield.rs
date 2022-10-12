use std::fmt;

use nalgebra::Vector2;

use crate::{tetromino::Tetromino, cell::Cell};

const WIDTH: usize = 10;
const HEIGHT: usize = 24;

pub struct Playfield {
    // 10 cells wide by 24 cells tall 
    cells: Vec<Vec<Cell>>,
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

        Ok(())
    }
}

impl Playfield {
    pub fn new() -> Self {
        let mut cells = Vec::new();

        for _ in 0..HEIGHT {
            let mut row = Vec::new();
            for _ in 0..WIDTH {
                row.push(Cell::new(' ', Vector2::new(0, 0)));
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
        // First apply physics to all cells except the tetromino
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let cell = &mut self.cells[y][x];
                // No physics need to be applied to empty cells
                if cell.character != ' ' {
                    // We are not applying physics to tetrominos in this loop 
                    if !cell.tetromino_part { 
                        
                    }
                }
                
            }
        }

        let mut attached_cells = Vec::new();
        // Get the tetromino
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let cell = &mut self.cells[y][x];
                // Is this loop we are applying physics to the pieces themselves
                if cell.tetromino_part { 
                    attached_cells.push((y, x));
                }
            }
        }

        let mut tetromino_cells = Vec::new();

        // Save all the cells to a vector
        for (y, x) in &attached_cells {
            tetromino_cells.push((self.cells[*y][*x].clone(), (*y, *x)));
        }
        // Clear all the cells
        for (y, x) in &attached_cells {
            self.cells[*y][*x] = Cell::new(' ', Vector2::new(0, 0));
        }
        // Set the cells
        for (cell, (y, x)) in tetromino_cells {
            self.cells[y+1][x] = cell;
        }
        

        true
    }
}