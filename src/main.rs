mod tetray;

use std::time::{SystemTime, UNIX_EPOCH};

use sola_raylib::{color::Color, drawing::RaylibDraw};

use crate::tetray::board::{BOARD_DEPTH, BOARD_WIDTH};
use crate::tetray::game::Game;

// 화면에 그릴 행 수 (보드 하단 기준, 스폰 영역 포함)
const VISIBLE_ROWS: usize = 22;
const CELL_SIZE: i32 = 24;
const BOARD_X: i32 = 280;
const BOARD_Y: i32 = 30;

fn draw_board(d: &mut impl RaylibDraw, game: &Game) {
    let first_row = BOARD_DEPTH - VISIBLE_ROWS;
    for r in first_row..BOARD_DEPTH {
        for c in 0..BOARD_WIDTH {
            let x = BOARD_X + c as i32 * CELL_SIZE;
            let y = BOARD_Y + (r - first_row) as i32 * CELL_SIZE;
            if let Some(color) = game.get_cell_color(r, c) {
                d.draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, color);
            }
            d.draw_rectangle_lines(x, y, CELL_SIZE, CELL_SIZE, Color::LIGHTGRAY);
        }
    }
    d.draw_rectangle_lines(
        BOARD_X - 1,
        BOARD_Y - 1,
        BOARD_WIDTH as i32 * CELL_SIZE + 2,
        VISIBLE_ROWS as i32 * CELL_SIZE + 2,
        Color::DARKGRAY,
    );
}

fn main() {
    let (mut win, thread) = sola_raylib::init()
        .size(800, 600)
        .title("Tetray")
        .build();

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);

    let mut game = Game::new(seed as usize);

    win.set_target_fps(10);

    while !win.window_should_close() {
        game.update();

        let mut d = win.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        draw_board(&mut d, &game);
    }
}
