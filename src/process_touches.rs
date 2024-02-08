use crate::{
    cmd::{Cmd, Direction},
    globals::Globals,
    my_sdl::MySdl,
    pos::Pos,
    touches::Touches,
};

#[allow(clippy::manual_range_contains)]
pub fn process_touches(touches: &mut Touches, cmds: &mut Vec<Cmd>, globals: &Globals, ts: u64) {
    let Globals { drag_diff, .. } = *globals;

    let mut cmd: Option<Cmd> = None;
    let Pos(x, y) = if let Some(mouse) = &touches.down {
        mouse.pos
    } else {
        return;
    };

    let Pos(current_x, current_y) = if let Some(current) = touches.touches.last() {
        current.pos
    } else {
        return;
    };

    let delta_x = current_x - x;
    let delta_y = current_y - y;

    if delta_x.abs() > delta_y.abs() {
        if delta_x > globals.drag_diff {
            cmd = Some(Cmd::Move(Direction::Right));
        } else if delta_x < -drag_diff {
            cmd = Some(Cmd::Move(Direction::Left));
        }
    }

    if delta_y > drag_diff && delta_y > delta_x.abs() {
        cmd = Some(Cmd::Move(Direction::Down));
    } else if cmd.is_none() {
        if let Some(velo) = &touches.velocity {
            let string = format!("{}veloy: {},{}", touches.touches.len(), velo.0, velo.1);
            MySdl::log(string);

            if velo.1 > 2.0 {
                cmd = Some(Cmd::DropPiece);
            }
        }
    }

    if let Some(cmd) = cmd {
        cmds.push(cmd);
        touches.dragged = true;
        touches.moved_piece(ts);
    }
}
