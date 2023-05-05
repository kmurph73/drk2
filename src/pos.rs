use crate::{
    dot::Dot,
    prelude::{COLS, ROWS},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pos(pub i32, pub i32);

impl Pos {
    pub fn top_left_px(&self, square_size: i32) -> Pos {
        let Pos(x, y) = self;

        let y = (y + 2) * square_size;

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

    pub fn blocked(&self, squares: &[Option<Dot>]) -> bool {
        self.outside_of_grid() || self.intersects(squares)
    }

    pub fn outside_of_grid(&self) -> bool {
        if self.0 < 0 || self.1 < 0 {
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
