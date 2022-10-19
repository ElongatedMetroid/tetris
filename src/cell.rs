use std::fmt;

use nalgebra::Vector2;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub(super) character: char,
    /// Only used for spawning in the tetromino
    pub(super) position: Vector2<isize>,
}

/// A collection of Cell's attached in a certain way
#[derive(Clone)]
pub struct CellBunch {
    /// Vector of Cell's attached to each other
    pub(super) cells: Vec<Cell>,
}

pub struct CellBunchBuilder {
    cells: Vec<Cell>,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.character)
    }
}

impl Cell {
    pub fn new(character: char, position: Vector2<isize>) -> Self {
        Self {
            character,
            position,
        }
    }
}

impl CellBunch {
    pub fn builder() -> CellBunchBuilder {
        CellBunchBuilder { cells: Vec::new() }
    }
}

impl CellBunchBuilder {
    pub fn build(self) -> CellBunch {
        CellBunch { cells: self.cells }
    }

    pub fn attach_cell(mut self, cell: Cell) -> CellBunchBuilder {
        self.cells.push(cell);

        self
    }
}
