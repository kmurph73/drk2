use crate::{
    dot::Dot,
    pos::Pos,
    prelude::{COLS, NUM_SQUARES, ROWS},
    util::map_idx,
};
use rand::{rngs::ThreadRng, Rng};

fn random_tile(rng: &mut ThreadRng) -> Pos {
    let row = rng.gen_range(3..ROWS);
    let col = rng.gen_range(0..COLS);

    Pos(col, row)
}

// https://www.reddit.com/r/rust/comments/qjh00f/comment/hiqrmc9/?utm_source=share&utm_medium=web2x&context=3
const NONE: Option<Dot> = None;
pub fn random_scenario(rng: &mut ThreadRng) -> Vec<Option<Dot>> {
    let mut squares: Vec<Option<Dot>> = Vec::from([NONE; NUM_SQUARES as usize]);

    let mut cnt = 0;

    loop {
        let tile = random_tile(rng);
        let Pos(x, y) = tile;

        let idx = map_idx(x, y);
        if squares[idx].is_some() {
            continue;
        }

        let bad_dot = Dot::random_bad(rng, tile);

        squares[idx] = Some(bad_dot);

        cnt += 1;

        if cnt > 3 {
            break;
        }
    }

    squares
}
