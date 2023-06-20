use crate::{
    cmd::{Cmd, Direction},
    pos::Pos,
    prelude::{DRAG_DIFF, DROP_DRAG_DIFF},
    touches::Touches,
};

#[allow(clippy::manual_range_contains)]
pub fn process_touches(touches: &mut Touches, cmds: &mut Vec<Cmd>) {
    let mut cmd: Option<Cmd> = None;
    let diff = DRAG_DIFF;
    let Pos(x, y) = if let Some(down) = touches.down {
        down
    } else {
        return;
    };

    let Pos(current_x, current_y) = if let Some(current) = touches.current {
        current
    } else {
        return;
    };

    let delta_x = current_x - x;

    if delta_x > diff {
        cmd = Some(Cmd::Move(Direction::Right));
    } else if delta_x < -diff {
        cmd = Some(Cmd::Move(Direction::Left));
    }

    let delta_y = current_y - y;

    if delta_y > diff && delta_y > delta_x.abs() {
        cmd = Some(Cmd::Move(Direction::Down));
    } else if delta_y < -DROP_DRAG_DIFF && cmd.is_none() {
        cmd = Some(Cmd::DropPiece);
    }

    if let Some(cmd) = cmd {
        cmds.push(cmd);
        touches.dragged = true;
        touches.moved_piece();
    }
}
