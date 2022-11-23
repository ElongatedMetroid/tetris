use std::{cell::RefCell, rc::Rc};

use nalgebra::Vector2;

use crate::{
    cell::Cell,
    playfield::{Playfield, PLAYFIELD_HEIGHT, PLAYFIELD_WIDTH},
};

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

    /// Move the piece without checking if it will hit a, an area that is out of bounds, or b, another cell
    pub fn move_unchecked(&self, playfield: &mut Playfield, axis: Axis, amount: isize) {
        let mut cell_options = Vec::new();

        // Take all the cell pointers (at the old positions) from the grid
        for cell in &self.cells {
            // Take the cell from the grid
            cell_options.push(
                playfield.grid.grid[cell.borrow().position.y as usize]
                    [cell.borrow().position.x as usize]
                    .take(),
            );
        }

        // Put the cell pointers that we took from the grid earlier back into the grid at the updated positions. This is done in two seperate loops so that
        // we do not overwright the piece with itself.
        for (cell, cell_option) in self.cells.iter().zip(cell_options) {
            match axis {
                Axis::Horizontal => {
                    cell.borrow_mut().position.x += amount;
                }
                Axis::Vertical => {
                    cell.borrow_mut().position.y += amount;
                }
            }

            // Put the cell back in the grid at the updated position
            playfield.grid.grid[cell.borrow().position.y as usize]
                [cell.borrow().position.x as usize] = cell_option;
        }
    }

    /// Move the piece with checking if it will collide with either a, the bounds of the playfield, or b, another cell. If it does collide
    /// the piece will not move there and this will return false (will return true on a successful movement)
    pub fn move_checked(&self, playfield: &mut Playfield, axis: Axis, amount: isize) -> bool {
        // Check if the piece can move to the given position, then call move_unchecked
        // Check each of the positions that this piece will take up once moved make sure they are None
        for cell in &self.cells {
            let mut y = cell.borrow().position.y as isize;
            let mut x = cell.borrow().position.x as isize;

            match axis {
                Axis::Horizontal => {
                    x += amount;
                }
                Axis::Vertical => {
                    y += amount;
                }
            }

            // Check if we are trying to move out of bounds
            if x >= PLAYFIELD_WIDTH as isize || x < 0 || y >= PLAYFIELD_HEIGHT as isize || y < 0 {
                return false;
            }

            match &playfield.grid.grid[y as usize][x as usize] {
                Some(new_cell) => {
                    // Check if the cell where we are trying to move is part of our tetromino
                    if !self.cells.contains(&new_cell) {
                        return false;
                    }
                }
                None => (),
            }
        }

        self.move_unchecked(playfield, axis, amount);

        true
    }

    pub fn fall(&self, playfield: &mut Playfield) {
        while self.down(playfield) {}
    }

    pub fn up(&self, playfield: &mut Playfield) -> bool {
        self.move_checked(playfield, Axis::Vertical, -1)
    }

    pub fn down(&self, playfield: &mut Playfield) -> bool {
        self.move_checked(playfield, Axis::Vertical, 1)
    }

    pub fn left(&self, playfield: &mut Playfield) -> bool {
        self.move_checked(playfield, Axis::Horizontal, -1)
    }

    pub fn right(&self, playfield: &mut Playfield) -> bool {
        self.move_checked(playfield, Axis::Horizontal, 1)
    }
}
