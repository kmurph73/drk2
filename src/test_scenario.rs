use crate::{
    dot::Dot,
    pos::Pos,
    prelude::NUM_SQUARES,
    util::{empty_array, map_idx},
};

fn assign(tile: Pos, squares: &mut Vec<Option<Dot>>) {
    let Pos(x, y) = tile;
    let dot = Dot::new_blue_bad(tile);
    let idx = map_idx(x, y);

    squares[idx] = Some(dot);
}

pub fn test_scenario() -> Vec<Option<Dot>> {
    let mut squares: Vec<Option<Dot>> = empty_array(NUM_SQUARES as usize);

    assign(Pos(0, 4), &mut squares);
    assign(Pos(1, 2), &mut squares);

    assign(Pos(7, 5), &mut squares);
    assign(Pos(6, 3), &mut squares);

    squares
}
