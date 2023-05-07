use crate::{cmd::Event, piece::Piece};

pub fn process_events(events: &[Event], piece: &mut Piece) {
    for event in events {
        match event {
            Event::Move((lhs, rhs)) => {
                piece.set_pos(*lhs, *rhs);
            }

            Event::Rotate(rotation) => {
                piece.set_rotation(rotation);
            }
        }
    }
}
