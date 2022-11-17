use nalgebra::Vector2;

pub struct Cell {
    /// Controls whether the cell can move or not (ie fall)
    pub(crate) can_move: bool,
    pub(crate) character: char,
    pub(crate) position: Vector2<isize>,
}

impl Cell {
    pub fn new(can_move: bool, character: char, position: Vector2<isize>) -> Self {
        Cell {
            can_move,
            character,
            position,
        }
    }
}
