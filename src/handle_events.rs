use std::ptr::null_mut;

use crate::{
    cmd::Cmd,
    handle_keydown::handle_keydown,
    handle_keyup::handle_keyup,
    handle_mousedown::handle_mousedown,
    keyboard::KeyboardState,
    my_sdl::{
        SDL_Event, SDL_EventType_SDL_KEYDOWN, SDL_EventType_SDL_KEYUP,
        SDL_EventType_SDL_MOUSEBUTTONDOWN, SDL_EventType_SDL_QUIT, SDL_PollEvent,
    },
    Button, GameState, Msg,
};

#[allow(non_upper_case_globals)]
pub fn handle_events(
    keys: &mut KeyboardState,
    cmds: &mut Vec<Cmd>,
    state: &GameState,
    buttons: &[Button],
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

                    let msg = handle_mousedown(button.x, button.y, state, is_right_click, buttons);

                    match msg {
                        Msg::Nada => {}
                        _ => return msg,
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
