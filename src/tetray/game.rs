use crate::tetray::{bag::{Bag, CASE_COUNT}, board::Board, rotation::get_kick_positions, score::Score, tetromino::{Matrix5x5, MinoKind, Rotate, Tetromino}};

const NEXT_PREVIEW_COUNT: usize = 5;

const SPAWN_POINT: (i32, i32) = (18, 2);
const ENDLINE: usize = 19;

pub enum Command {
    Move { col: i32 },
    Rotate { rot: Rotate },
    HardDrop,
    SoftDrop,
    Hold,
}

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

struct MinoHold {
    hold: MinoKind,
    holdable: bool,
}

pub struct Game {
    runtime_mino: RuntimeTetromino,
    mino_queue: MinoQueue,
    mino_hold: MinoHold,
    board: Board,
    score: Score,
    game_over: bool,
}

pub struct GameView {
    pub board: Board,
    pub nexts: [MinoKind; NEXT_PREVIEW_COUNT],
    pub hold: MinoKind,
    pub score: usize,
    pub highscore: usize,
    pub ended: bool,
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

    fn set_absolute_rotation(&mut self, rot: Rotate) {
        self.rotated = rot;
    }

    fn get_absolute_rotation(&self, relative: Rotate) -> Rotate {
        let from = self.rotated as i32;
        let to = from + relative as i32 - 1;
        match to.rem_euclid(4) {
            0 => Rotate::Left,
            1 => Rotate::Straight,
            2 => Rotate::Right,
            3 => Rotate::Half,
            _ => unreachable!("{to}"),
        }
    }

    fn get_current_shape(&self) -> &Matrix5x5 {
        self.mino.get_rotated_shape(self.rotated)
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

    fn preview_list(&self) -> [MinoKind; NEXT_PREVIEW_COUNT] {
        self.queue
    }
}

impl MinoHold {
    fn new() -> Self {
        Self {
            hold: MinoKind::None,
            holdable: true,
        }
    }
}

impl Game {
    pub fn new(seed: usize) -> Self {
        let mut queue = MinoQueue::new(seed);
        let mino = MinoKind::get_mino(queue.get_next());
        Self {
            runtime_mino: RuntimeTetromino::new(SPAWN_POINT.0, SPAWN_POINT.1, mino),
            mino_queue: queue,
            mino_hold: MinoHold::new(),
            board: Board::new(),
            score: Score::new(),
            game_over: false,
        }
    }

    pub fn command(&mut self, cmd: Command) -> Result<String, String> {
        if self.game_over {
            return Err(format!("Was Game Over"))
        }

        match cmd {
            Command::Move { col } => {
                let r = self.runtime_mino.r;
                let c = self.runtime_mino.c;
                if self.board.collision(r, c + col, self.runtime_mino.get_current_shape()) {
                    return Err(format!("Failed to Move to: (r: {r}, c: {c})"))
                }
                self.runtime_mino.move_horizontal(col);

                Ok(format!("Successed to Move to: (r: {r}, c: {c})"))
            },
            Command::Rotate { rot } => {
                let rt = &mut self.runtime_mino;
                let from = rt.rotated;
                let to = rt.get_absolute_rotation(rot);
                let shape = rt.mino.get_rotated_shape(to);
                let positions = get_kick_positions(
                    rt.mino.get_mino_kind(),
                    from,
                    to
                );

                for (r, c) in positions {
                    let kick_r = rt.r + r;
                    let kick_c = rt.c + c;
                    if self.board.collision(kick_r, kick_c, shape) {
                        continue;
                    }

                    rt.r = kick_r;
                    rt.c = kick_c;
                    rt.set_absolute_rotation(to);

                    return Ok(format!("Successed to Rotate from: {from} to: {to}"))
                }

                Err(format!("Failed to Rotate from: {from} to: {to}"))
            },
            Command::HardDrop => {
                let rt = &mut self.runtime_mino;
                let r = rt.r;
                let c = rt.c;
                let mut down = 1;
                while !self.board.collision(r + down, c, rt.get_current_shape()) {
                    down += 1;
                }
                rt.move_vertical(down - 1);
                self.drop_mino();

                Ok(format!("Successed to HardDrop"))
            },
            Command::SoftDrop => {
                self.drop_mino();

                Ok(format!("Successed to SoftDrop"))
            },
            Command::Hold => {
                if !self.mino_hold.holdable {
                    return Err(format!("Failed to Hold"))
                }

                let hold = self.mino_hold.hold;
                self.mino_hold.hold = self.runtime_mino.mino.get_mino_kind();
                if hold == MinoKind::None {
                    self.pick_mino();
                } else {
                    self.runtime_mino = RuntimeTetromino::new(
                        SPAWN_POINT.0,
                        SPAWN_POINT.1,
                        hold.get_mino()
                    );
                }

                self.mino_hold.holdable = false;

                Ok(format!("Successed to Hold"))
            },
        }
    }

    pub fn update(&mut self) {
        if self.game_over {
            return;
        }

        self.drop_mino();
    }

    fn drop_mino(&mut self) {
        let rt = &mut self.runtime_mino;
        let shape = rt.mino.get_rotated_shape(rt.rotated);
        if self.board.collision(rt.r + 1, rt.c, shape) {
            self.board.place(rt.r, rt.c, shape, rt.mino.get_mino_kind());

            let x = self.board.update_board();
            self.score.add_score((x + (1 << x >> 2)) * 100);

            self.pick_mino();
            self.mino_hold.holdable = true;
            if !self.board.is_empty_line(ENDLINE) {
                self.game_over = true;
            }
        } else {
            rt.move_vertical(1);
        }
    }

    fn pick_mino(&mut self) {
        let (mut r, c) = SPAWN_POINT;
        let kind = self.mino_queue.get_next();
        let mino = MinoKind::get_mino(kind);
        let shape = mino.get_rotated_shape(Rotate::Straight);
        if self.board.collision(r, c, shape) {
            r -= 1;
        }

        self.runtime_mino = RuntimeTetromino::new(r, c, mino);
    }

    pub fn view(&self) -> GameView {
        let mut board = self.board.clone();
        let rt = &self.runtime_mino;
        board.place(rt.r, rt.c, rt.get_current_shape(), rt.mino.get_mino_kind());

        GameView {
            board: board,
            nexts: self.mino_queue.preview_list(),
            hold: self.mino_hold.hold,
            score: self.score.get_score(),
            highscore: self.score.get_highscore(),
            ended: self.game_over,
        }
    }

    pub fn is_ended(&self) -> bool {
        self.game_over
    }
}