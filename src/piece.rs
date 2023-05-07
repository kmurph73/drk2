use rand::rngs::ThreadRng;

use crate::{cmd::Direction, dot::Dot, pos::Pos};

pub struct Piece {
    pub lhs: Dot,
    pub rhs: Dot,
    pub rotation: i32,
}

pub struct Rotation {
    left_offset: Pos,
    right_offset: Pos,
    next_rotation: i32,
}

fn attempt_rotation(rotation: i32, attempt: usize) -> Rotation {
    let (lhs, rhs, next_rotation) = match attempt {
        0 => {
            let (lhs, rhs) = match rotation {
                0 => ((0, 0), (-1, -1)),
                1 => ((1, 0), (0, 1)),
                2 => ((-1, -1), (0, 0)),
                3 => ((0, 1), (1, 0)),
                _ => panic!("{rotation} should be 0..3"),
            };

            (lhs, rhs, get_next_rotation(rotation))
        }
        1 => {
            let (lhs, rhs) = match rotation {
                0 => ((1, 0), (0, -1)),
                1 => ((0, 0), (-1, 1)),
                2 => ((0, -1), (1, 0)),
                3 => ((-1, 1), (0, 0)),
                _ => panic!("{rotation} should be 0..3"),
            };

            (lhs, rhs, get_next_rotation(rotation))
        }
        _ => panic!("{attempt} should be 0..1"),
    };

    Rotation {
        left_offset: Pos::from_tuple(lhs),
        right_offset: Pos::from_tuple(rhs),
        next_rotation,
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
    pub fn custom() -> Piece {
        let tile = Pos(0, 0);
        let lhs = Dot::green(tile);

        let tile = Pos(1, 0);
        let rhs = Dot::blue(tile);

        Piece {
            lhs,
            rhs,
            rotation: 0,
        }
    }

    pub fn random(rng: &mut ThreadRng) -> Piece {
        let tile = Pos(0, 4);
        let lhs = Dot::random_good(rng, tile);

        let tile = Pos(0, 3);
        let rhs = Dot::random_good(rng, tile);

        Piece {
            lhs,
            rhs,
            rotation: 1,
        }
    }

    fn lowest_top(&self) -> i32 {
        if self.rhs.tile.1 < 0 || self.lhs.tile.1 < 0 {
            -1
        } else {
            0
        }
    }

    pub fn attempt_move(&self, dir: &Direction, squares: &[Option<Dot>]) -> Option<(Pos, Pos)> {
        let offset = dir.offset();
        let lhs = self.lhs.tile.add(offset);
        let rhs = self.rhs.tile.add(offset);

        let lowest_top = self.lowest_top();

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

        while attempt < 2 {
            let Rotation {
                left_offset,
                right_offset,
                next_rotation,
            } = attempt_rotation(rotation, attempt);

            let lhs = self.lhs.tile.add(left_offset);
            let rhs = self.rhs.tile.add(right_offset);

            if rhs.blocked(squares, -1) || lhs.blocked(squares, -1) {
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
