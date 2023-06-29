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

    pub fn can_lower(&self, squares: &[Option<Dot>]) -> bool {
        let lhs_index = self.lhs.idx();
        let rhs_index = self.rhs.idx();

        self.lhs.can_drop2(squares, rhs_index) && self.rhs.can_drop2(squares, lhs_index)
    }

    pub fn has_idx(&self, idx: usize) -> bool {
        self.lhs.idx() == idx || self.rhs.idx() == idx
    }

    pub fn is_horizontal(&self) -> bool {
        self.rotation == 0 || self.rotation == 2
    }

    pub fn attempt_to_lower(
        &self,
        squares: &[Option<Dot>],
        ignores: &[usize],
        blocks: &[usize],
    ) -> Option<(usize, usize)> {
        let lhs_index = self.lhs.idx();
        let rhs_index = self.rhs.idx();

        if self.lhs.can_lower3(squares, rhs_index, ignores, blocks)
            && self.rhs.can_lower3(squares, lhs_index, ignores, blocks)
        {
            let (lower_index, higher_index) = if self.rhs.lower_than(&self.lhs) {
                (rhs_index, lhs_index)
            } else {
                (lhs_index, rhs_index)
            };

            Some((lower_index, higher_index))
        } else {
            None
        }
    }

    pub fn lower(&self) -> Piece {
        Piece {
            lhs: self.lhs.lower(),
            rhs: self.rhs.lower(),
            rotation: self.rotation,
            landed: self.landed,
        }
    }

    pub fn indexes(&self) -> (usize, usize) {
        (self.lhs.idx(), self.rhs.idx())
    }

    pub fn lower_higher_index(&self) -> (usize, usize) {
        if self.rhs.lower_than(&self.lhs) {
            (self.rhs.idx(), self.lhs.idx())
        } else {
            (self.lhs.idx(), self.rhs.idx())
        }
    }

    pub fn custom() -> Piece {
        let tile = Pos(3, 2);
        let lhs = Dot::green(tile);

        let tile = Pos(4, 2);
        let rhs = Dot::blue(tile);

        Piece {
            lhs,
            rhs,
            rotation: 0,
            landed: false,
        }
    }

    pub fn custom_on_deck() -> Piece {
        let tile = Pos(-1, 0);
        let lhs = Dot::green(tile);

        let tile = Pos(0, 0);
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

    pub fn add(&mut self, x: i32, y: i32) {
        self.lhs.add_mut(x, y);
        self.rhs.add_mut(x, y);
    }

    pub fn move_on_deck(&mut self) -> bool {
        let Pos(x, _y) = self.rhs.tile;

        let (dx, dy) = if x == 4 { (0, 1) } else { (1, 0) };

        self.add(dx, dy);

        dy == 1
    }

    pub fn originate_mut(&mut self) {
        self.lhs.tile.0 = 3;
        self.lhs.tile.1 = 1;

        self.rhs.tile.0 = 4;
        self.rhs.tile.1 = 1;
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

    pub fn random_on_deck(rng: &mut ThreadRng) -> Piece {
        let tile = Pos(-1, 0);
        let lhs = Dot::random_good(rng, tile);

        let tile = Pos(0, 0);
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

    fn lowest_x_offset(&self, squares: &[Option<Dot>]) -> i32 {
        if self.is_horizontal() {
            if self.lhs.tile.0 < self.rhs.tile.0 {
                self.lhs.lowest_x_offset(squares)
            } else {
                self.rhs.lowest_x_offset(squares)
            }
        } else {
            let lhs_lowest = self.lhs.lowest_x_offset(squares);
            let rhs_lowest = self.rhs.lowest_x_offset(squares);

            std::cmp::max(rhs_lowest, lhs_lowest)
        }
    }

    pub fn snap_left(&self, squares: &[Option<Dot>]) -> Option<(Pos, Pos)> {
        let x_offset = self.lowest_x_offset(squares);

        if x_offset == 0 {
            None
        } else {
            Some((self.lhs.tile.add_x(x_offset), self.rhs.tile.add_x(x_offset)))
        }
    }

    pub fn snap_right(&self, squares: &[Option<Dot>]) -> Option<(Pos, Pos)> {
        let x_offset = self.greatest_x_offset(squares);

        if x_offset == 0 {
            None
        } else {
            Some((self.lhs.tile.add_x(x_offset), self.rhs.tile.add_x(x_offset)))
        }
    }

    fn greatest_x_offset(&self, squares: &[Option<Dot>]) -> i32 {
        if self.is_horizontal() {
            if self.lhs.tile.0 > self.rhs.tile.0 {
                self.lhs.greatest_x_offset(squares)
            } else {
                self.rhs.greatest_x_offset(squares)
            }
        } else {
            let lhs_greatest = self.lhs.greatest_x_offset(squares);
            let rhs_greatest = self.rhs.greatest_x_offset(squares);

            std::cmp::min(rhs_greatest, lhs_greatest)
        }
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
