use crate::{
    cmd::{Cmd, Direction},
    keyboard::{Keyboard, KeyboardState},
    GameState, Msg,
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

pub fn handle_keydown(
    key: u8,
    keys: &mut KeyboardState,
    state: &GameState,
    cmds: &mut Vec<Cmd>,
) -> Msg {
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
        _ => {}
    }

    let Keyboard {
        esc,
        x,
        left,
        up,
        right,
        down,
        y,
        n,
        space,
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

    println!("y: {y} - {}, {:#?}", keys.enabled.y, state);
    if y && keys.enabled.y && *state == GameState::Victory {
        return Msg::NewGame;
    }

    if n && keys.enabled.n && *state == GameState::Victory {
        return Msg::Quit;
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

    Msg::Nada
}
