use crate::{ButtonKind, GameState, ImageButton, Msg, TextButton};

pub fn handle_mousedown(
    x: i32,
    y: i32,
    state: &GameState,
    is_right_click: bool,
    help_buttons: &[TextButton],
    endgame_buttons: &[TextButton],
    menu_buttons: &[TextButton],
    image_buttons: &[ImageButton],
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
                _ => {}
            }
        };
    } else if state.is_endgame() {
        if let Some(btn) = endgame_buttons.iter().find(|b| b.rect.contains(x, y)) {
            match btn.kind {
                ButtonKind::Resume => {}
                ButtonKind::NewGame => return Msg::NewGame,
                ButtonKind::Menu => return Msg::Menu,
                ButtonKind::Quit => return Msg::Quit,
                _ => {}
            }
        };
    } else if state.is_menu() {
        if let Some(btn) = menu_buttons.iter().find(|b| b.rect.contains(x, y)) {
            match btn.kind {
                ButtonKind::NewGame => return Msg::NewGame,
                ButtonKind::Quit => return Msg::Quit,
                _ => {}
            }
        } else if let Some(btn) = image_buttons.iter().find(|b| b.dstrect.contains(x, y)) {
            match btn.kind {
                ButtonKind::LevelUp => return Msg::LevelUp,
                ButtonKind::LevelDown => return Msg::LevelDown,
                _ => {}
            }
        }
    }

    Msg::Nada
}
