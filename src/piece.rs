use rand::rngs::ThreadRng;

use crate::{cmd::Direction, dot::Dot, pos::Pos};

pub struct Piece {
    pub lhs: Dot,
    pub rhs: Dot,
    pub rotation: i32,
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

    pub fn move_mut(&mut self, dir: &Direction) {
        self.adjust_mut(dir.offset());
    }

    pub fn rotate_mut(&mut self) {
        if self.rotation == 3 {
            self.rotation = 0;
        }

        self.rotation += 1;
    }
}
