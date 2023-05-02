use crate::{
    dot::{Dot, DotColor},
    pos::Pos,
    prelude::{COLS, NUM_SQUARES, ROWS},
    util::map_idx,
};
use rand::{rngs::ThreadRng, Rng};

pub fn number_to_color(n: i32) -> DotColor {
    match n {
        0 => DotColor::Red,
        1 => DotColor::Green,
        2 => DotColor::Blue,
        3 => DotColor::Yellow,
        4 => DotColor::Orange,
        _ => panic!("number {n} doesnt correspond to a DotColor"),
    }
}

fn random_tile(rng: &mut ThreadRng) -> Pos {
    let row = rng.gen_range(3..ROWS);
    let col = rng.gen_range(0..COLS);

    Pos(col, row)
}

// https://www.reddit.com/r/rust/comments/qjh00f/comment/hiqrmc9/?utm_source=share&utm_medium=web2x&context=3
const NONE: Option<Dot> = None;
pub fn random_scenario() -> Vec<Option<Dot>> {
    let mut squares: Vec<Option<Dot>> = Vec::from([NONE; NUM_SQUARES as usize]);

    let mut rng = rand::thread_rng();

    let mut cnt = 0;

    loop {
        let tile = random_tile(&mut rng);
        let Pos(x, y) = tile;

        let idx = map_idx(x, y);
        if squares[idx].is_some() {
            continue;
        }

        let color = rng.gen_range(0..5);
        let color = number_to_color(color);

        let bad_dot = Dot::bad(tile, color);

        println!("{bad_dot:#?}");

        squares[idx] = Some(bad_dot);

        cnt += 1;

        if cnt > 3 {
            break;
        }
    }

    squares
}
