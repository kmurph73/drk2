use std::ptr::null_mut;

use crate::{
    cmd::Cmd,
    handle_keydown::handle_keydown,
    handle_keyup::handle_keyup,
    handle_mousedown::handle_mousedown,
    handle_mouseup::handle_mouseup,
    keyboard::KeyboardState,
    my_sdl::{
        SDL_Event, SDL_EventType_SDL_APP_DIDENTERFOREGROUND,
        SDL_EventType_SDL_APP_WILLENTERBACKGROUND, SDL_EventType_SDL_KEYDOWN,
        SDL_EventType_SDL_KEYUP, SDL_EventType_SDL_MOUSEBUTTONDOWN,
        SDL_EventType_SDL_MOUSEBUTTONUP, SDL_EventType_SDL_MOUSEMOTION, SDL_EventType_SDL_QUIT,
        SDL_PollEvent,
    },
    touches::Touches,
    GameState, ImageButton, Msg,
};

#[allow(non_upper_case_globals)]
pub fn handle_events(
    keys: &mut KeyboardState,
    cmds: &mut Vec<Cmd>,
    touches: &mut Touches,
    state: &GameState,
    help_buttons: &[ImageButton],
    menu_buttons: &[ImageButton],
    endgame_buttons: &[ImageButton],
    current_ts: u128,
) -> Msg {
    unsafe {
        let mut _event: *mut SDL_Event = null_mut();

        let mut event = SDL_Event { type_: 1 };
        let sdl_event = &mut event as *mut SDL_Event;

        while SDL_PollEvent(sdl_event) == 1 {
            let button = (*sdl_event).button;

            match (*sdl_event).type_ {
                SDL_EventType_SDL_KEYDOWN => {
                    let msg = handle_keydown(button.button, keys, state, cmds);
                    match msg {
                        Msg::Nada => {}
                        _ => return msg,
                    }
                }
                SDL_EventType_SDL_KEYUP => {
                    handle_keyup(button.button, keys);
                }
                SDL_EventType_SDL_MOUSEBUTTONDOWN => {
                    let is_right_click = button.button == 3;

                    let msg =
                        handle_mousedown(button.x, button.y, current_ts, touches, is_right_click);

                    match msg {
                        Msg::Nada => {}
                        _ => return msg,
                    }
                }
                SDL_EventType_SDL_MOUSEMOTION => {
                    touches.assign_motion(button.x, button.y);
                }
                SDL_EventType_SDL_APP_WILLENTERBACKGROUND => {
                    if state.is_normal() {
                        return Msg::SuspendGame;
                    }
                }

                SDL_EventType_SDL_APP_DIDENTERFOREGROUND => {
                    if state.is_suspended() {
                        return Msg::ResumeGame;
                    }
                }
                SDL_EventType_SDL_MOUSEBUTTONUP => {
                    let is_right_click = button.button == 3;

                    let msg = handle_mouseup(
                        button.x,
                        button.y,
                        state,
                        touches,
                        is_right_click,
                        help_buttons,
                        menu_buttons,
                        endgame_buttons,
                        cmds,
                    );

                    match msg {
                        Msg::Nada => {}
                        _ => {
                            touches.clear();
                            return msg;
                        }
                    }
                }
                SDL_EventType_SDL_QUIT => {
                    return Msg::Quit;
                }
                _ => {}
            }
        }

        Msg::Nada
    }
}
