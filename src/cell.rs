use std::rc::Rc;

use nalgebra::Vector2;

pub struct Cell {
    character: char,
    position: Vector2<isize>,
}

/// A collection of Cell's attached in a certain way
pub struct CellBunch {
    /// Main cell of the bunch, this cell will be used to rotate around
    main_cell: Cell,
    /// Vector of positions in which attached cells will be at (thinking of the main_cell as position 0,0)
    attached_cells: Vec<Rc<Cell>>
}

pub struct CellBunchBuilder {
    main_cell: Cell,
    attached_cells: Vec<Rc<Cell>>
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
        CellBunch { main_cell: self.main_cell, attached_cells: self.attached_cells }
    }

    pub fn attach_cell(mut self, cell: Cell) -> CellBunchBuilder {
        self.attached_cells.push(Rc::new(cell));

        self
    }
}
