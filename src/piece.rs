use rand::rngs::ThreadRng;

use crate::{dot::Dot, pos::Pos};

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
}
