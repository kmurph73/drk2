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
    if attempt > 2 {
        println!("attempt: {attempt}, rot: {rotation}");
    }

    let (lhs, rhs, next_rotation) = match attempt {
        0 => {
            let lhs = (0, 0);

            let rhs = match rotation {
                0 => (-1, -1),
                1 => (-1, 1),
                2 => (1, 1),
                3 => (1, -1),
                _ => panic!("{rotation} should be 0..3"),
            };

            (lhs, rhs, get_next_rotation(rotation))
        }
        1 => {
            let (lhs, rhs) = match rotation {
                0 => ((0, -1), (-1, -1)),
                1 => ((1, 0), (0, 1)),
                2 => ((0, -1), (1, 1)),
                3 => ((-1, 0), (0, -1)),
                _ => panic!("{rotation} should be 0..3"),
            };

            (lhs, rhs, get_next_rotation(rotation))
        }
        2 => match rotation {
            0 => ((0, 1), (-1, 0), 1),
            1 => ((1, 0), (0, 1), 2),
            2 => ((-1, 0), (0, 1), 3),
            3 => ((-1, 1), (0, 0), 0),
            _ => panic!("{rotation} should be 0..3"),
        },
        3 => match rotation {
            0 => ((1, 0), (0, -1), 1),
            1 => ((0, -1), (-1, 0), 2),
            2 => ((0, -1), (1, 0), 3),
            3 => ((0, 1), (1, 0), 0),
            _ => panic!("{rotation} should be 0..3"),
        },
        4 => match rotation {
            0 => ((0, 0), (0, 0), rotation),
            1 => ((0, 0), (0, 0), rotation),
            2 => ((-1, -1), (0, 0), 3),
            3 => ((0, 0), (0, 0), rotation),
            _ => panic!("{rotation} should be 0..3"),
        },
        _ => panic!("{attempt} should be 0..4"),
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
        let mut attempt = 0;
        let rotation = self.rotation;

        while attempt < 5 {
            let Rotation {
                left_offset,
                right_offset,
                next_rotation,
            } = attempt_rotation(rotation, attempt);

            let lhs = self.lhs.tile.add(left_offset);
            let rhs = self.rhs.tile.add(right_offset);

            if rhs.blocked(squares) || lhs.blocked(squares) {
                attempt += 1;
                continue;
            } else {
                return Some((lhs, rhs, next_rotation));
            }
        }

        None
    }

    pub fn move_mut(&mut self, dir: &Direction) {
        self.adjust_mut(dir.offset());
    }

    pub fn set_rotation(&mut self, (lhs, rhs, rotation): &(Pos, Pos, i32)) {
        self.lhs.tile = *lhs;
        self.rhs.tile = *rhs;
        self.rotation = *rotation;
    }
}
