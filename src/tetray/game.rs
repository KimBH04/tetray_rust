use sola_raylib::color::Color;

use crate::tetray::{bag::{Bag, CASE_COUNT}, board::{BOARD_DEPTH, BOARD_WIDTH, Board}, tetromino::{MinoKind, Rotate, MINO_SIZE, Tetromino}};

const NEXT_PREVIEW_COUNT: usize = 5;

const SPAWN_POINT: (i32, i32) = (18, 2);

struct RuntimeTetromino {
    r: i32,
    c: i32,
    mino: Tetromino,
    rotated: Rotate
}

struct MinoQueue {
    queue: [MinoKind; NEXT_PREVIEW_COUNT],
    bag: Bag,
    shuffled_idx: usize,
    multi: usize,
    add: usize,
}

pub struct Game {
    runtime_mino: RuntimeTetromino,

    mino_queue: MinoQueue,
    hold: MinoKind,

    board: Board,
}

impl RuntimeTetromino {
    fn new(r: i32, c: i32, mino: Tetromino) -> Self {
        Self {
            r: r,
            c: c,
            mino: mino,
            rotated: Rotate::Straight
        }
    }

    fn move_vertical(&mut self, r: i32) {
        self.r += r;
    }

    fn move_horizontal(&mut self, c: i32) {
        self.c += c;
    }

    fn relative_rotation(&mut self, delta: Rotate) {
        let from = self.rotated as i32;
        let to = from + delta as i32 - 1;
        self.rotated = match to.rem_euclid(4) {
            0 => Rotate::Left,
            1 => Rotate::Straight,
            2 => Rotate::Right,
            3 => Rotate::Half,
            _ => unreachable!("{to}"),
        };
    }
}

impl MinoQueue {
    fn new(seed: usize) -> Self {
        let idx = seed % CASE_COUNT;
        let mut bag = Bag::new(idx).unwrap();
        let mut queue = [MinoKind::None; NEXT_PREVIEW_COUNT];
        for i in 0..NEXT_PREVIEW_COUNT {
            queue[i] = bag.get_next().unwrap();
        }

        Self {
            queue: queue,
            bag: bag,
            shuffled_idx: idx,
            multi: 420 * (seed % 11 + 1) + 1,
            add: {
                let coprimes = [11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 1013, 1357, 2011, 3137, 4001, 4999];
                coprimes[seed % coprimes.len()]
            },
        }
    }

    fn get_next(&mut self) -> MinoKind {
        let next = self.queue[0];
        for i in 1..NEXT_PREVIEW_COUNT {
            self.queue[i - 1] = self.queue[i];
        }
        
        self.queue[NEXT_PREVIEW_COUNT - 1] =
            if let Some(kind) = self.bag.get_next() {
                kind
            } else {
                self.shuffled_idx *= self.multi;
                self.shuffled_idx += self.add;
                self.shuffled_idx %= CASE_COUNT;
                self.bag = Bag::new(self.shuffled_idx).unwrap();
                self.bag.get_next().unwrap()
            };
        
        next
    }

    fn preview_list(&self) -> &[MinoKind; NEXT_PREVIEW_COUNT] {
        &self.queue
    }
}

impl Game {
    pub fn new(seed: usize) -> Self {
        let mut queue = MinoQueue::new(seed);
        let mino = MinoKind::get_mino(queue.get_next());
        Self {
            runtime_mino: RuntimeTetromino::new(SPAWN_POINT.0, SPAWN_POINT.1, mino),
            mino_queue: queue,
            hold: MinoKind::None,
            board: Board::new(),
        }
    }

    pub fn update(&mut self) {
        let rt = &mut self.runtime_mino;
        let shape = rt.mino.get_rotated_shape(rt.rotated);
        if self.board.collision(rt.r + 1, rt.c, shape) {
            self.board.place(rt.r, rt.c, shape, rt.mino.get_mino_kind());
            self.board.update_board();
            self.runtime_mino = RuntimeTetromino::new(
                SPAWN_POINT.0,
                SPAWN_POINT.1,
                MinoKind::get_mino(self.mino_queue.get_next())
            );
        } else {
            rt.move_vertical(1);
        }
    }

    pub fn get_cell_color(&self, r: usize, c: usize) -> Option<Color> {
        if r >= BOARD_DEPTH || c >= BOARD_WIDTH {
            return None
        }

        let rt = &self.runtime_mino;
        let size_i32 = MINO_SIZE;
        let shape_r = (r as i32 - rt.r) as usize;
        let shape_c = (c as i32 - rt.c) as usize;
        if shape_r < size_i32 &&
            shape_c < size_i32 &&
            rt.mino.get_rotated_shape(rt.rotated).get_cell(shape_r, shape_c) {
            return Some(rt.mino.get_color())
        }

        if let Some(kind) = self.board.get_cell(r, c) {
            Some(kind.get_color())
        } else {
            None
        }
    }
}