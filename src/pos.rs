use crate::{
    dot::Dot,
    prelude::{COLS, ROWS},
    util::map_idx,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pos(pub i32, pub i32);

impl Pos {
    pub fn idx(&self) -> usize {
        map_idx(self.0, self.1)
    }

    pub fn add_x(&self, x_offset: i32) -> Pos {
        Pos(self.0 + x_offset, self.1)
    }

    pub fn add_y(&self, y_offset: i32) -> Pos {
        Pos(self.0, self.1 + y_offset)
    }

    pub fn from_tuple((x, y): (i32, i32)) -> Pos {
        Pos(x, y)
    }

    pub fn top_left_px(&self, square_size: i32) -> Pos {
        let Pos(x, y) = self;

        let y = y * square_size;

        let x = (x + 1) * square_size;

        Pos(x, y)
    }

    pub fn add_mut(&mut self, Pos(x, y): Pos) {
        self.0 += x;
        self.1 += y;
    }

    pub fn add(&self, pos: Pos) -> Pos {
        Pos(self.0 + pos.0, self.1 + pos.1)
    }

    pub fn intersects(&self, squares: &[Option<Dot>]) -> bool {
        for square in squares.iter().flatten() {
            if square.tile == *self {
                return true;
            }
        }

        false
    }

    pub fn blocked(&self, squares: &[Option<Dot>], lowest_top: i32) -> bool {
        self.outside_of_grid(lowest_top) || self.intersects(squares)
    }

    pub fn get_lowest_x(&self, squares: &[Option<Dot>]) -> i32 {
        let Pos(x, y) = self;

        let mut x = x - 1;
        let mut offset = 0;

        while x > 0 {
            let idx = map_idx(x, *y);

            if squares[idx].is_some() {
                return offset;
            }

            offset -= 1;
            x -= 1;
        }

        offset
    }

    pub fn outside_of_grid(&self, lowest_top: i32) -> bool {
        if self.0 < 0 || self.1 < lowest_top {
            return true;
        }

        if self.0 >= COLS {
            return true;
        }

        if self.1 >= ROWS {
            return true;
        }

        false
    }
}
