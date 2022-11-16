use nalgebra::Vector2;

pub struct Cell {
    pub(crate) character: char,
    pub(crate) position: Vector2<usize>,
}
