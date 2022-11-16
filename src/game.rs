use crate::playfield::Playfield;

pub struct Game<'a> {
    playfield: Playfield<'a>,
}

impl Game<'_> {
    /// Create a new instance of the game
    pub fn new() -> Self {
        Self {
            playfield: Playfield::new(),
        }
    }
    /// Run the game
    pub fn run(self) {
        println!("{}", self.playfield);
    }
}
