use nalgebra::Vector2;

#[derive(PartialEq, Eq, Clone)]
pub struct Cell {
    pub(crate) character: char,
    pub(crate) position: Vector2<isize>,
}

impl Cell {
    pub fn new(character: char, position: Vector2<isize>) -> Self {
        Cell {
            character,
            position,
        }
    }
}
