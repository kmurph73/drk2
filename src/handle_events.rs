use crate::{
    cmd::Cmd,
    globals::Globals,
    handle_keydown::handle_keydown,
    handle_keyup::handle_keyup,
    handle_mousedown::handle_mousedown,
    handle_mouseup::handle_mouseup,
    keyboard::KeyboardState,
    msg::MetaMsg,
    my_sdl::{MySdl, SdlEvent},
    touches::Touches,
    GameState, ImageButton,
};

#[allow(non_upper_case_globals)]
pub fn handle_events(
    sdl: &MySdl,
    keys: &mut KeyboardState,
    cmds: &mut Vec<Cmd>,
    touches: &mut Touches,
    state: &GameState,
    help_buttons: &[ImageButton],
    menu_buttons: &[ImageButton],
    endgame_buttons: &[ImageButton],
    victory_buttons: &[ImageButton],
    globals: &Globals,
    ts: u64,
) -> Vec<MetaMsg> {
    let events = sdl.poll_events();

    let mut msgs: Vec<MetaMsg> = Vec::new();

    for event in events {
        match event {
            SdlEvent::KeyDown(key) => {
                if let Some(msg) = handle_keydown(key, keys, state, cmds) {
                    msgs.push(msg);
                }
            }
            SdlEvent::KeyUp(key) => {
                handle_keyup(key, keys);
            }
            SdlEvent::MouseDown { x, y, button } => {
                let is_right_click = button == 3;

                handle_mousedown(x, y, touches, is_right_click, ts);
            }
            SdlEvent::Motion((x, y)) => {
                touches.assign_motion(x, y, ts);
                if let Some((x, y)) = &touches.velocity {
                    let msg = format!("velo: {x}, {y}");
                    MySdl::log(msg);
                }
            }
            SdlEvent::WillEnterBackground => {
                if state.is_normal() {
                    msgs.push(MetaMsg::SuspendGame);
                }
            }
            SdlEvent::DidEnterForeground => {
                if state.is_suspended() {
                    msgs.push(MetaMsg::ResumeGame);
                }
            }
            SdlEvent::MouseUp { x, y, button } => {
                let is_right_click = button == 3;

                match handle_mouseup(
                    x,
                    y,
                    state,
                    touches,
                    is_right_click,
                    help_buttons,
                    menu_buttons,
                    endgame_buttons,
                    victory_buttons,
                    cmds,
                    globals,
                ) {
                    None => {}
                    Some(msg) => {
                        touches.clear();
                        msgs.push(msg);
                    }
                }
            }
            SdlEvent::Quit => {
                msgs.push(MetaMsg::Quit);
            }
        }
    }

    msgs
}
