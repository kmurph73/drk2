use crate::touches::Touches;

pub fn handle_mousedown(x: i32, y: i32, touches: &mut Touches, is_right_click: bool, ts: u64) {
    if is_right_click {
        return;
    }

    touches.assign_down(x, y, ts);
}
