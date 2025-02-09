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

    let Pos(current_x, current_y) = if let Some(current) = touches.latest() {
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
        if let Some((x, y)) = touches.velocity {
            let valid = !x.is_nan() && !y.is_nan() && x.is_finite() && y.is_finite();
            let log = format!("{x}, {y}");
            MySdl::log(log);

            if valid && y.abs() > x.abs() && y > 2.5 {
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
