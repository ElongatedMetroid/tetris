use nalgebra::Vector2;

pub struct Cell {
    pub(crate) character: char,
    pub(crate) position: Vector2<usize>,
}

impl Cell {
    pub fn new(character: char, position: Vector2<usize>) -> Self {
        Cell {
            character,
            position,
        }
    }
}
