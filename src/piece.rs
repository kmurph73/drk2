use rand::rngs::ThreadRng;

use crate::{
    cmd::{Direction, Event},
    dot::Dot,
    pos::Pos,
};

pub struct Piece {
    pub lhs: Dot,
    pub rhs: Dot,
    pub rotation: i32,
}

fn rotation_offset(n: i32) -> Pos {
    let (x, y) = match n {
        0 => (1, -1),
        1 => (-1, -1),
        2 => (-1, 1),
        3 => (1, 1),
        _ => panic!("{n} should be 0..3"),
    };

    Pos(x, y)
}

fn piece_intersects(lhs: &Pos, rhs: &Pos, squares: &Vec<Option<Dot>>) -> bool {
    for square in squares {
        if let Some(square) = square {
            if square.tile == *lhs || square.tile == *rhs {
                return true;
            }
        }
    }

    false
}

impl Piece {
    pub fn random(rng: &mut ThreadRng) -> Piece {
        let lhs = Dot::random_good(rng, Pos(0, 0));
        let rhs = Dot::random_good(rng, Pos(1, 0));

        Piece {
            lhs,
            rhs,
            rotation: 0,
        }
    }

    pub fn adjust_mut(&mut self, offset: Pos) {
        self.lhs.tile.add_mut(offset);
        self.rhs.tile.add_mut(offset);
    }

    pub fn can_move(&self, dir: &Direction, squares: &Vec<Option<Dot>>) -> bool {
        let offset = dir.offset();
        let lhs = self.lhs.tile.add(offset);
        let rhs = self.rhs.tile.add(offset);

        if lhs.outside_of_grid() || rhs.outside_of_grid() || piece_intersects(&lhs, &rhs, squares) {
            return false;
        }

        true
    }

    pub fn can_rotate(&self, squares: &Vec<Option<Dot>>) -> bool {
        let rotation = if self.rotation == 3 {
            0
        } else {
            self.rotation + 1
        };

        let offset = rotation_offset(self.rotation);

        let rhs = self.rhs.tile.add(offset);

        if rhs.outside_of_grid() || rhs.intersects(squares) {
            return false;
        }

        true
    }

    pub fn move_mut(&mut self, dir: &Direction) {
        self.adjust_mut(dir.offset());
    }

    pub fn rotate_mut(&mut self) {
        if self.rotation == 3 {
            self.rotation = 0;
        } else {
            self.rotation += 1;
        }

        let offset = rotation_offset(self.rotation);

        self.rhs.tile.add_mut(offset);
    }
}
