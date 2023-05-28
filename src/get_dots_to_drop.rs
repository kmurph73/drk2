use crate::dot::Dot;
use crate::piece::Piece;
use crate::prelude::{COLS, ROWS};
use crate::util::map_idx;

pub fn calc_dots_to_drop(squares: &[Option<Dot>], pieces: &[Piece]) -> Vec<usize> {
    let mut tiles_to_drop: Vec<usize> = Vec::new();
    let mut handled_pieces: Vec<usize> = Vec::new();

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

            if let Some((idx, piece)) = pieces
                .iter()
                .enumerate()
                .find(|(_idx, piece)| piece.has_tile(tile))
            {
                if handled_pieces.contains(&idx) {
                    continue;
                }

                if let Some((lhs_index, rhs_index)) = piece.attempt_drop(squares) {
                    tiles_to_drop.push(lhs_index);
                    tiles_to_drop.push(rhs_index);
                    handled_pieces.push(idx);
                }
            } else if dot.can_drop(squares) {
                tiles_to_drop.push(idx);
            }
        }
    }

    tiles_to_drop
}