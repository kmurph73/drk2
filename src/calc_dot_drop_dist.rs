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

#[derive(Debug, PartialEq)]
pub struct DroppingDot {
    pub ts: u128,
    pub dist: i32,
    pub pixel_dist: i32,
    pub total_time: f64,
}

impl DroppingDot {
    pub fn get_offset_y(&self, current_ts: u128) -> i32 {
        let delta = (current_ts - self.ts) as f64;
        let mut pct = delta / self.total_time;

        if pct > 1.0 {
            return self.pixel_dist;
        }

        pct = ease_in_sine(pct);

        let y_offset = pct * (self.pixel_dist as f64);

        y_offset as i32
    }
}

pub fn calc_dot_drop_dist(
    squares: &[Option<Dot>],
    pieces: &[Piece],
    current_ts: u128,
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

                    let mut dist: i32 = 1;
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

                    let total_time = DROP_MS * dist;
                    let total_time = total_time as f64;
                    let pixel_dist = dist * SQUARE_SIZE;

                    arr[idx] = Some(DroppingDot {
                        dist,
                        pixel_dist,
                        ts: current_ts,
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

                let total_time = DROP_MS * dist;
                let drop = DroppingDot {
                    dist,
                    pixel_dist: dist * SQUARE_SIZE,
                    ts: current_ts,
                    total_time: total_time as f64,
                };

                arr[idx] = Some(drop);
            }
        }

        y -= 1;
    }

    arr
}

pub fn get_y_offsets(dots: &[Option<DroppingDot>], current_ts: u128) -> (Vec<i32>, bool) {
    let mut finished = true;

    let mut offsets: Vec<i32> = Vec::with_capacity(NUM_SQUARES_USIZE);

    for (idx, dot) in dots.iter().enumerate() {
        if let Some(dot) = dot {
            let offset = dot.get_offset_y(current_ts);

            if offset != dot.dist {
                finished = false
            }

            offsets.push(offset);
        } else {
            offsets.push(0);
        }
    }

    (offsets, finished)
}
