mod tetray;

use sola_raylib::{color::Color, consts::KeyboardKey, drawing::RaylibDraw, RaylibHandle};

use crate::tetray::board::{BOARD_DEPTH, BOARD_WIDTH};
use crate::tetray::game::GameView;
use crate::tetray::runner::TetrayRunner;
use crate::tetray::tetromino::{MINO_SIZE, MinoKind, Rotate};

// 화면에 그릴 행 수 (보드 하단 기준, 스폰 영역 포함)
const VISIBLE_ROWS: usize = 22;
const CELL_SIZE: i32 = 24;
const BOARD_X: i32 = 280;
const BOARD_Y: i32 = 30;

// 미리보기(홀드/넥스트) 셀 크기
const PREVIEW_CELL: i32 = 16;
const PREVIEW_BOX: i32 = PREVIEW_CELL * MINO_SIZE as i32;
const HOLD_X: i32 = BOARD_X - PREVIEW_BOX - 40;
const NEXT_X: i32 = BOARD_X + BOARD_WIDTH as i32 * CELL_SIZE + 40;
const LABEL_SIZE: i32 = 20;
// 점수 패널 x 좌표 (HIGH SCORE 문구가 보드와 겹치지 않도록 왼쪽 여백 확보)
const SCORE_X: i32 = 30;

const SCREEN_W: i32 = 800;
const SCREEN_H: i32 = 600;
const GAME_OVER_TEXT: &str = "GAME OVER!";
const GAME_OVER_SIZE: i32 = 30;
const GAME_OVER_MARGIN: i32 = 20;

fn handle_input(win: &RaylibHandle, runner: &mut TetrayRunner) {
    if win.is_key_pressed(KeyboardKey::KEY_LEFT) || win.is_key_pressed_repeat(KeyboardKey::KEY_LEFT) {
        runner.try_move(-1);
    }
    if win.is_key_pressed(KeyboardKey::KEY_RIGHT) || win.is_key_pressed_repeat(KeyboardKey::KEY_RIGHT) {
        runner.try_move(1);
    }
    if win.is_key_pressed(KeyboardKey::KEY_UP) || win.is_key_pressed(KeyboardKey::KEY_X) {
        runner.try_relative_rotation(Rotate::Right);
    }
    if win.is_key_pressed(KeyboardKey::KEY_Z) {
        runner.try_relative_rotation(Rotate::Left);
    }
    if win.is_key_pressed(KeyboardKey::KEY_A) {
        runner.try_relative_rotation(Rotate::Half);
    }
    if win.is_key_pressed(KeyboardKey::KEY_DOWN) || win.is_key_pressed_repeat(KeyboardKey::KEY_DOWN) {
        runner.soft_drop();
    }
    if win.is_key_pressed(KeyboardKey::KEY_SPACE) {
        runner.hard_drop();
    }
    if win.is_key_pressed(KeyboardKey::KEY_C) || win.is_key_pressed(KeyboardKey::KEY_LEFT_SHIFT) {
        runner.hold();
    }
}

fn draw_board(d: &mut impl RaylibDraw, view: &GameView) {
    let first_row = BOARD_DEPTH - VISIBLE_ROWS;
    for r in first_row..BOARD_DEPTH {
        for c in 0..BOARD_WIDTH {
            let x = BOARD_X + c as i32 * CELL_SIZE;
            let y = BOARD_Y + (r - first_row) as i32 * CELL_SIZE;
            if let Some(kind) = view.board.get_cell(r, c) {
                if kind != MinoKind::None {
                    d.draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, kind.get_color());
                }
            }
            d.draw_rectangle_lines(x, y, CELL_SIZE, CELL_SIZE, Color::LIGHTGRAY);
        }
    }
    // 위에서 두 번째 칸 아래 경계선
    d.draw_rectangle(
        BOARD_X,
        BOARD_Y + 2 * CELL_SIZE - 1,
        BOARD_WIDTH as i32 * CELL_SIZE,
        2,
        Color::RED,
    );
    d.draw_rectangle_lines(
        BOARD_X - 1,
        BOARD_Y - 1,
        BOARD_WIDTH as i32 * CELL_SIZE + 2,
        VISIBLE_ROWS as i32 * CELL_SIZE + 2,
        Color::DARKGRAY,
    );
}

fn draw_mino_preview(d: &mut impl RaylibDraw, kind: MinoKind, x: i32, y: i32) {
    d.draw_rectangle_lines(x, y, PREVIEW_BOX, PREVIEW_BOX, Color::LIGHTGRAY);
    if kind == MinoKind::None {
        return;
    }

    let mino = kind.get_mino();
    let shape = mino.get_rotated_shape(Rotate::Straight);
    for r in 0..MINO_SIZE {
        for c in 0..MINO_SIZE {
            if shape.get_cell(r, c) {
                d.draw_rectangle(
                    x + c as i32 * PREVIEW_CELL,
                    y + r as i32 * PREVIEW_CELL,
                    PREVIEW_CELL,
                    PREVIEW_CELL,
                    mino.get_color(),
                );
            }
        }
    }
}

fn draw_side_panels(d: &mut impl RaylibDraw, view: &GameView) {
    d.draw_text("HOLD", HOLD_X, BOARD_Y, LABEL_SIZE, Color::DARKGRAY);
    draw_mino_preview(d, view.hold, HOLD_X, BOARD_Y + LABEL_SIZE + 8);

    // 홀드 박스 아래에 점수, 최고 점수 순서로 표시
    let score_y = BOARD_Y + LABEL_SIZE + 8 + PREVIEW_BOX + 30;
    d.draw_text("SCORE", SCORE_X, score_y, LABEL_SIZE, Color::DARKGRAY);
    d.draw_text(&view.score.to_string(), SCORE_X, score_y + LABEL_SIZE + 4, LABEL_SIZE, Color::BLACK);

    let high_y = score_y + (LABEL_SIZE + 4) * 2 + 16;
    d.draw_text("HIGH SCORE", SCORE_X, high_y, LABEL_SIZE, Color::DARKGRAY);
    d.draw_text(&view.highscore.to_string(), SCORE_X, high_y + LABEL_SIZE + 4, LABEL_SIZE, Color::BLACK);

    d.draw_text("NEXT", NEXT_X, BOARD_Y, LABEL_SIZE, Color::DARKGRAY);
    for (i, kind) in view.nexts.iter().enumerate() {
        let y = BOARD_Y + LABEL_SIZE + 8 + i as i32 * (PREVIEW_BOX + 8);
        draw_mino_preview(d, *kind, NEXT_X, y);
    }
}

fn draw_game_over(d: &mut impl RaylibDraw, text_width: i32) {
    let y = SCREEN_H - GAME_OVER_SIZE - GAME_OVER_MARGIN;
    d.draw_text(GAME_OVER_TEXT, GAME_OVER_MARGIN, y, GAME_OVER_SIZE, Color::RED);
    d.draw_text(
        GAME_OVER_TEXT,
        SCREEN_W - text_width - GAME_OVER_MARGIN,
        y,
        GAME_OVER_SIZE,
        Color::RED,
    );
}

fn main() {
    let (mut win, thread) = sola_raylib::init()
        .size(SCREEN_W, SCREEN_H)
        .title("Tetray")
        .build();

    let game_over_width = win.measure_text(GAME_OVER_TEXT, GAME_OVER_SIZE);

    let mut runner = TetrayRunner::new();
    runner.run();

    while !win.window_should_close() {
        handle_input(&win, &mut runner);

        let view = runner.get_game_state();

        let mut d = win.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        draw_board(&mut d, &view);
        draw_side_panels(&mut d, &view);
        d.draw_fps(10, 10);
        if view.ended {
            draw_game_over(&mut d, game_over_width);
        }
    }

    runner.stop();
}
