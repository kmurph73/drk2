use crate::{
    cmd::{Cmd, Direction},
    dot::Dot,
    msg::GameMsg,
    piece::Piece,
};

pub fn handle_cmds_mut(
    cmds: &[Cmd],
    piece: &mut Piece,
    squares: &[Option<Dot>],
) -> Option<GameMsg> {
    for cmd in cmds {
        match cmd {
            Cmd::Move(dir) => {
                if let Some(new_pos) = piece.attempt_move(dir, squares) {
                    let (lhs, rhs) = new_pos;
                    piece.set_pos(lhs, rhs);
                } else if *dir == Direction::Down {
                    return Some(GameMsg::PieceLanded);
                }
            }
            Cmd::Rotate => {
                if let Some(rotation) = piece.attempt_rotation(squares) {
                    piece.set_rotation(&rotation);
                }
            }
            Cmd::DropPiece => {
                return Some(GameMsg::DropPiece);
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

    None
}
