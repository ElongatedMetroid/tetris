use nalgebra::Vector2;

use crate::cell::{CellBunch, Cell};

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
    cell_data: CellBunch,
    rotation: usize,
}

impl Tetromino {
    pub fn new(kind: TetrominoKind, position: Vector2<isize>, rotation: usize) -> Self {
        match kind {
            TetrominoKind::I => {
                Tetromino {
                    kind,
                    cell_data: 
                        CellBunch::builder(Cell::new('■', position))
                            .build(),
                    rotation,
                }
            },
            TetrominoKind::J => todo!(),
            TetrominoKind::L => todo!(),
            TetrominoKind::O => todo!(),
            TetrominoKind::S => todo!(),
            TetrominoKind::T => todo!(),
            TetrominoKind::Z => todo!(),
        }
    }
}