mod tetray;

use sola_raylib::{color::Color, drawing::RaylibDraw};

use crate::tetray::tetromino::{ROTATED_COUNT, Rotate, SIZE};

fn main() {
    let (mut win, thread) = sola_raylib::init()
        .size(800, 600)
        .title("Tetray")
        .build();

    win.set_target_fps(1);
    let mut rotate = 0;
    let rotates = [Rotate::Left, Rotate::Straight, Rotate::Right, Rotate::Half];

    while !win.window_should_close() {
        let mut d = win.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        
        rotate += 1;
        rotate %= ROTATED_COUNT;
        for r in 0..SIZE {
            for c in 0..SIZE {
                use crate::tetray::tetromino::*;

                let x = c as i32 * 40;
                let y = r as i32 * 40;
                let color = |m: &Tetromino| {
                    match m.get_rotated_shape(rotates[rotate]).get_cell(r, c) {
                        Some(true) => m.get_color(),
                        _ => Color::BLACK,
                    }
                };

                d.draw_rectangle(x,       y,       40, 40, color(MINO_J));
                d.draw_rectangle(x + 200, y,       40, 40, color(MINO_L));
                d.draw_rectangle(x + 400, y,       40, 40, color(MINO_S));
                d.draw_rectangle(x + 600, y,       40, 40, color(MINO_T));
                d.draw_rectangle(x,       y + 200, 40, 40, color(MINO_Z));
                d.draw_rectangle(x + 200, y + 200, 40, 40, color(MINO_I));
                d.draw_rectangle(x + 400, y + 200, 40, 40, color(MINO_O));
            }
        }
    }
}
