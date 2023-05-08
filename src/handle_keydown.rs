use crate::{
    cmd::{Cmd, Direction},
    keyboard::{Keyboard, KeyboardState},
    Msg,
};

pub const C: u8 = 6;
pub const X: u8 = 27;
pub const ESCAPE: u8 = 41;
pub const UP: u8 = 82;
pub const DOWN: u8 = 81;
pub const LEFT: u8 = 80;
pub const RIGHT: u8 = 79;

pub fn handle_keydown(key: u8, keys: &mut KeyboardState, cmds: &mut Vec<Cmd>) -> Msg {
    match key {
        ESCAPE => keys.pressed.esc = true,
        X => keys.pressed.x = true,
        C => keys.pressed.c = true,
        UP => keys.pressed.up = true,
        LEFT => keys.pressed.left = true,
        RIGHT => keys.pressed.right = true,
        DOWN => keys.pressed.down = true,
        _ => {}
    }

    let Keyboard {
        esc,
        x,
        left,
        up,
        right,
        down,
        c,
        ..
    } = keys.pressed;

    if esc {
        return Msg::Quit;
    }

    if let Some(dir) = if left {
        Some(Direction::Left)
    } else if right {
        Some(Direction::Right)
    } else if down {
        Some(Direction::Down)
    } else if up {
        Some(Direction::Up)
    } else {
        None
    } {
        let cmd = Cmd::Move(dir);
        cmds.push(cmd);
    }

    if x && keys.enabled.x {
        let cmd = Cmd::Rotate;
        cmds.push(cmd);
        keys.enabled.x = false;
    }

    if c && keys.enabled.c {
        let cmd = Cmd::DropPiece;
        cmds.push(cmd);
        keys.enabled.c = false;
    }

    Msg::Nada
}
