use core::fmt;

use crate::{cell::Cell, grid::Grid};

const PLAYFIELD_WIDTH: usize = 10;
const PLAYFIELD_HEIGHT: usize = 24;

pub struct Playfield<'a> {
    /// The grid will hold refrences to Cell's
    grid: Grid<&'a Option<Cell>>,
}

impl fmt::Display for Playfield<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid.grid {
            write!(f, "|").unwrap();
            for (i, cell) in row.iter().enumerate() {
                write!(
                    f,
                    "{}{}",
                    if i == 0 { "" } else { "|" },
                    match cell {
                        Some(cell) => cell.character,
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

impl Playfield<'_> {
    /// Create a new Playfield instance
    pub fn new() -> Self {
        let mut playfield = Self {
            grid: Grid::<&Option<Cell>>::new(),
        };

        for _ in 0..PLAYFIELD_HEIGHT {
            let mut row = Vec::new();
            for _ in 0..PLAYFIELD_WIDTH {
                row.push(&None);
            }
            playfield.grid.grid.push(row);
        }

        playfield
    }
}
