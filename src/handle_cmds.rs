use crate::{
    cmd::{Cmd, Event},
    dot::Dot,
    piece::Piece,
};

pub fn handle_cmds(cmds: &[Cmd], piece: &Piece, squares: &Vec<Option<Dot>>) -> Vec<Event> {
    let mut events: Vec<Event> = Vec::new();

    for cmd in cmds {
        match cmd {
            Cmd::Move(dir) => {
                if piece.can_move(dir, squares) {
                    events.push(Event::Move(*dir));
                }
            }
            Cmd::Rotate => {
                if piece.can_rotate(squares) {
                    events.push(Event::Rotate)
                }
            }
        }
    }

    events
}
