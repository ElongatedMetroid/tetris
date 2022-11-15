use crate::{grid::Grid, cell::Cell};

pub struct Playfield {
    /// The grid will hold refrences to Cell's
    grid: Grid<&Cell>
}

impl Playfield {
    /// Create a new Playfield instance
    pub fn new() -> Self {
        Self {
            grid: Grid::<&Cell>::new()
        }
    }
}