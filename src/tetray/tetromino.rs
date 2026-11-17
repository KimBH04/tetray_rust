use sola_raylib::color::Color;

const X: bool = false;
const O: bool = true;

pub const ROTATED_COUNT: usize = 4;
pub const MINO_SIZE: usize = 5;
pub const MINO_COUNT: usize = 7;

pub const MINO_NONE: Tetromino = Tetromino::new(
    MinoKind::None,
    Color::new(0, 0, 0, 0),
    Matrix5x5 { shape: [[X; MINO_SIZE]; MINO_SIZE] }
);

pub const MINO_J: Tetromino = Tetromino::new(
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

pub const MINO_L: Tetromino = Tetromino::new(
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

pub const MINO_S: Tetromino = Tetromino::new(
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

pub const MINO_T: Tetromino = Tetromino::new(
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

pub const MINO_Z: Tetromino = Tetromino::new(
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

pub const MINO_I: Tetromino = Tetromino::new(
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

pub const MINO_O: Tetromino = Tetromino::new(
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
    Left = 0,
    Straight = 1,
    Right = 2,
    Half = 3,
}

#[derive(Clone, Copy, PartialEq)]
pub enum MinoKind { None, J, L, S, T, Z, I, O }

pub struct Matrix5x5 {
    shape: [[bool; MINO_SIZE]; MINO_SIZE]
}

pub struct Tetromino {
    mino_kind: MinoKind,
    color: Color,
    rotated_shape: [Matrix5x5; ROTATED_COUNT]
}

impl Matrix5x5 {
    pub fn get_cell(&self, r: usize, c: usize) -> bool {
        r < MINO_SIZE && c < MINO_SIZE && self.shape[r][c]
    }
}

impl MinoKind {
    pub const fn get_color(self) -> Color {
        match self {
            MinoKind::None => Color::new(0, 0, 0, 0),
            MinoKind::J => Color::new(60, 120, 255, 255),
            MinoKind::L => Color::new(255, 150, 30, 255),
            MinoKind::S => Color::new(60, 230, 80, 255),
            MinoKind::T => Color::new(190, 80, 255, 255),
            MinoKind::Z => Color::new(255, 60, 60, 255),
            MinoKind::I => Color::new(0, 240, 240, 255),
            MinoKind::O => Color::new(255, 220, 0, 255),
        }
    }

    pub const fn get_mino(self) -> Tetromino {
        match self {
            MinoKind::None => MINO_NONE,
            MinoKind::J => MINO_J,
            MinoKind::L => MINO_L,
            MinoKind::S => MINO_S,
            MinoKind::T => MINO_T,
            MinoKind::Z => MINO_Z,
            MinoKind::I => MINO_I,
            MinoKind::O => MINO_O,
        }
    }
}

impl Tetromino {
    const fn new(mino_kind: MinoKind, color: Color, raw_shape: Matrix5x5) -> Self {
        let mut left = [[false; MINO_SIZE]; MINO_SIZE];
        let mut right = [[false; MINO_SIZE]; MINO_SIZE];
        let mut half = [[false; MINO_SIZE]; MINO_SIZE];

        let mut r = 0;
        let mut c = 0;
        while r < MINO_SIZE {
            while c < MINO_SIZE {
                let raw_cell = raw_shape.shape[r][c];
                left[MINO_SIZE - (c + 1)][r] = raw_cell;
                right[c][MINO_SIZE - (r + 1)] = raw_cell;
                half[MINO_SIZE - (r + 1)][MINO_SIZE - (c + 1)] = raw_cell;

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
