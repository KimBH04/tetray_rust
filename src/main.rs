mod tetray;

use std::time::{SystemTime, UNIX_EPOCH};

use sola_raylib::{color::Color, drawing::RaylibDraw};

use crate::tetray::bag::{Bag, CASE_COUNT};
use crate::tetray::tetromino::{Rotate, SIZE, Tetromino};

const CELL: i32 = 20;
const GAP: i32 = 10;

/// 현재 유닉스 시간(나노초)을 시드로 하는 xorshift64 의사 난수
fn time_random() -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    let mut x = nanos | 1;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

fn main() {
    let (mut win, thread) = sola_raylib::init()
        .size(800, 600)
        .title("Tetray")
        .build();

    // 1초마다 갱신
    win.set_target_fps(1);

    while !win.window_should_close() {
        let nth = (time_random() % CASE_COUNT as u64) as usize;
        let mut bag = Bag::new(nth).expect("nth out of range");

        let mut minos = Vec::new();
        while let Some(kind) = bag.get_next() {
            minos.push(kind);
        }

        let mut d = win.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        let base_y = 200;
        for (i, kind) in minos.iter().enumerate() {
            let base_x = GAP + i as i32 * (SIZE as i32 * CELL + GAP);
            let mino = Tetromino::get_mino(*kind);
            for r in 0..SIZE {
                for c in 0..SIZE {
                    let color = if mino.get_rotated_shape(Rotate::Straight).get_cell(r, c) {
                        mino.get_color()
                    } else {
                        Color::BLACK
                    };
                    d.draw_rectangle(
                        base_x + c as i32 * CELL,
                        base_y + r as i32 * CELL,
                        CELL,
                        CELL,
                        color,
                    );
                }
            }
        }

        let text = format!("Permutation #{} / {}", nth, CASE_COUNT);
        let text_y = base_y + SIZE as i32 * CELL + 20;
        d.draw_text(&text, GAP, text_y, 30, Color::DARKGRAY);
    }
}
