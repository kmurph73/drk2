use crate::{
    cmd::Cmd, globals::Globals, msg::MetaMsg, touches::Touches, ButtonKind, GameState, ImageButton,
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
) -> Option<MetaMsg> {
    touches.down?;

    if is_right_click {
        return None;
    }

    match state {
        GameState::Paused => {
            if let Some(btn) = help_buttons.iter().find(|b| b.dstrect.contains(x, y)) {
                match btn.kind {
                    ButtonKind::Resume => return Some(MetaMsg::ResumeGame),
                    ButtonKind::NewGame => return Some(MetaMsg::NewGame),
                    ButtonKind::Menu => return Some(MetaMsg::Menu),
                    ButtonKind::Quit => return Some(MetaMsg::Quit),
                    _ => {}
                }
            };
        }
        GameState::Victory => {
            if let Some(btn) = victory_buttons.iter().find(|b| b.dstrect.contains(x, y)) {
                match btn.kind {
                    ButtonKind::NewGame => return Some(MetaMsg::NewGame),
                    ButtonKind::NextLevel => {
                        return Some(MetaMsg::NextLevel);
                    }
                    ButtonKind::Menu => return Some(MetaMsg::Menu),
                    ButtonKind::Quit => return Some(MetaMsg::Quit),
                    _ => {}
                }
            };
        }
        GameState::Defeat => {
            if let Some(btn) = endgame_buttons.iter().find(|b| b.dstrect.contains(x, y)) {
                match btn.kind {
                    ButtonKind::NewGame => return Some(MetaMsg::NewGame),
                    ButtonKind::Menu => return Some(MetaMsg::Menu),
                    ButtonKind::Quit => return Some(MetaMsg::Quit),
                    _ => {}
                }
            };
        }
        GameState::Menu(_) => {
            if let Some(btn) = menu_buttons.iter().find(|b| b.dstrect.contains(x, y)) {
                match btn.kind {
                    ButtonKind::NewGame => return Some(MetaMsg::NewGame),
                    ButtonKind::About => return Some(MetaMsg::About),
                    ButtonKind::Quit => return Some(MetaMsg::Quit),
                    _ => {}
                }
            }

            return Some(MetaMsg::MenuMouseUp);
        }
        GameState::Normal(_) => {
            if globals.menu_btn.contains(x, y) {
                return Some(MetaMsg::PauseGame);
            } else if !touches.dragged && touches.down.is_some() {
                let cmd = Cmd::Rotate;
                cmds.push(cmd);
            }
        }
        GameState::PreppingNextPiece(_, _) => {
            if globals.menu_btn.contains(x, y) {
                return Some(MetaMsg::PauseGame);
            }
        }
        GameState::About => {
            return Some(MetaMsg::Menu);
        }
        _ => {}
    }

    touches.clear();

    None
}
