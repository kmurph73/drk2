use crate::{cmd::Cmd, dot::Dot, piece::Piece};

pub fn handle_cmds(cmds: &[Cmd], piece: &mut Piece, squares: &[Option<Dot>]) {
    for cmd in cmds {
        match cmd {
            Cmd::Move(dir) => {
                if let Some(new_pos) = piece.attempt_move(dir, squares) {
                    let (lhs, rhs) = new_pos;
                    piece.set_pos(lhs, rhs);
                }
            }
            Cmd::Rotate => {
                if let Some(rotation) = piece.attempt_rotation(squares) {
                    piece.set_rotation(&rotation);
                }
            }
            Cmd::DropPiece => {
                let new_pos = piece.find_lowest_drop(squares);
                let (lhs, rhs) = new_pos;
                piece.set_pos(lhs, rhs);
            }
        }
    }
}
