use crate::{
    dot::Dot,
    pos::Pos,
    prelude::NUM_SQUARES,
    util::{empty_array, map_idx},
};

fn assign_green(tile: Pos, squares: &mut [Option<Dot>]) {
    let Pos(x, y) = tile;
    let dot = Dot::new_green_bad(tile);
    let idx = map_idx(x, y);

    squares[idx] = Some(dot);
}

fn assign_blue(tile: Pos, squares: &mut [Option<Dot>]) {
    let Pos(x, y) = tile;
    let dot = Dot::new_blue_bad(tile);
    let idx = map_idx(x, y);

    squares[idx] = Some(dot);
}

pub fn test_scenario() -> Vec<Option<Dot>> {
    let mut squares: Vec<Option<Dot>> = empty_array(NUM_SQUARES as usize);

    assign_blue(Pos(0, 6), &mut squares);
    assign_blue(Pos(1, 4), &mut squares);

    assign_blue(Pos(7, 7), &mut squares);
    assign_blue(Pos(6, 7), &mut squares);
    assign_blue(Pos(6, 5), &mut squares);

    assign_blue(Pos(4, 5), &mut squares);
    assign_blue(Pos(4, 6), &mut squares);
    assign_blue(Pos(4, 7), &mut squares);
    assign_blue(Pos(5, 7), &mut squares);

    assign_green(Pos(4, 10), &mut squares);
    assign_green(Pos(4, 11), &mut squares);
    assign_green(Pos(4, 12), &mut squares);

    squares
}

pub fn test_scenario2() -> Vec<Option<Dot>> {
    let mut squares: Vec<Option<Dot>> = empty_array(NUM_SQUARES as usize);

    assign_blue(Pos(7, 13), &mut squares);
    assign_blue(Pos(7, 14), &mut squares);
    assign_blue(Pos(7, 15), &mut squares);

    // assign_blue(Pos(6, 13), &mut squares);
    // assign_blue(Pos(6, 14), &mut squares);
    // assign_blue(Pos(6, 15), &mut squares);

    squares
}

pub fn test_scenario3() -> Vec<Option<Dot>> {
    let mut squares: Vec<Option<Dot>> = empty_array(NUM_SQUARES as usize);

    assign_blue(Pos(6, 15), &mut squares);

    // assign_blue(Pos(6, 13), &mut squares);
    // assign_blue(Pos(6, 14), &mut squares);
    // assign_blue(Pos(6, 15), &mut squares);

    squares
}
