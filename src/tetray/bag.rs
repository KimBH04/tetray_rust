use crate::tetray::tetromino::{MINO_COUNT, MinoKind};

pub const CASE_COUNT: usize = {
    let mut i = 1;
    let mut r = 1;
    while i <= MINO_COUNT {
        r *= i;
        i += 1;
    }
    r
};
        
const MINOS: [MinoKind; MINO_COUNT] = [
    MinoKind::J,
    MinoKind::L,
    MinoKind::S,
    MinoKind::T,
    MinoKind::Z,
    MinoKind::I,
    MinoKind::O,
];

pub struct Bag {
    idx: usize,
    queue: [MinoKind; MINO_COUNT],
}

impl Bag {
    /// n 번째 미노 순열을 반환합니다. (0 번째 부터 시작)\
    /// 입력 된 값이 7!보다 크거나 같으면 에러를 반환합니다.
    pub fn new(mut nth: usize) -> Result<Self, String> {
        if nth >= CASE_COUNT {
            return Err(format!("'nth' must be less than {CASE_COUNT}"))
        }

        let mut index_vec = vec![];
        for i in 1..=MINO_COUNT {
            index_vec.push(nth % i);
            nth /= i;
        }

        let mut mino_vec = Vec::from_iter(MINOS);
        let mut res = [MinoKind::None; MINO_COUNT];
        for i in 0..MINO_COUNT {
            let index = index_vec.pop().unwrap();
            res[i] = mino_vec.remove(index);
        }

        Ok(Self {
            idx: 0,
            queue: res,
        })
    }

    /// 다음 값을 반환합니다.\
    /// 다음이 없으면 None 입니다.
    pub fn get_next(&mut self) -> Option<MinoKind> {
        if self.idx >= MINO_COUNT {
            return None
        }

        let index = self.idx;
        self.idx += 1;
        Some(self.queue[index])
    }
}