use nalgebra::Vector2;

use crate::cell::Cell;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TetrominoKind {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

pub struct Tetromino {
    kind: TetrominoKind,
    // All the Cells that make up the tetromino
    cells: Vec<Cell>,
}

impl Tetromino {
    pub fn new(kind: TetrominoKind, position: Vector2<usize>) -> Self {
        let tetromino_cells = match kind {
            TetrominoKind::I => vec![
                Cell::new('■', Vector2::new(position.x, position.y)),
                Cell::new('■', Vector2::new(position.x + 0, position.y + 1)),
                Cell::new('■', Vector2::new(position.x + 0, position.y + 1)),
                Cell::new('■', Vector2::new(position.x + 0, position.y - 2)),
            ],
            TetrominoKind::J => vec![
                Cell::new('■', Vector2::new(position.x, position.y)),
                Cell::new('■', Vector2::new(position.x + 0, position.y + 1)),
                Cell::new('■', Vector2::new(position.x + 0, position.y - 1)),
                Cell::new('■', Vector2::new(position.x - 1, position.y + 1)),
            ],
            TetrominoKind::L => vec![
                Cell::new('■', Vector2::new(position.x, position.y)),
                Cell::new('■', Vector2::new(position.x + 0, position.y + 1)),
                Cell::new('■', Vector2::new(position.x + 0, position.y - 1)),
                Cell::new('■', Vector2::new(position.x + 1, position.y + 1)),
            ],
            TetrominoKind::O => vec![
                Cell::new('■', Vector2::new(position.x, position.y)),
                Cell::new('■', Vector2::new(position.x + 0, position.y + 1)),
                Cell::new('■', Vector2::new(position.x + 1, position.y + 0)),
                Cell::new('■', Vector2::new(position.x + 1, position.y + 1)),
            ],
            TetrominoKind::S => vec![
                Cell::new('■', Vector2::new(position.x, position.y)),
                Cell::new('■', Vector2::new(position.x + 1, position.y + 0)),
                Cell::new('■', Vector2::new(position.x + 0, position.y + 1)),
                Cell::new('■', Vector2::new(position.x - 1, position.y + 1)),
            ],
            TetrominoKind::T => vec![
                Cell::new('■', Vector2::new(position.x, position.y)),
                Cell::new('■', Vector2::new(position.x - 1, position.y + 0)),
                Cell::new('■', Vector2::new(position.x + 0, position.y + 1)),
                Cell::new('■', Vector2::new(position.x + 1, position.y + 0)),
            ],
            TetrominoKind::Z => vec![
                Cell::new('■', Vector2::new(position.x, position.y)),
                Cell::new('■', Vector2::new(position.x + 1, position.y + 0)),
                Cell::new('■', Vector2::new(position.x + 0, position.y - 1)),
                Cell::new('■', Vector2::new(position.x - 1, position.y - 1)),
            ],
        };

        Self {
            kind,
            cells: tetromino_cells,
        }
    }

    /// Returns a vector containing all basic tetrominos
    pub fn all(position: Vector2<usize>) -> Vec<Tetromino> {
        let i = Tetromino::new(TetrominoKind::I, position);
        let j = Tetromino::new(TetrominoKind::J, position);
        let l = Tetromino::new(TetrominoKind::L, position);
        let o = Tetromino::new(TetrominoKind::O, position);
        let s = Tetromino::new(TetrominoKind::S, position);
        let t = Tetromino::new(TetrominoKind::T, position);
        let z = Tetromino::new(TetrominoKind::Z, position);

        let pieces = vec![i, j, l, o, s, t, z];

        pieces
    }
}
