use crate::{prelude::SCREEN_HEIGHT, touches::Touches, GameState, Msg};

pub fn handle_mousedown(
    x: i32,
    y: i32,
    state: &GameState,
    touches: &mut Touches,
    is_right_click: bool,
) -> Msg {
    if is_right_click {
        return Msg::Nada;
    }

    let max_y = SCREEN_HEIGHT - 50;

    if state.is_normal() && y < max_y {
        touches.assign_down(x, y);
    }

    Msg::Nada
}
