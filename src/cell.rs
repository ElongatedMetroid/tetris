use std::fmt;

use nalgebra::Vector2;

#[derive(Clone, Debug)]
pub struct Cell {
    pub(super) character: char,
    /// Only used for spawning in the tetromino
    pub(super) position: Vector2<isize>,
}

/// A collection of Cell's attached in a certain way
pub struct CellBunch {
    /// Main cell of the bunch, this cell will be used to rotate around
    pub(super) main_cell: Cell,
    /// Vector of Cell's (the position of the cells is relative to the main_cell)
    pub(super) attached_cells: Vec<Cell>,
}

pub struct CellBunchBuilder {
    main_cell: Cell,
    attached_cells: Vec<Cell>,
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
    pub fn builder(main_cell: Cell) -> CellBunchBuilder {
        CellBunchBuilder {
            main_cell,
            attached_cells: Vec::new(),
        }
    }
}

impl CellBunchBuilder {
    pub fn build(self) -> CellBunch {
        CellBunch {
            main_cell: self.main_cell,
            attached_cells: self.attached_cells,
        }
    }

    pub fn attach_cell(mut self, cell: Cell) -> CellBunchBuilder {
        self.attached_cells.push(cell);

        self
    }
}
