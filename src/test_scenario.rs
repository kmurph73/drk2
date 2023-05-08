use crate::{
    dot::Dot,
    pos::Pos,
    prelude::NUM_SQUARES,
    util::{empty_array, map_idx},
};

fn assign_blue(tile: Pos, squares: &mut [Option<Dot>]) {
    let Pos(x, y) = tile;
    let dot = Dot::new_blue_bad(tile);
    let idx = map_idx(x, y);

    squares[idx] = Some(dot);
}

pub fn test_scenario() -> Vec<Option<Dot>> {
    let mut squares: Vec<Option<Dot>> = empty_array(NUM_SQUARES as usize);

    assign_blue(Pos(0, 4), &mut squares);
    assign_blue(Pos(1, 2), &mut squares);

    assign_blue(Pos(7, 5), &mut squares);
    assign_blue(Pos(6, 5), &mut squares);
    assign_blue(Pos(6, 3), &mut squares);

    assign_blue(Pos(4, 3), &mut squares);
    assign_blue(Pos(4, 4), &mut squares);
    assign_blue(Pos(4, 5), &mut squares);
    assign_blue(Pos(5, 5), &mut squares);

    squares
}
