use nalgebra::Vector2;

type TetrominoStr = &'static str;

/// ■■■■
static I: TetrominoStr = 
    "■■■■";
///  ■
///  ■
/// ■■
static J: TetrominoStr = 
    " ■\n ■\n■■";
/// ■
/// ■
/// ■■
static L: TetrominoStr = 
    "■\n■\n■■";
/// ■■
/// ■■
static O: TetrominoStr = "■■\n■■";
///  ■■
/// ■■
static S: TetrominoStr = " ■■\n■■";
/// ■■■
///  ■
static T: TetrominoStr = "■■■\n ■";
/// ■■
///  ■■
static Z: TetrominoStr = "■■\n ■■";

pub enum TetrominoKind {
    I(TetrominoStr),
    J(TetrominoStr),
    L(TetrominoStr),
    O(TetrominoStr),
    S(TetrominoStr),
    T(TetrominoStr),
    Z(TetrominoStr),
}

pub struct Tetromino {
    kind: TetrominoKind,
    position: Vector2<usize>,
    rotation: usize,
}