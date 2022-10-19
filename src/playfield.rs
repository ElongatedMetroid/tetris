use std::fmt;

use nalgebra::Vector2;

use crate::{cell::Cell, tetromino::Tetromino};

const WIDTH: usize = 10;
const HEIGHT: usize = 24;

pub struct Playfield {
    // 10 cells wide by 24 cells tall
    cells: Vec<Vec<Cell>>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Down,
    Left,
    Right,
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

        Self { cells }
    }

    /// Clone each Cell of the tetromino onto the playfield
    pub fn spawn_tetromino(&mut self, t: &mut Tetromino) {
        for cell in &mut t.cell_data.cells {
            // Convert relative coords to playfield coords
            let y = cell.position.y as usize;
            let x = cell.position.x as usize;

            self.cells[y][x] = cell.clone();
        }
    }

    /// Applys `physics` to the given positions, returns true if you cannot fall (move down) anymore
    pub fn move_tetromino(&mut self, direction: Direction, t: &mut Tetromino) -> bool {
        // Foreach of the positions ...
        for Cell { position, .. } in &*t.cell_data.cells {
            let y = position.y as usize;
            let x = position.x as usize;

            // If the cell below is not empty, and (y+1, x) is not part of the tetromino/positions or y is greater than (HEIGHT - 1), do not
            // try to fall
            if if direction == Direction::Down { y } else { x }
                == (if direction == Direction::Down {
                    HEIGHT - 1
                } else if direction == Direction::Right {
                    WIDTH - 1
                } else {
                    0
                })
                || self.cells[if direction == Direction::Down {
                    y + 1
                } else {
                    y
                }][if direction == Direction::Right {
                    x + 1
                } else if direction == Direction::Left {
                    x - 1
                } else {
                    x
                }]
                .character
                    != ' '
                    && !t.cell_data.cells.contains(&Cell::new(
                        '■',
                        Vector2::new(
                            if direction == Direction::Right {
                                x + 1
                            } else if direction == Direction::Left {
                                x - 1
                            } else {
                                x
                            } as isize,
                            if direction == Direction::Down {
                                y + 1
                            } else {
                                y
                            } as isize,
                        ),
                    ))
            {
                return true;
            }
        }

        for Cell { position, .. } in t.cell_data.cells.iter_mut().rev() {
            // Move each piece one cell down since they all passed the check
            self.cells[position.y as usize][position.x as usize]
                .position
                .y += if direction == Direction::Down { 1 } else { 0 };
            self.cells[position.y as usize][position.x as usize]
                .position
                .x += if direction == Direction::Right {
                1
            } else if direction == Direction::Left {
                -1
            } else {
                0
            };
            position.y += if direction == Direction::Down { 1 } else { 0 };
            position.x += if direction == Direction::Right {
                1
            } else if direction == Direction::Left {
                -1
            } else {
                0
            };
        }

        self.update_positions(direction);

        false
    }

    pub fn update_positions(&mut self, direction: Direction) {
        let mut update_position = |real_x: usize, real_y: usize| {
            let cell_y = self.cells[real_y][real_x].position.y as usize;
            let cell_x = self.cells[real_y][real_x].position.x as usize;

            if (real_y, real_x) != (cell_y, cell_x) {
                let cell = self.cells[real_y][real_x].clone();

                self.cells[real_y][real_x] =
                    Cell::new(' ', Vector2::new(real_x as isize, real_y as isize));

                self.cells[cell_y][cell_x] = cell;
            }
        };

        for real_y in (0..HEIGHT).rev() {
            if direction == Direction::Left { 
                for real_x in 0..WIDTH {
                    update_position(real_x, real_y);
                }
            } else {
                for real_x in (0..WIDTH).rev() {
                    update_position(real_x, real_y);
                }
            }
        }
    }
}
