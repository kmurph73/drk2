use crate::{
    dot::Dot,
    easings::ease_in_sine,
    piece::Piece,
    prelude::{COLS, DROP_MS, NUM_SQUARES_USIZE, ROWS, SQUARE_SIZE},
    util::{empty_array, map_idx},
};

pub enum DropKind {
    Piece,
    Dot,
}

pub struct DroppingDot {
    pub ts: u128,
    pub dist: i32,
    pub total_time: f64,
}

impl DroppingDot {
    pub fn get_offset_y(&self, current_ts: u128) -> (i32, bool) {
        let delta = (current_ts - self.ts) as f64;
        let mut pct = delta / self.total_time;
        let pixel_dist = self.dist * SQUARE_SIZE as f64;

        if pct > 1.0 {
            pi
        }

        let mult = ease_in_sine(pct);

        pi
    }
}

pub fn calc_dot_drop_dist(
    squares: &[Option<Dot>],
    pieces: &[Piece],
    ts: u128,
) -> Vec<Option<DroppingDot>> {
    let mut arr: Vec<Option<DroppingDot>> = empty_array(NUM_SQUARES_USIZE);
    let mut handled_pieces: Vec<usize> = Vec::new();
    let mut y = ROWS - 1;

    let mut ignores: Vec<usize> = Vec::new();
    let mut blocks: Vec<usize> = Vec::new();

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

                if let Some((piece, lower_index, higher_index)) =
                    piece.attempt_drop(squares, &ignores, &blocks)
                {
                    ignores.push(lower_index);
                    ignores.push(higher_index);

                    let mut dist = 1;
                    let mut next_piece = piece;

                    loop {
                        if let Some((piece, _lower_index, _higher_index)) =
                            next_piece.attempt_drop(squares, &ignores, &blocks)
                        {
                            dist += 1;

                            next_piece = piece;
                        } else {
                            let (lower_index, higher_index) = next_piece.lower_higher_index();
                            blocks.push(lower_index);
                            blocks.push(higher_index);
                            break;
                        }
                    }

                    let total_time = DROP_MS * dist as u128;
                    arr[idx] = Some(DroppingDot {
                        dist,
                        ts,
                        total_time,
                    });
                }

                handled_pieces.push(idx);
            } else if dot.can_drop4(squares, &ignores, &blocks) {
                let mut dist = 1;
                ignores.push(idx);

                let mut dot = dot.lower();

                loop {
                    if dot.can_drop4(squares, &ignores, &blocks) {
                        dist += 1;
                        dot = dot.lower();
                    } else {
                        let d = dot.tile.add_y(dist);
                        blocks.push(d.idx());
                        break;
                    }
                }

                let total_time = DROP_MS * dist as u128;
                let drop = DroppingDot {
                    dist,
                    ts,
                    total_time,
                };

                arr[idx] = Some(drop);
            }
        }

        y -= 1;
    }

    arr
}
