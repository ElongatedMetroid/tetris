use nalgebra::Vector2;

use crate::cell::{Cell, CellBunch};

#[derive(Clone, Copy)]
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
    pub(super) cell_data: CellBunch,
    rotation: usize,
}

impl Tetromino {
    pub fn new(kind: TetrominoKind, position: Vector2<isize>, rotation: usize) -> Self {
        Tetromino {
            kind: kind,
            cell_data: match kind {
                // TODO: Have all tetrominos stored as statics, so they dont have to be loaded into memory each time
                TetrominoKind::I => {
                    // ■ <-- Main
                    CellBunch::builder(Cell::new('▢', position))
                        // ■ <-- Main
                        // ■ <-- this
                        .attach_cell(Cell::new('■', Vector2::new(0, 1)))
                        // ■ <-- This
                        // ■ <-- Main
                        // ■
                        .attach_cell(Cell::new('■', Vector2::new(0, -1)))
                        // ■ <-- This
                        // ■
                        // ■ <-- Main
                        // ■
                        .attach_cell(Cell::new('■', Vector2::new(0, -2)))
                        .build()
                }
                TetrominoKind::J => todo!(),
                TetrominoKind::L => todo!(),
                TetrominoKind::O => todo!(),
                TetrominoKind::S => todo!(),
                TetrominoKind::T => todo!(),
                TetrominoKind::Z => todo!(),
            },
            rotation,
        }
    }
}
