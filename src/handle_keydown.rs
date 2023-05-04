use crate::{
    keyboard::{Keyboard, KeyboardState},
    Msg,
};

const X: u8 = 27;
const ESCAPE: u8 = 41;
const UP: u8 = 82;
const DOWN: u8 = 81;
const LEFT: u8 = 80;
const RIGHT: u8 = 79;

pub fn handle_keydown(key: u8, keys: &mut KeyboardState) -> Msg {
    match key {
        ESCAPE => keys.pressed.esc = true,
        X => keys.pressed.x = true,
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
        ..
    } = keys.pressed;

    Msg::Nada
}
