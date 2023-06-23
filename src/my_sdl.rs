use std::ffi::CString;

use crate::prelude::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[allow(clippy::all)]
#[allow(warnings, unused)]
mod bindings {
    include!("../bindings.rs");
}

#[allow(clippy::all)]
pub use self::bindings::{
    IMG_Init, IMG_InitFlags_IMG_INIT_PNG, IMG_LoadTexture, SDL_BlendMode_SDL_BLENDMODE_BLEND,
    SDL_Color, SDL_CreateRenderer, SDL_CreateTextureFromSurface, SDL_CreateWindow, SDL_Delay,
    SDL_DestroyRenderer, SDL_DestroyWindow, SDL_Event, SDL_EventType_SDL_APP_WILLENTERBACKGROUND,
    SDL_EventType_SDL_KEYDOWN, SDL_EventType_SDL_KEYUP, SDL_EventType_SDL_MOUSEBUTTONDOWN,
    SDL_EventType_SDL_MOUSEBUTTONUP, SDL_EventType_SDL_MOUSEMOTION, SDL_EventType_SDL_QUIT,
    SDL_FreeSurface, SDL_GetError, SDL_GetWindowSize, SDL_Init, SDL_Log, SDL_PollEvent,
    SDL_QueryTexture, SDL_Quit, SDL_Rect, SDL_RenderClear, SDL_RenderCopy, SDL_RenderDrawLine,
    SDL_RenderDrawRect, SDL_RenderFillRect, SDL_RenderPresent, SDL_RenderSetLogicalSize,
    SDL_RenderSetScale, SDL_Renderer, SDL_RendererFlags_SDL_RENDERER_ACCELERATED,
    SDL_RendererFlags_SDL_RENDERER_PRESENTVSYNC, SDL_Scancode_SDL_SCANCODE_ESCAPE, SDL_SetHint,
    SDL_SetRenderDrawBlendMode, SDL_SetRenderDrawColor, SDL_SetWindowModalFor, SDL_Texture,
    SDL_Window, SDL_WindowFlags_SDL_WINDOW_ALLOW_HIGHDPI, SDL_WindowFlags_SDL_WINDOW_BORDERLESS,
    SDL_WindowFlags_SDL_WINDOW_FULLSCREEN, SDL_INIT_VIDEO, SDL_WINDOWPOS_UNDEFINED_MASK,
};

pub struct MySdl {
    pub texture: *mut SDL_Texture,
    pub renderer: *mut SDL_Renderer,
    pub window: *mut SDL_Window,
}

impl MySdl {
    pub fn init_sdl() -> Self {
        unsafe {
            if SDL_Init(SDL_INIT_VIDEO) < 0 {
                panic!("failed to initialize sdl2 with video");
            };

            let window_flags =
                SDL_WindowFlags_SDL_WINDOW_ALLOW_HIGHDPI | SDL_WindowFlags_SDL_WINDOW_BORDERLESS;

            let title = CString::new("Dr K Dos").expect("CString::new failed");

            // let (width, height) = if IS_MOBILE {
            //     let mut x = 0;
            //     let mut y = 0;

            //     SDL_GetWindowSize(window, &mut w, &mut h);
            //     SDL_RenderSetLogicalSize(renderer, w, h);
            // }

            let window = SDL_CreateWindow(
                title.as_ptr(),
                0,
                0,
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
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

            IMG_Init((IMG_InitFlags_IMG_INIT_PNG).try_into().unwrap());

            let file = CString::new("skyline-packer-output.png").unwrap();
            let texture = IMG_LoadTexture(renderer, file.as_ptr());

            SDL_SetRenderDrawBlendMode(renderer, SDL_BlendMode_SDL_BLENDMODE_BLEND);

            MySdl {
                texture,
                renderer,
                window,
            }
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
            // SDL_Delay(32);
        }
    }

    pub fn draw(&self) {
        let rect = SDL_Rect::new(0, SCREEN_HEIGHT - 100, SCREEN_WIDTH, 100);
        unsafe {
            SDL_SetRenderDrawColor(self.renderer, 150, 150, 150, 255);
            SDL_RenderFillRect(self.renderer, &rect);
        }
    }
}
