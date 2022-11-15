use crate::{grid::Grid, cell::Cell};

pub struct Playfield<'a> {
    /// The grid will hold refrences to Cell's
    grid: Grid<&'a Cell>
}

impl Playfield<'_> {
    /// Create a new Playfield instance
    pub fn new() -> Self {
        Self {
            grid: Grid::<&Cell>::new()
        }
    }
}