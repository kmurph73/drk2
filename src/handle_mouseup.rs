use crate::{
    cmd::Cmd, globals::Globals, touches::Touches, ButtonKind, GameState, ImageButton, Msg,
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
    victory_buttons: &[ImageButton],
    cmds: &mut Vec<Cmd>,
    globals: &Globals,
) -> Msg {
    if touches.down.is_none() {
        return Msg::Nada;
    }

    if is_right_click {
        return Msg::Nada;
    }

    match state {
        GameState::Paused => {
            if let Some(btn) = help_buttons.iter().find(|b| b.dstrect.contains(x, y)) {
                match btn.kind {
                    ButtonKind::Resume => return Msg::ResumeGame,
                    ButtonKind::NewGame => return Msg::NewGame,
                    ButtonKind::Menu => return Msg::Menu,
                    ButtonKind::Quit => return Msg::Quit,
                    _ => {}
                }
            };
        }
        GameState::Victory => {
            if let Some(btn) = victory_buttons.iter().find(|b| b.dstrect.contains(x, y)) {
                match btn.kind {
                    ButtonKind::NewGame => return Msg::NewGame,
                    ButtonKind::NextLevel => {
                        println!("NEXT LEVEL PLZ");
                        return Msg::NextLevel;
                    }
                    ButtonKind::Menu => return Msg::Menu,
                    ButtonKind::Quit => return Msg::Quit,
                    _ => {}
                }
            };
        }
        GameState::Defeat => {
            if let Some(btn) = endgame_buttons.iter().find(|b| b.dstrect.contains(x, y)) {
                match btn.kind {
                    ButtonKind::NewGame => return Msg::NewGame,
                    ButtonKind::Menu => return Msg::Menu,
                    ButtonKind::Quit => return Msg::Quit,
                    _ => {}
                }
            };
        }
        GameState::Menu(_) => {
            if let Some(btn) = menu_buttons.iter().find(|b| b.dstrect.contains(x, y)) {
                match btn.kind {
                    ButtonKind::NewGame => return Msg::NewGame,
                    ButtonKind::About => return Msg::About,
                    ButtonKind::Quit => return Msg::Quit,
                    _ => {}
                }
            }
        }
        GameState::Normal(_) => {
            if globals.menu_btn.contains(x, y) {
                return Msg::PauseGame;
            } else if !touches.dragged && touches.down.is_some() {
                let cmd = Cmd::Rotate;
                cmds.push(cmd);
            }
        }
        GameState::PreppingNextPiece(_, _) => {
            if globals.menu_btn.contains(x, y) {
                return Msg::PauseGame;
            }
        }
        GameState::About => {
            return Msg::Menu;
        }
        _ => {}
    }

    touches.clear();

    Msg::MouseUp
}
