use crate::blocks::Blocks;
use crate::dot::Dot;
use crate::piece::Piece;
use crate::prelude::{COLS, ROWS};
use crate::util::map_idx;

pub fn calc_dots_to_drop(squares: &[Option<Dot>], pieces: &[Piece]) -> Vec<usize> {
    let mut tiles_to_drop: Vec<usize> = Vec::new();
    let mut handled_pieces: Vec<usize> = Vec::new();
    let mut y = ROWS - 1;

    let mut blocks = Blocks::new(squares);

    while y >= 0 {
        for x in 0..COLS {
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

                if let Some((lower_index, higher_index)) = piece.attempt_to_lower(&blocks) {
                    blocks.set_both_passable(lower_index, higher_index);
                    tiles_to_drop.push(lower_index);
                    tiles_to_drop.push(higher_index);
                    handled_pieces.push(idx);
                }
            } else if dot.can_drop4(&blocks) {
                blocks.set_passable(idx);
                tiles_to_drop.push(idx);
            }
        }

        y -= 1;
    }

    tiles_to_drop
}
