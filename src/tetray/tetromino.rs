use sola_raylib::color::Color;

const X: bool = false;
const O: bool = true;

pub const ROTATED_COUNT: usize = 4;
pub const SIZE: usize = 5;

pub const MINO_J: &Tetromino = &Tetromino::new(
    MinoKind::J,
    Color::new(60, 120, 255, 255),
    Matrix5x5 {
        shape: [
            [X, X, X, X, X],
            [X, O, X, X, X],
            [X, O, O, O, X],
            [X, X, X, X, X],
            [X, X, X, X, X],
        ]
    }
);

pub const MINO_L: &Tetromino = &Tetromino::new(
    MinoKind::L,
    Color::new(255, 150, 30, 255),
    Matrix5x5 {
        shape: [
            [X, X, X, X, X],
            [X, X, X, O, X],
            [X, O, O, O, X],
            [X, X, X, X, X],
            [X, X, X, X, X],
        ]
    }
);

pub const MINO_S: &Tetromino = &Tetromino::new(
    MinoKind::S,
    Color::new(60, 230, 80, 255),
    Matrix5x5 {
        shape: [
            [X, X, X, X, X],
            [X, X, O, O, X],
            [X, O, O, X, X],
            [X, X, X, X, X],
            [X, X, X, X, X],
        ]
    }
);

pub const MINO_T: &Tetromino = &Tetromino::new(
    MinoKind::T,
    Color::new(190, 80, 255, 255),
    Matrix5x5 {
        shape: [
            [X, X, X, X, X],
            [X, X, O, X, X],
            [X, O, O, O, X],
            [X, X, X, X, X],
            [X, X, X, X, X],
        ]
    }
);

pub const MINO_Z: &Tetromino = &Tetromino::new(
    MinoKind::Z,
    Color::new(255, 60, 60, 255),
    Matrix5x5 {
        shape: [
            [X, X, X, X, X],
            [X, O, O, X, X],
            [X, X, O, O, X],
            [X, X, X, X, X],
            [X, X, X, X, X],
        ]
    }
);

pub const MINO_I: &Tetromino = &Tetromino::new(
    MinoKind::I,
    Color::new(0, 240, 240, 255),
    Matrix5x5 {
        shape: [
            [X, X, X, X, X],
            [X, X, X, X, X],
            [X, O, O, O, O],
            [X, X, X, X, X],
            [X, X, X, X, X],
        ]
    }
);

pub const MINO_O: &Tetromino = &Tetromino::new(
    MinoKind::O,
    Color::new(255, 220, 0, 255),
    Matrix5x5 {
        shape: [
            [X, X, X, X, X],
            [X, X, O, O, X],
            [X, X, O, O, X],
            [X, X, X, X, X],
            [X, X, X, X, X],
        ]
    }
);

#[derive(Clone, Copy)]
pub enum Rotate {
    Left,
    Straight,
    Right,
    Half,
}

#[derive(Clone, Copy, PartialEq)]
pub enum MinoKind { None, J, L, S, T, Z, I, O }

pub struct Matrix5x5 {
    shape: [[bool; SIZE]; SIZE]
}

pub struct Tetromino {
    mino_kind: MinoKind,
    color: Color,
    rotated_shape: [Matrix5x5; ROTATED_COUNT]
}

impl Matrix5x5 {
    pub fn get_cell(&self, r: usize, c: usize) -> bool {
        r < SIZE && c < SIZE && self.shape[r][c]
    }
}

impl Tetromino {
    const fn new(mino_kind: MinoKind, color: Color, raw_shape: Matrix5x5) -> Self {
        let mut left = [[false; SIZE]; SIZE];
        let mut right = [[false; SIZE]; SIZE];
        let mut half = [[false; SIZE]; SIZE];

        let mut r = 0;
        let mut c = 0;
        while r < SIZE {
            while c < SIZE {
                let raw_cell = raw_shape.shape[r][c];
                left[SIZE - (c + 1)][r] = raw_cell;
                right[c][SIZE - (r + 1)] = raw_cell;
                half[SIZE - (r + 1)][SIZE - (c + 1)] = raw_cell;

                c += 1;
            }
            r += 1;
            c = 0;
        }

        Self {
            mino_kind,
            color: color,
            rotated_shape: [
                Matrix5x5 { shape: left },
                raw_shape,
                Matrix5x5 { shape: right },
                Matrix5x5 { shape: half }
            ]
        }
    }

    pub fn get_mino_kind(&self) -> MinoKind {
        self.mino_kind
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_rotated_shape(&self, rotate: Rotate) -> &Matrix5x5 {
        let idx = match rotate {
            Rotate::Left  => 0,
            Rotate::Straight  => 1,
            Rotate::Right => 2,
            Rotate::Half  => 3,
        };
        &self.rotated_shape[idx]
    }
}
