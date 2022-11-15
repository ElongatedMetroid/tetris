use crate::playfield::Playfield;

pub struct Game {
    playfield: Playfield,
}

impl Game {
    /// Create a new instance of the game
    pub fn new() -> Self {
        Self {
            playfield: Playfield::new(),
        }
    }
    /// Run the game
    pub fn run(self) {

    }
}