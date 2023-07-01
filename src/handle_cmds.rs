use crate::{
    cmd::{Cmd, Direction},
    dot::Dot,
    piece::Piece,
    Msg,
};

pub fn handle_cmds_mut(cmds: &[Cmd], piece: &mut Piece, squares: &[Option<Dot>]) -> Msg {
    for cmd in cmds {
        match cmd {
            Cmd::Move(dir) => {
                if let Some(new_pos) = piece.attempt_move(dir, squares) {
                    let (lhs, rhs) = new_pos;
                    piece.set_pos(lhs, rhs);
                } else if *dir == Direction::Down {
                    return Msg::PieceLanded;
                }
            }
            Cmd::Rotate => {
                if let Some(rotation) = piece.attempt_rotation(squares) {
                    piece.set_rotation(&rotation);
                }
            }
            Cmd::DropPiece => {
                return Msg::DropPiece;
            }
            Cmd::SnapLeft => {
                if let Some((lhs, rhs)) = piece.snap_left(squares) {
                    piece.set_pos(lhs, rhs);
                }
            }
            Cmd::SnapRight => {
                if let Some((lhs, rhs)) = piece.snap_right(squares) {
                    piece.set_pos(lhs, rhs);
                }
            }
        }
    }

    Msg::Nada
}
