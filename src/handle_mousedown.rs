use crate::{touches::Touches, Msg};

pub fn handle_mousedown(
    x: i32,
    y: i32,
    current_ts: u64,
    touches: &mut Touches,
    is_right_click: bool,
) -> Msg {
    if is_right_click {
        return Msg::Nada;
    }

    touches.assign_down(x, y, current_ts);

    Msg::Nada
}
