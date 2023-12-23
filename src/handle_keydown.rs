use crate::{
    cmd::{Cmd, Direction},
    keyboard::{Keyboard, KeyboardState},
    msg::MetaMsg,
    GameState,
};

pub const Y: u8 = 28;
pub const C: u8 = 6;
pub const X: u8 = 27;
pub const N: u8 = 17;
pub const ESCAPE: u8 = 41;
pub const UP: u8 = 82;
pub const DOWN: u8 = 81;
pub const LEFT: u8 = 80;
pub const RIGHT: u8 = 79;
pub const SPACE: u8 = 44;
pub const P: u8 = 19;
pub const R: u8 = 21;

pub fn handle_keydown(
    key: u8,
    keys: &mut KeyboardState,
    state: &GameState,
    cmds: &mut Vec<Cmd>,
) -> Option<MetaMsg> {
    match key {
        ESCAPE => keys.pressed.esc = true,
        X => keys.pressed.x = true,
        C => keys.pressed.c = true,
        UP => keys.pressed.up = true,
        LEFT => keys.pressed.left = true,
        RIGHT => keys.pressed.right = true,
        DOWN => keys.pressed.down = true,
        SPACE => keys.pressed.space = true,
        Y => keys.pressed.y = true,
        N => keys.pressed.n = true,
        P => keys.pressed.p = true,
        R => keys.pressed.r = true,
        _ => {}
    }

    let Keyboard {
        esc,
        x,
        left,
        up,
        right,
        down,
        n,
        p,
        space,
        ..
    } = keys.pressed;

    if esc {
        return Some(MetaMsg::Quit);
    }

    if let Some(dir) = if left {
        Some(Direction::Left)
    } else if right {
        Some(Direction::Right)
    } else if down {
        Some(Direction::Down)
    } else {
        None
    } {
        let cmd = Cmd::Move(dir);
        cmds.push(cmd);
    }

    if up {
        let cmd = Cmd::DropPiece;
        cmds.push(cmd);
    }

    if p && keys.enabled.p {
        if state.is_normal() {
            return Some(MetaMsg::PauseGame);
        } else if state.is_paused() {
            return Some(MetaMsg::ResumeGame);
        }
    }

    if n && keys.enabled.n && state.is_endgame() {
        return Some(MetaMsg::NewGame);
    }

    if x && keys.enabled.x {
        let cmd = Cmd::Rotate;
        cmds.push(cmd);
        keys.enabled.x = false;
    }

    if space && keys.enabled.space {
        let cmd = Cmd::DropPiece;
        cmds.push(cmd);
        keys.enabled.c = false;
    }

    None
}
