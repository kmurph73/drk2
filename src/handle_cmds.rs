use crate::{cmd::Cmd, piece::Piece};

pub fn handle_cmds(cmds: &[Cmd], piece: &mut Piece) {
    for cmd in cmds {
        match cmd {
            Cmd::Move(dir) => {
                piece.move_mut(dir);
            }
            Cmd::Rotate => piece.rotate_mut(),
        }
    }
}
