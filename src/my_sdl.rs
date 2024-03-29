use std::ffi::CString;

use std::ptr::null_mut;

use crate::globals::Globals;

#[allow(clippy::all)]
#[allow(warnings, unused)]
mod bindings {
    include!("../bindings.rs");
}

#[allow(clippy::all)]
pub use self::bindings::{
    IMG_Init, IMG_InitFlags_IMG_INIT_PNG, IMG_LoadTexture, SDL_BlendMode_SDL_BLENDMODE_BLEND,
    SDL_Color, SDL_CreateRenderer, SDL_CreateTextureFromSurface, SDL_CreateWindow, SDL_Delay,
    SDL_DestroyRenderer, SDL_DestroyWindow, SDL_Event, SDL_EventType_SDL_APP_DIDENTERFOREGROUND,
    SDL_EventType_SDL_APP_WILLENTERBACKGROUND, SDL_EventType_SDL_KEYDOWN, SDL_EventType_SDL_KEYUP,
    SDL_EventType_SDL_MOUSEBUTTONDOWN, SDL_EventType_SDL_MOUSEBUTTONUP,
    SDL_EventType_SDL_MOUSEMOTION, SDL_EventType_SDL_QUIT, SDL_FreeSurface, SDL_GetBasePath,
    SDL_GetError, SDL_GetPrefPath, SDL_GetTicks64, SDL_GetWindowSize, SDL_Init, SDL_Log,
    SDL_PollEvent, SDL_QueryTexture, SDL_Quit, SDL_Rect, SDL_RenderClear, SDL_RenderCopy,
    SDL_RenderDrawLine, SDL_RenderDrawRect, SDL_RenderFillRect, SDL_RenderPresent,
    SDL_RenderSetLogicalSize, SDL_RenderSetScale, SDL_Renderer,
    SDL_RendererFlags_SDL_RENDERER_ACCELERATED, SDL_RendererFlags_SDL_RENDERER_PRESENTVSYNC,
    SDL_Scancode_SDL_SCANCODE_ESCAPE, SDL_SetHint, SDL_SetRenderDrawBlendMode,
    SDL_SetRenderDrawColor, SDL_SetWindowModalFor, SDL_Texture, SDL_Window,
    SDL_WindowEventID_SDL_WINDOWEVENT_FOCUS_GAINED, SDL_WindowEventID_SDL_WINDOWEVENT_FOCUS_LOST,
    SDL_WindowFlags_SDL_WINDOW_ALLOW_HIGHDPI, SDL_WindowFlags_SDL_WINDOW_BORDERLESS,
    SDL_WindowFlags_SDL_WINDOW_FULLSCREEN, SDL_INIT_VIDEO, SDL_WINDOWPOS_UNDEFINED_MASK,
};

pub enum SdlEvent {
    KeyDown(u8),
    KeyUp(u8),
    Quit,
    MouseDown { x: i32, y: i32, button: u8 },
    MouseUp { x: i32, y: i32, button: u8 },
    Motion((i32, i32)),
    WillEnterBackground,
    DidEnterForeground,
}

pub struct MySdl {
    pub texture: *mut SDL_Texture,
    pub about_texture: *mut SDL_Texture,
    pub renderer: *mut SDL_Renderer,
    pub window: *mut SDL_Window,
}

impl MySdl {
    pub fn get_ticks() -> u64 {
        unsafe { SDL_GetTicks64() }
    }

    pub fn log(msg: String) {
        let str = CString::new(msg).unwrap();
        unsafe {
            SDL_Log(str.as_ptr());
        }
    }

    pub fn init_sdl(is_android: bool) -> (MySdl, Globals) {
        unsafe {
            if SDL_Init(SDL_INIT_VIDEO) < 0 {
                panic!("failed to initialize sdl2 with video");
            };

            let window_flags = if is_android {
                SDL_WindowFlags_SDL_WINDOW_ALLOW_HIGHDPI
                    | SDL_WindowFlags_SDL_WINDOW_BORDERLESS
                    | SDL_WindowFlags_SDL_WINDOW_FULLSCREEN
            } else {
                SDL_WindowFlags_SDL_WINDOW_ALLOW_HIGHDPI
            };

            let title = CString::new("Dr. Kodama").expect("CString::new failed");

            let window_width = 700;
            let window_height = 1000;

            let window = SDL_CreateWindow(
                title.as_ptr(),
                0,
                0,
                window_width,
                window_height,
                window_flags,
            );

            let linear = CString::new("linear").unwrap();
            let quality = CString::new("SDL_RENDER_SCALE_QUALITY").unwrap();
            SDL_SetHint(quality.as_ptr(), linear.as_ptr());

            let renderer_flags = SDL_RendererFlags_SDL_RENDERER_PRESENTVSYNC;

            let renderer = SDL_CreateRenderer(window, 0, renderer_flags);

            let scale = 2.0;
            SDL_RenderSetScale(renderer, scale, scale);

            let mut w = 0;
            let mut h = 0;
            SDL_GetWindowSize(window, &mut w, &mut h);
            SDL_RenderSetLogicalSize(renderer, w, h);
            let window_width = w;
            let window_height = h;

            IMG_Init((IMG_InitFlags_IMG_INIT_PNG).try_into().unwrap());

            let file = CString::new("resources/skyline-packer-output.png").unwrap();
            let texture = IMG_LoadTexture(renderer, file.as_ptr());

            let globals = Globals::make(window_width, window_height);

            SDL_SetRenderDrawBlendMode(renderer, SDL_BlendMode_SDL_BLENDMODE_BLEND);

            let about_file = CString::new("resources/aboot.png").unwrap();
            let about_texture = IMG_LoadTexture(renderer, about_file.as_ptr());

            let sdl = MySdl {
                texture,
                about_texture,
                renderer,
                window,
            };

            (sdl, globals)
        }
    }

    pub fn clear(&self) {
        unsafe {
            SDL_SetRenderDrawColor(self.renderer, 10, 10, 10, 255);
            SDL_RenderClear(self.renderer);
        }
    }

    pub fn quit(&self) {
        unsafe {
            SDL_DestroyRenderer(self.renderer);
            SDL_DestroyWindow(self.window);
            SDL_Quit();
        }
    }

    pub fn present(&self) {
        unsafe {
            SDL_RenderPresent(self.renderer);
        }
    }

    pub fn draw(&self, globals: &Globals) {
        let rect = SDL_Rect::new(0, globals.window_height - 100, globals.window_width, 100);
        unsafe {
            SDL_SetRenderDrawColor(self.renderer, 150, 150, 150, 255);
            SDL_RenderFillRect(self.renderer, &rect);
        }
    }

    pub fn draw_horizontal_line(&self, x: i32, y: i32, w: i32) {
        let rect = SDL_Rect { x, y, w, h: 2 };

        unsafe {
            SDL_RenderDrawRect(self.renderer, &rect);
        }
    }

    pub fn draw_vertical_line(&self, x: i32, y: i32, h: i32) {
        let rect = SDL_Rect { x, y, w: 2, h };

        unsafe {
            SDL_RenderDrawRect(self.renderer, &rect);
        }
    }

    pub fn draw_image(&self, srcrect: &SDL_Rect, dstrect: &SDL_Rect) {
        unsafe {
            SDL_RenderCopy(self.renderer, self.texture, srcrect, dstrect);
        }
    }

    pub fn draw_about_image(&self, srcrect: &SDL_Rect, dstrect: &SDL_Rect) {
        unsafe {
            SDL_RenderCopy(self.renderer, self.about_texture, srcrect, dstrect);
        }
    }

    pub fn set_draw_color_tuple(&self, (r, g, b, a): (u8, u8, u8, u8)) {
        unsafe {
            SDL_SetRenderDrawColor(self.renderer, r, g, b, a);
        }
    }

    pub fn set_draw_color(&self, SDL_Color { r, g, b, a }: SDL_Color) {
        unsafe {
            SDL_SetRenderDrawColor(self.renderer, r, g, b, a);
        }
    }

    pub fn fill_rect(&self, rect: &SDL_Rect) {
        unsafe {
            SDL_RenderFillRect(self.renderer, rect);
        }
    }

    pub fn draw_rect(&self, rect: &SDL_Rect) {
        unsafe {
            SDL_RenderDrawRect(self.renderer, rect);
        }
    }

    #[allow(non_upper_case_globals)]
    pub fn poll_events(&self) -> Vec<SdlEvent> {
        let mut events: Vec<SdlEvent> = Vec::new();

        unsafe {
            let mut _event: *mut SDL_Event = null_mut();

            let mut event = SDL_Event { type_: 1 };
            let sdl_event = &mut event as *mut SDL_Event;

            while SDL_PollEvent(sdl_event) == 1 {
                let button = (*sdl_event).button;

                match (*sdl_event).type_ {
                    SDL_EventType_SDL_KEYDOWN => {
                        let event = SdlEvent::KeyDown(button.button);
                        events.push(event);
                    }
                    SDL_EventType_SDL_KEYUP => {
                        let event = SdlEvent::KeyUp(button.button);
                        events.push(event);
                    }
                    SDL_EventType_SDL_QUIT => {
                        let event = SdlEvent::Quit;
                        events.push(event);
                    }
                    SDL_EventType_SDL_MOUSEBUTTONDOWN => {
                        let event = SdlEvent::MouseDown {
                            x: button.x,
                            y: button.y,
                            button: button.button,
                        };
                        events.push(event);
                    }
                    SDL_EventType_SDL_MOUSEMOTION => {
                        let event = SdlEvent::Motion((button.x, button.y));
                        events.push(event);
                    }

                    SDL_EventType_SDL_APP_WILLENTERBACKGROUND => {
                        let event = SdlEvent::WillEnterBackground;
                        events.push(event);
                    }
                    SDL_EventType_SDL_APP_DIDENTERFOREGROUND => {
                        let event = SdlEvent::DidEnterForeground;
                        events.push(event);
                    }

                    SDL_EventType_SDL_MOUSEBUTTONUP => {
                        let event = SdlEvent::MouseUp {
                            x: button.x,
                            y: button.y,
                            button: button.button,
                        };

                        events.push(event);
                    }
                    _ => {}
                }
            }
        }

        events
    }
}
