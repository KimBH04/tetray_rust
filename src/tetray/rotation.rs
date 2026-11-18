use crate::tetray::tetromino::{MinoKind, Rotate};

#[rustfmt::skip]
const OFFSETS_DEFAULT: [[(i32, i32); 5]; 4] = [
    [( 0,  0), (-1,  0), (-1,  1), ( 0, -2), (-1, -2)],
    [( 0,  0), ( 0,  0), ( 0,  0), ( 0,  0), ( 0,  0)],
    [( 0,  0), ( 1,  0), ( 1,  1), ( 0, -2), ( 1, -2)],
    [( 0,  0), ( 0,  0), ( 0,  0), ( 0,  0), ( 0,  0)],
];

#[rustfmt::skip]
const OFFSETS_I: [[(i32, i32); 5]; 4] = [
    [( 0, -1), ( 0, -1), ( 0, -1), ( 0,  1), ( 0, -2)],
    [( 0,  0), (-1,  0), ( 2,  0), (-1,  0), ( 2,  0)],
    [(-1,  0), ( 0,  0), ( 0,  0), ( 0, -1), ( 0,  2)],
    [(-1, -1), ( 1, -1), (-2, -1), ( 1,  0), (-2,  0)],
];

#[rustfmt::skip]
const OFFSETS_O: [[(i32, i32); 1]; 4] = [
    [(-1,  0)],
    [( 0,  0)],
    [( 0,  1)],
    [(-1,  1)],
];

pub fn get_kick_positions(kind: MinoKind, from: Rotate, to: Rotate) -> Vec<(i32, i32)> {
    let from_offsets = match kind {
        MinoKind::I => Vec::from_iter(OFFSETS_I[from as usize]),
        MinoKind::O => Vec::from_iter(OFFSETS_O[from as usize]),
        _ => Vec::from_iter(OFFSETS_DEFAULT[from as usize]),
    };
    
    let to_offsets = match kind {
        MinoKind::I => Vec::from_iter(OFFSETS_I[to as usize]),
        MinoKind::O => Vec::from_iter(OFFSETS_O[to as usize]),
        _ => Vec::from_iter(OFFSETS_DEFAULT[to as usize]),
    };

    let mut positions = vec![];
    let cnt = from_offsets.len();
    for i in 0..cnt {
        positions.push((
            from_offsets[i].1 - to_offsets[i].1,
            from_offsets[i].0 - to_offsets[i].0,
        ));
    }

    positions
}