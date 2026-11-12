use crate::tetray::tetromino::{Matrix5x5, SIZE, MinoKind};

pub const BOARD_DEPTH: usize = 40;
pub const BOARD_WIDTH: usize = 10;

pub struct Board {
    cells: [[MinoKind; BOARD_WIDTH]; BOARD_DEPTH],
}

impl Board {
    pub fn new() -> Self {
        Self { cells: [[MinoKind::None; BOARD_WIDTH]; BOARD_DEPTH], }
    }

    pub fn get_cell(&self, r: usize, c: usize) -> Option<MinoKind> {
        if r >= BOARD_DEPTH || c >= BOARD_WIDTH {
            None
        } else {
            Some(self.cells[r][c])
        }
    }

    pub fn collision_check(&self, r: i32, c: i32, shape: Matrix5x5) -> bool {
        for shape_r in 0..SIZE {
            for shape_c in 0..SIZE {
                if !shape.get_cell(shape_r, shape_c) {
                    continue;
                }

                let current_r = r as usize + shape_r;
                let current_c = c as usize + shape_c;
                if current_r >= BOARD_DEPTH || current_c >= BOARD_WIDTH || self.cells[current_r][current_c] != MinoKind::None {
                    return false;
                }
            }
        }

        true
    }

    /// 채워진 줄을 지우고 차례대로 아래로 내립니다.\
    /// 지워진 줄 개수를 반환합니다.
    pub fn update_board(&mut self) -> usize {
        let mut remove_cnt = 0;

        'line: for line_index in (BOARD_DEPTH - 1)..=0 {
            for cell_index in 0..BOARD_WIDTH {
                if self.cells[line_index][cell_index] == MinoKind::None {
                    self.cells[line_index + remove_cnt] = self.cells[line_index];
                    continue 'line;
                }
            }
            
            remove_cnt += 1;
        }
        
        remove_cnt
    }
}