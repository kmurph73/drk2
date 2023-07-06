use crate::{
    cmd::Cmd, globals::Globals, my_sdl::MySdl, touches::Touches, ButtonKind, GameState,
    ImageButton, Msg,
};

pub fn handle_mouseup(
    x: i32,
    y: i32,
    state: &GameState,
    touches: &mut Touches,
    is_right_click: bool,
    help_buttons: &[ImageButton],
    menu_buttons: &[ImageButton],
    endgame_buttons: &[ImageButton],
    cmds: &mut Vec<Cmd>,
    sdl: &MySdl,
    globals: &Globals,
) -> Msg {
    if touches.down.is_none() {
        return Msg::Nada;
    }

    if is_right_click {
        return Msg::Nada;
    }

    if state.is_paused() {
        if let Some(btn) = help_buttons.iter().find(|b| b.dstrect.contains(x, y)) {
            match btn.kind {
                ButtonKind::Resume => return Msg::ResumeGame,
                ButtonKind::NewGame => return Msg::NewGame,
                ButtonKind::Menu => return Msg::Menu,
                ButtonKind::Quit => return Msg::Quit,
                _ => {}
            }
        };
    } else if state.is_endgame() {
        if let Some(btn) = endgame_buttons.iter().find(|b| b.dstrect.contains(x, y)) {
            match btn.kind {
                ButtonKind::NewGame => return Msg::NewGame,
                ButtonKind::Menu => return Msg::Menu,
                ButtonKind::Quit => return Msg::Quit,
                _ => {}
            }
        };
    } else if state.is_menu() {
        if let Some(btn) = menu_buttons.iter().find(|b| b.dstrect.contains(x, y)) {
            match btn.kind {
                ButtonKind::NewGame => return Msg::NewGame,
                ButtonKind::Quit => return Msg::Quit,
                _ => {}
            }
        }
    } else if state.is_normal() {
        if globals.menu_btn.contains(x, y) {
            return Msg::PauseGame;
        } else if !touches.dragged && touches.down.is_some() {
            let cmd = Cmd::Rotate;
            cmds.push(cmd);
        }
    }

    touches.clear();

    Msg::MouseUp
}
