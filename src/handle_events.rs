use std::ptr::null_mut;

use crate::{
    my_sdl::{SDL_Event, SDL_EventType_SDL_KEYDOWN, SDL_EventType_SDL_QUIT, SDL_PollEvent},
    Msg,
};

pub const ESCAPE: u8 = 41;

#[allow(non_upper_case_globals)]
pub fn handle_events() -> Msg {
    unsafe {
        let mut _event: *mut SDL_Event = null_mut();

        let mut event = SDL_Event { type_: 1 };
        let sdl_event = &mut event as *mut SDL_Event;

        while SDL_PollEvent(sdl_event) == 1 {
            let button = (*sdl_event).button;

            match (*sdl_event).type_ {
                SDL_EventType_SDL_KEYDOWN => {
                    if button.button == ESCAPE {
                        return Msg::Quit;
                    }
                }
                SDL_EventType_SDL_QUIT => {
                    return Msg::Quit;
                }
                _ => break,
            }
        }

        Msg::Nada
    }
}
