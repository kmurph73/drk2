use crate::{Button, ButtonKind, GameState, Msg};

pub fn handle_mousedown(
    x: i32,
    y: i32,
    state: &GameState,
    is_right_click: bool,
    help_buttons: &[Button],
    endgame_buttons: &[Button],
    menu_buttons: &[Button],
) -> Msg {
    if is_right_click {
        return Msg::Nada;
    }

    if state.is_paused() {
        if let Some(btn) = help_buttons.iter().find(|b| b.rect.contains(x, y)) {
            match btn.kind {
                ButtonKind::Resume => return Msg::ResumeGame,
                ButtonKind::NewGame => return Msg::NewGame,
                ButtonKind::Menu => return Msg::Menu,
                ButtonKind::Quit => return Msg::Quit,
            }
        };
    } else if state.is_endgame() {
        if let Some(btn) = endgame_buttons.iter().find(|b| b.rect.contains(x, y)) {
            match btn.kind {
                ButtonKind::Resume => {}
                ButtonKind::NewGame => return Msg::NewGame,
                ButtonKind::Menu => return Msg::Menu,
                ButtonKind::Quit => return Msg::Quit,
            }
        };
    } else if state.is_menu() {
        if let Some(btn) = menu_buttons.iter().find(|b| b.rect.contains(x, y)) {
            match btn.kind {
                ButtonKind::Resume => {}
                ButtonKind::NewGame => return Msg::NewGame,
                ButtonKind::Menu => {}
                ButtonKind::Quit => return Msg::Quit,
            }
        };
    }

    Msg::Nada
}
