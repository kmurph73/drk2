use crate::dot::Dot;
use crate::piece::Piece;
use crate::prelude::{COLS, ROWS};
use crate::util::map_idx;

pub fn calc_dots_to_drop(squares: &[Option<Dot>], pieces: &Vec<Piece>) -> Vec<usize> {
    let tiles_to_drop: Vec<usize> = Vec::new();
    for x in 0..COLS {
        for y in 0..ROWS {
            let idx = map_idx(x, y);

            let dot = if let Some(dot) = &squares[idx] {
                if dot.is_good() {
                    dot
                } else {
                    continue;
                }
            } else {
                continue;
            };

            let tile = &dot.tile;

            if let Some(piece) = pieces.iter().find(|p| p.has_tile(tile)) {}
        }
    }

    dots_to_drop
}
