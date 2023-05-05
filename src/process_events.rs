use crate::{cmd::Event, piece::Piece};

pub fn process_events(events: &[Event], piece: &mut Piece) {
    for event in events {
        match event {
            Event::Move(dir) => {
                piece.move_mut(dir);
            }

            Event::Rotate(rotation) => {
                piece.set_rotation(rotation);
            }
        }
    }
}
