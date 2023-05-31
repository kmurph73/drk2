use rand::rngs::ThreadRng;

use crate::{cmd::Direction, dot::Dot, pos::Pos};

#[derive(Clone)]
pub struct Piece {
    pub lhs: Dot,
    pub rhs: Dot,
    pub rotation: i32,
    pub landed: bool,
}

pub struct Rotation {
    left_offset: Pos,
    right_offset: Pos,
    next_rotation: i32,
}

fn attempt_rotation(rotation: i32, attempt: usize) -> Rotation {
    let is_even = rotation % 2 == 0;

    let (lhs, rhs) = match attempt {
        0 => match is_even {
            true => ((0, 0), (-1, -1)),
            false => ((1, 0), (0, 1)),
        },
        1 => match is_even {
            true => ((0, 1), (-1, 0)),
            false => ((0, 0), (-1, 1)),
        },
        2 => match is_even {
            true => ((1, 0), (0, -1)),
            false => ((0, -1), (-1, 0)),
        },
        _ => panic!("{attempt} should be 0..2"),
    };

    println!("attempt: {attempt}; {is_even}");

    let (lhs, rhs) = if rotation > 1 { (rhs, lhs) } else { (lhs, rhs) };

    Rotation {
        left_offset: Pos::from_tuple(lhs),
        right_offset: Pos::from_tuple(rhs),
        next_rotation: get_next_rotation(rotation),
    }
}

fn piece_intersects(lhs: &Pos, rhs: &Pos, squares: &[Option<Dot>]) -> bool {
    for square in squares.iter().flatten() {
        if square.tile == *lhs || square.tile == *rhs {
            return true;
        }
    }

    false
}

fn get_next_rotation(n: i32) -> i32 {
    if n == 3 {
        0
    } else {
        n + 1
    }
}

impl Piece {
    pub fn lower_mut(&mut self) {
        self.lhs.tile.1 += 1;
        self.rhs.tile.1 += 1;
    }

    pub fn has_idx(&self, idx: usize) -> bool {
        self.lhs.idx() == idx || self.rhs.idx() == idx
    }

    pub fn is_horizontal(&self) -> bool {
        self.rotation == 0 || self.rotation == 2
    }

    pub fn attempt_drop(&self, squares: &[Option<Dot>]) -> Option<(usize, usize)> {
        let lhs_index = self.lhs.idx();
        let rhs_index = self.rhs.idx();

        if self.lhs.can_drop2(squares, rhs_index) && self.rhs.can_drop2(squares, lhs_index) {
            Some((lhs_index, rhs_index))
        } else {
            None
        }
    }

    pub fn custom() -> Piece {
        let tile = Pos(3, 10);
        let lhs = Dot::green(tile);

        let tile = Pos(4, 10);
        let rhs = Dot::blue(tile);

        Piece {
            lhs,
            rhs,
            rotation: 0,
            landed: false,
        }
    }

    pub fn has_tile(&self, tile: &Pos) -> bool {
        self.lhs.tile == *tile || self.rhs.tile == *tile
    }

    pub fn random(rng: &mut ThreadRng) -> Piece {
        let tile = Pos(3, 1);
        let lhs = Dot::random_good(rng, tile);

        let tile = Pos(4, 1);
        let rhs = Dot::random_good(rng, tile);

        Piece {
            lhs,
            rhs,
            rotation: 0,
            landed: false,
        }
    }

    fn lowest_top(&self) -> i32 {
        if self.rhs.tile.1 < 2 || self.lhs.tile.1 < 2 {
            1
        } else {
            2
        }
    }

    fn lowest_y_offset(&self, squares: &[Option<Dot>]) -> i32 {
        let lh_offset = self.lhs.lowest_y_offset(squares);
        let rh_offset = self.rhs.lowest_y_offset(squares);

        std::cmp::min(lh_offset, rh_offset)
    }

    pub fn find_lowest_drop(&self, squares: &[Option<Dot>]) -> (Pos, Pos) {
        let y_offset = self.lowest_y_offset(squares);

        let lhs = self.lhs.tile.add_y(y_offset);
        let rhs = self.rhs.tile.add_y(y_offset);

        (lhs, rhs)
    }

    pub fn attempt_move(&self, dir: &Direction, squares: &[Option<Dot>]) -> Option<(Pos, Pos)> {
        let offset = dir.offset();
        let lhs = self.lhs.tile.add(offset);
        let rhs = self.rhs.tile.add(offset);

        let lowest_top = if dir.is_horizontal() {
            0
        } else {
            self.lowest_top()
        };

        if lhs.outside_of_grid(lowest_top)
            || rhs.outside_of_grid(lowest_top)
            || piece_intersects(&lhs, &rhs, squares)
        {
            return None;
        }

        Some((lhs, rhs))
    }

    pub fn attempt_rotation(&self, squares: &[Option<Dot>]) -> Option<(Pos, Pos, i32)> {
        let mut attempt = 0;
        let rotation = self.rotation;

        while attempt < 3 {
            let Rotation {
                left_offset,
                right_offset,
                next_rotation,
            } = attempt_rotation(rotation, attempt);

            let lhs = self.lhs.tile.add(left_offset);
            let rhs = self.rhs.tile.add(right_offset);

            if rhs.blocked(squares, -2) || lhs.blocked(squares, -2) {
                attempt += 1;
                continue;
            } else {
                return Some((lhs, rhs, next_rotation));
            }
        }

        None
    }

    pub fn set_pos(&mut self, lhs: Pos, rhs: Pos) {
        self.lhs.tile = lhs;
        self.rhs.tile = rhs;
    }

    pub fn set_rotation(&mut self, (lhs, rhs, rotation): &(Pos, Pos, i32)) {
        self.lhs.tile = *lhs;
        self.rhs.tile = *rhs;
        self.rotation = *rotation;
    }
}
