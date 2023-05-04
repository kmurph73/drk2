use std::ptr::null_mut;

use crate::{
    handle_keydown::handle_keydown,
    keyboard::KeyboardState,
    my_sdl::{SDL_Event, SDL_EventType_SDL_KEYDOWN, SDL_EventType_SDL_QUIT, SDL_PollEvent},
    Msg,
};

#[allow(non_upper_case_globals)]
pub fn handle_events(keys: &mut KeyboardState) -> Msg {
    unsafe {
        let mut _event: *mut SDL_Event = null_mut();

        let mut event = SDL_Event { type_: 1 };
        let sdl_event = &mut event as *mut SDL_Event;

        while SDL_PollEvent(sdl_event) == 1 {
            let button = (*sdl_event).button;

            match (*sdl_event).type_ {
                SDL_EventType_SDL_KEYDOWN => {
                    if let Msg::Quit = handle_keydown(button.button, keys) {
                        return Msg::Quit;
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
