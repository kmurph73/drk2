use rand::rngs::ThreadRng;

use crate::{cmd::Direction, dot::Dot, pos::Pos};

pub struct Piece {
    pub lhs: Dot,
    pub rhs: Dot,
    pub rotation: i32,
}

fn rotation_offset(n: i32) -> Pos {
    let (x, y) = match n {
        0 => (-1, -1),
        1 => (-1, 1),
        2 => (1, 1),
        3 => (1, -1),
        _ => panic!("{n} should be 0..3"),
    };

    Pos(x, y)
}

fn forced_rotation_offset(n: i32) -> (Pos, Pos) {
    let ((lx, ly), (rx, ry)) = match n {
        0 => ((0, -1), (-1, -1)),
        1 => ((0, 0), (1, -1)),
        2 => ((0, -1), (1, 1)),
        3 => ((-1, 0), (0, -1)),
        _ => panic!("{n} should be 0..3"),
    };

    (Pos(lx, ly), Pos(rx, ry))
}

fn piece_intersects(lhs: &Pos, rhs: &Pos, squares: &[Option<Dot>]) -> bool {
    for square in squares.iter().flatten() {
        if square.tile == *lhs || square.tile == *rhs {
            return true;
        }
    }

    false
}

fn next_rotation(n: i32) -> i32 {
    if n == 3 {
        0
    } else {
        n + 1
    }
}

impl Piece {
    pub fn random(rng: &mut ThreadRng) -> Piece {
        let tile = Pos(7, 2);
        let lhs = Dot::random_good(rng, tile);

        let tile = Pos(7, 3);
        let rhs = Dot::random_good(rng, tile);

        Piece {
            lhs,
            rhs,
            rotation: 3,
        }
    }

    pub fn adjust_mut(&mut self, offset: Pos) {
        self.lhs.tile.add_mut(offset);
        self.rhs.tile.add_mut(offset);
    }

    pub fn can_move(&self, dir: &Direction, squares: &[Option<Dot>]) -> bool {
        let offset = dir.offset();
        let lhs = self.lhs.tile.add(offset);
        let rhs = self.rhs.tile.add(offset);

        if lhs.outside_of_grid() || rhs.outside_of_grid() || piece_intersects(&lhs, &rhs, squares) {
            return false;
        }

        true
    }

    pub fn attempt_rotation(&self, squares: &[Option<Dot>]) -> Option<(Pos, Pos, i32)> {
        let rotation = self.rotation;

        let offset = rotation_offset(rotation);

        let lhs = self.lhs.tile;
        let rhs = self.rhs.tile.add(offset);

        if rhs.blocked(squares) {
            let (left_offset, right_offset) = forced_rotation_offset(rotation);

            let lhs = self.lhs.tile.add(left_offset);
            let rhs = self.rhs.tile.add(right_offset);

            if rhs.blocked(squares) || lhs.blocked(squares) {
                return None;
            } else {
                return Some((lhs, rhs, next_rotation(rotation)));
            }
        }

        Some((lhs, rhs, next_rotation(rotation)))
    }

    pub fn move_mut(&mut self, dir: &Direction) {
        self.adjust_mut(dir.offset());
    }

    pub fn set_rotation(&mut self, (lhs, rhs, rotation): &(Pos, Pos, i32)) {
        self.lhs.tile = *lhs;
        self.rhs.tile = *rhs;
        println!("rot: {rotation}");
        self.rotation = *rotation;
    }
}
