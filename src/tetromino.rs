use nalgebra::Vector2;

use crate::cell::{Cell, CellBunch};

#[derive(Clone, Copy, Debug)]
pub enum TetrominoKind {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Clone)]
pub struct Tetromino {
    pub kind: TetrominoKind,
    pub(super) cell_data: CellBunch,
    rotation: usize,
}

impl Tetromino {
    pub fn new(kind: TetrominoKind, position: Vector2<isize>, rotation: usize) -> Self {
        Tetromino {
            kind,
            cell_data: match kind {
                // TODO: Have all tetrominos stored as statics, so they dont have to be loaded into memory each time
                TetrominoKind::I => {
                    // ■ <-- Main
                    CellBunch::builder()
                        .attach_cell(Cell::new('■', Vector2::new(position.x, position.y)))
                        // ■ <-- Main
                        // ■ <-- this
                        .attach_cell(Cell::new('■', Vector2::new(position.x + 0, position.y + 1)))
                        // ■ <-- This
                        // ■ <-- Main
                        // ■
                        .attach_cell(Cell::new('■', Vector2::new(position.x + 0, position.y - 1)))
                        // ■ <-- This
                        // ■
                        // ■ <-- Main
                        // ■
                        .attach_cell(Cell::new('■', Vector2::new(position.x + 0, position.y - 2)))
                        .build()
                }
                TetrominoKind::J => {
                    // ■ <-- Main
                    CellBunch::builder()
                        .attach_cell(Cell::new('■', Vector2::new(position.x, position.y)))
                        // ■ <-- Main
                        // ■ <-- this
                        .attach_cell(Cell::new('■', Vector2::new(position.x + 0, position.y + 1)))
                        // ■ <-- This
                        // ■ <-- Main
                        // ■
                        .attach_cell(Cell::new('■', Vector2::new(position.x + 0, position.y - 1)))
                        //  ■
                        //  ■ <-- Main
                        // ■■
                        .attach_cell(Cell::new('■', Vector2::new(position.x - 1, position.y + 1)))
                        .build()
                }
                TetrominoKind::L => {
                    // ■ <-- Main
                    CellBunch::builder()
                        .attach_cell(Cell::new('■', Vector2::new(position.x, position.y)))
                        // ■ <-- Main
                        // ■ <-- this
                        .attach_cell(Cell::new('■', Vector2::new(position.x + 0, position.y + 1)))
                        // ■ <-- This
                        // ■ <-- Main
                        // ■
                        .attach_cell(Cell::new('■', Vector2::new(position.x + 0, position.y - 1)))
                        // ■
                        // ■ <-- Main
                        // ■■
                        .attach_cell(Cell::new('■', Vector2::new(position.x + 1, position.y + 1)))
                        .build()
                }
                TetrominoKind::O => CellBunch::builder()
                    .attach_cell(Cell::new('■', Vector2::new(position.x, position.y)))
                    .attach_cell(Cell::new('■', Vector2::new(position.x + 0, position.y + 1)))
                    .attach_cell(Cell::new('■', Vector2::new(position.x + 1, position.y + 0)))
                    .attach_cell(Cell::new('■', Vector2::new(position.x + 1, position.y + 1)))
                    .build(),
                TetrominoKind::S => CellBunch::builder()
                    .attach_cell(Cell::new('■', Vector2::new(position.x, position.y)))
                    .attach_cell(Cell::new('■', Vector2::new(position.x + 1, position.y + 0)))
                    .attach_cell(Cell::new('■', Vector2::new(position.x + 0, position.y + 1)))
                    .attach_cell(Cell::new('■', Vector2::new(position.x - 1, position.y + 1)))
                    .build(),
                TetrominoKind::T => CellBunch::builder()
                    .attach_cell(Cell::new('■', Vector2::new(position.x, position.y)))
                    .attach_cell(Cell::new('■', Vector2::new(position.x - 1, position.y + 0)))
                    .attach_cell(Cell::new('■', Vector2::new(position.x + 0, position.y + 1)))
                    .attach_cell(Cell::new('■', Vector2::new(position.x + 1, position.y + 0)))
                    .build(),
                TetrominoKind::Z => CellBunch::builder()
                    .attach_cell(Cell::new('■', Vector2::new(position.x, position.y)))
                    .attach_cell(Cell::new('■', Vector2::new(position.x + 1, position.y + 0)))
                    .attach_cell(Cell::new('■', Vector2::new(position.x + 0, position.y - 1)))
                    .attach_cell(Cell::new('■', Vector2::new(position.x - 1, position.y - 1)))
                    .build(),
            },
            rotation,
        }
    }

    /// Returns a vector containing all basic tetrominos
    pub fn all(position: Vector2<isize>) -> Vec<Tetromino> {
        let i = Tetromino::new(TetrominoKind::I, position, 0);
        let j = Tetromino::new(TetrominoKind::J, position, 0);
        let l = Tetromino::new(TetrominoKind::L, position, 0);
        let o = Tetromino::new(TetrominoKind::O, position, 0);
        let s = Tetromino::new(TetrominoKind::S, position, 0);
        let t = Tetromino::new(TetrominoKind::T, position, 0);
        let z = Tetromino::new(TetrominoKind::Z, position, 0);

        let pieces = vec![i, j, l, o, s, t, z];

        pieces
    }
}
