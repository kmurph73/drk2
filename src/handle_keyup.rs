use crate::{
    handle_keydown::{DOWN, LEFT, RIGHT, UP, X},
    keyboard::KeyboardState,
};

pub fn handle_keyup(key: u8, keys: &mut KeyboardState) {
    match key {
        X => keys.pressed.x = false,
        UP => keys.pressed.up = false,
        LEFT => keys.pressed.left = false,
        RIGHT => keys.pressed.right = false,
        DOWN => keys.pressed.down = false,
        _ => {}
    };
}
