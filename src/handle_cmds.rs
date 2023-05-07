use crate::{
    cmd::{Cmd, Event},
    dot::Dot,
    piece::Piece,
};

pub fn handle_cmds(cmds: &[Cmd], piece: &Piece, squares: &[Option<Dot>]) -> Vec<Event> {
    let mut events: Vec<Event> = Vec::new();

    for cmd in cmds {
        match cmd {
            Cmd::Move(dir) => {
                if let Some(new_pos) = piece.attempt_move(dir, squares) {
                    events.push(Event::Move(new_pos));
                }
            }
            Cmd::Rotate => {
                if let Some(rotation) = piece.attempt_rotation(squares) {
                    events.push(Event::Rotate(rotation))
                }
            }
        }
    }

    events
}
