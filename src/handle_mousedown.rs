use crate::{touches::Touches, GameState, Msg};

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

    if state.is_normal() {
        touches.assign_down(x, y);
    }

    Msg::Nada
}
