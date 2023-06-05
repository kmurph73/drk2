use crate::{Button, ButtonKind, GameState, Msg};

pub fn handle_mousedown(
    x: i32,
    y: i32,
    state: &GameState,
    is_right_click: bool,
    buttons: &[Button],
) -> Msg {
    if is_right_click {
        return Msg::Nada;
    }

    if state.is_paused() {
        if let Some(btn) = buttons.iter().find(|b| b.rect.within(x, y)) {
            match btn.kind {
                ButtonKind::Resume => return Msg::ResumeGame,
                ButtonKind::NewGame => return Msg::NewGame,
                ButtonKind::Menu => {}
            }
        };
    }

    Msg::Nada
}
