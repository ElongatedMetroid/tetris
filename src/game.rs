use crate::playfield::Playfield;

pub struct Game {
    // (Temporarily public for testing purposes  )
    pub playfield: Playfield,
}

impl Game {
    pub fn new() -> Self {
        Self {
            playfield: Playfield::new(),
        }
    }
    pub fn print_playfield(&self) {
        if cfg!(debug_assertions) {
            print!("{:?}", self.playfield);
        } else {
            print!("{}", self.playfield);
        }
    }
}