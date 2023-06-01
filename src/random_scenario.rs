use crate::{
    dot::Dot,
    get_indexes_to_remove::get_indexes_to_remove,
    pos::Pos,
    prelude::{COLS, NUM_SQUARES_USIZE, ROWS},
    util::{empty_array, map_idx},
};
use rand::{rngs::ThreadRng, Rng};

fn random_tile(rng: &mut ThreadRng) -> Pos {
    let row = rng.gen_range(6..ROWS);
    let col = rng.gen_range(0..COLS);

    Pos(col, row)
}

fn assign_dots(rng: &mut ThreadRng, n: usize, squares: &mut [Option<Dot>]) {
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

        if cnt >= n {
            break;
        }
    }
}

pub fn random_scenario(rng: &mut ThreadRng, mut n: usize) -> Vec<Option<Dot>> {
    let mut squares = empty_array(NUM_SQUARES_USIZE);

    loop {
        assign_dots(rng, n, &mut squares);

        let indexes = get_indexes_to_remove(&squares);

        if indexes.is_empty() {
            break;
        }

        for i in &indexes {
            squares[*i] = None;
        }

        n -= indexes.len();
    }

    squares
}
