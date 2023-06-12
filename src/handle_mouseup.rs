use crate::{
    cmd::Cmd, my_sdl::SDL_Rect, prelude::MENU_BTN, touches::Touches, ButtonKind, GameState,
    ImageButton, Msg, TextButton,
};

pub fn handle_mouseup(
    x: i32,
    y: i32,
    state: &GameState,
    touches: &mut Touches,
    is_right_click: bool,
    help_buttons: &[TextButton],
    endgame_buttons: &[TextButton],
    menu_buttons: &[TextButton],
    image_buttons: &[ImageButton],
    cmds: &mut Vec<Cmd>,
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
    } else if state.is_normal() {
        let (rect_x, rect_y, w, h) = MENU_BTN;
        let rect = SDL_Rect {
            x: rect_x,
            y: rect_y,
            w,
            h,
        };

        if rect.contains(x, y) {
            return Msg::PauseGame;
        } else if !touches.dragged {
            let cmd = Cmd::Rotate;
            cmds.push(cmd);
        }
    }

    touches.clear();

    Msg::Nada
}