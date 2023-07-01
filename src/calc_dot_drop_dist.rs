use crate::{
    blocks::Blocks,
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

#[derive(Debug, PartialEq, Clone)]
pub struct DroppingDot {
    pub ts: u128,
    pub dist: i32,
    pub dist_px: i32,
    pub total_time: f64,
}

impl DroppingDot {
    pub fn get_offset_y(&self, current_ts: u128) -> i32 {
        let delta = (current_ts - self.ts) as f64;
        let mut pct = delta / self.total_time;

        if pct > 1.0 {
            return self.dist_px;
        }

        pct = ease_in_sine(pct);

        let y_offset = pct * (self.dist_px as f64);

        y_offset as i32
    }
}

pub fn move_squares(
    squares: &mut [Option<Dot>],
    dropping_dots: &[Option<DroppingDot>],
    pieces: &mut [Piece],
) {
    let mut y = ROWS - 1;

    let mut handled_pieces: Vec<usize> = Vec::new();
    while y >= 0 {
        for x in 0..COLS {
            let i = map_idx(x, y);
            if let Some(dd) = &dropping_dots[i] {
                let indexes = if let Some(dot) = &mut squares[i] {
                    let idx = dot.idx();
                    dot.add_mut(0, dd.dist);
                    if let Some((i, piece)) =
                        pieces.iter_mut().enumerate().find(|(_i, p)| p.has_idx(idx))
                    {
                        if !handled_pieces.contains(&i) {
                            piece.add_mut(0, dd.dist);
                            handled_pieces.push(i);
                        }
                    }
                    Some((idx, dot.idx()))
                } else {
                    None
                };

                if let Some((a, b)) = indexes {
                    squares.swap(a, b);
                }
            }
        }

        y -= 1;
    }
}

pub fn calc_dot_drop_dist(
    squares: &[Option<Dot>],
    pieces: &[Piece],
    current_ts: u128,
) -> Vec<Option<DroppingDot>> {
    let mut arr: Vec<Option<DroppingDot>> = empty_array(NUM_SQUARES_USIZE);
    let mut handled_pieces: Vec<usize> = Vec::new();

    let mut blocks = Blocks::new(squares);

    let mut y = ROWS - 1;

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

                if let Some((lower_index, higher_index)) = &piece.attempt_to_lower(&blocks) {
                    blocks.set_both_passable(*lower_index, *higher_index);

                    let mut dist: i32 = 1;
                    let mut next_piece = piece.lower();

                    loop {
                        if next_piece.attempt_to_lower(&blocks).is_some() {
                            dist += 1;
                            next_piece.lower_mut();
                        } else {
                            let (lower_index, higher_index) = next_piece.lower_higher_index();
                            blocks.set_both_block(lower_index, higher_index);

                            let total_time = DROP_MS * dist;
                            let total_time = total_time as f64;
                            let pixel_dist = dist * SQUARE_SIZE;

                            let (lhs, rhs) = piece.indexes();

                            let dot = DroppingDot {
                                dist,
                                dist_px: pixel_dist,
                                ts: current_ts,
                                total_time,
                            };

                            arr[lhs] = Some(dot.clone());
                            arr[rhs] = Some(dot);

                            break;
                        }
                    }
                }

                handled_pieces.push(idx);
            } else if dot.can_drop4(&blocks) {
                let mut dist = 1;
                blocks.set_passable(idx);

                let mut dot = dot.lower();

                loop {
                    if dot.can_drop4(&blocks) {
                        dist += 1;
                        dot = dot.lower();
                    } else {
                        blocks.set_blocks(dot.idx());
                        break;
                    }
                }

                let total_time = DROP_MS * dist;
                let drop = DroppingDot {
                    dist,
                    dist_px: dist * SQUARE_SIZE,
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

    for dot in dots.iter() {
        if let Some(dot) = dot {
            let offset_px = dot.get_offset_y(current_ts);

            if offset_px < dot.dist_px {
                finished = false
            }

            offsets.push(offset_px);
        } else {
            offsets.push(0);
        }
    }

    (offsets, finished)
}
