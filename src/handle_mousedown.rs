use crate::touches::Touches;

pub fn handle_mousedown(
    x: i32,
    y: i32,
    current_ts: u64,
    touches: &mut Touches,
    is_right_click: bool,
) {
    if is_right_click {
        return;
    }

    touches.assign_down(x, y, current_ts);
}
