use crate::{
    handle_keydown::{C, DOWN, LEFT, N, P, R, RIGHT, SPACE, UP, X, Y},
    keyboard::KeyboardState,
};

pub fn handle_keyup(key: u8, keys: &mut KeyboardState) {
    match key {
        X => {
            keys.pressed.x = false;
            keys.enabled.x = true;
        }
        C => {
            keys.pressed.c = false;
            keys.enabled.c = true;
        }
        UP => keys.pressed.up = false,
        LEFT => keys.pressed.left = false,
        RIGHT => keys.pressed.right = false,
        DOWN => keys.pressed.down = false,
        SPACE => keys.pressed.space = false,
        Y => keys.pressed.y = false,
        N => keys.pressed.n = false,
        R => keys.pressed.r = false,
        P => keys.pressed.p = false,
        _ => {}
    };
}
