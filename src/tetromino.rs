use std::{cell::RefCell, rc::Rc};

use nalgebra::Vector2;

use crate::cell::Cell;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Axis {
    Horizontal,
    Vertical,
}

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
    pub(crate) kind: TetrominoKind,
    // All the Cells that make up the tetromino
    pub(crate) cells: Vec<Rc<RefCell<Cell>>>,
}

impl Tetromino {
    pub fn new(can_move: bool, kind: TetrominoKind, position: Vector2<isize>) -> Self {
        let tetromino_cells = match kind {
            TetrominoKind::I => vec![
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x, position.y),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x + 0, position.y + 1),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x + 0, position.y - 1),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x + 0, position.y - 2),
                ))),
            ],
            TetrominoKind::J => vec![
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x, position.y),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x + 0, position.y + 1),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x + 0, position.y - 1),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x - 1, position.y + 1),
                ))),
            ],
            TetrominoKind::L => vec![
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x, position.y),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x + 0, position.y + 1),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x + 0, position.y - 1),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x + 1, position.y + 1),
                ))),
            ],
            TetrominoKind::O => vec![
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x, position.y),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x + 0, position.y + 1),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x + 1, position.y + 0),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x + 1, position.y + 1),
                ))),
            ],
            TetrominoKind::S => vec![
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x, position.y),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x + 1, position.y + 0),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x + 0, position.y + 1),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x - 1, position.y + 1),
                ))),
            ],
            TetrominoKind::T => vec![
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x, position.y),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x - 1, position.y + 0),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x + 0, position.y + 1),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x + 1, position.y + 0),
                ))),
            ],
            TetrominoKind::Z => vec![
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x, position.y),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x + 1, position.y + 0),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x + 0, position.y - 1),
                ))),
                Rc::new(RefCell::new(Cell::new(
                    can_move,
                    '■',
                    Vector2::new(position.x - 1, position.y - 1),
                ))),
            ],
        };

        Self {
            kind,
            cells: tetromino_cells,
        }
    }

    /// Returns a vector containing all basic tetrominos
    pub fn all(can_move: bool, position: Vector2<isize>) -> Vec<Tetromino> {
        let i = Tetromino::new(can_move, TetrominoKind::I, position);
        let j = Tetromino::new(can_move, TetrominoKind::J, position);
        let l = Tetromino::new(can_move, TetrominoKind::L, position);
        let o = Tetromino::new(can_move, TetrominoKind::O, position);
        let s = Tetromino::new(can_move, TetrominoKind::S, position);
        let t = Tetromino::new(can_move, TetrominoKind::T, position);
        let z = Tetromino::new(can_move, TetrominoKind::Z, position);

        let pieces = vec![i, j, l, o, s, t, z];

        pieces
    }

    pub fn move_unchecked(&self, axis: Axis, amount: isize) {
        for cell in &self.cells {
            match axis {
                Axis::Horizontal => {
                    cell.borrow_mut().position.x += amount;
                }
                Axis::Vertical => {
                    cell.borrow_mut().position.y += amount;
                }
            }
        }
    }
}
