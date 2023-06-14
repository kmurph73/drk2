use std::ffi::c_int;
use std::{
    ffi::{CStr, CString},
    ptr,
};

use crate::colors::{SDL_BLACK, SDL_WHITE};
use crate::TextButton;

use crate::prelude::{SCREEN_HEIGHT, SCREEN_WIDTH, SQUARE_SIZE};

#[allow(clippy::all)]
#[allow(warnings, unused)]
mod bindings {
    include!("../bindings.rs");
}

#[allow(clippy::all)]
pub use self::bindings::{
    IMG_Init, IMG_InitFlags_IMG_INIT_JPG, IMG_InitFlags_IMG_INIT_PNG, IMG_LoadTexture,
    SDL_BlendMode_SDL_BLENDMODE_BLEND, SDL_Color, SDL_CreateRenderer, SDL_CreateTextureFromSurface,
    SDL_CreateWindow, SDL_Delay, SDL_DestroyRenderer, SDL_DestroyWindow, SDL_Event,
    SDL_EventType_SDL_KEYDOWN, SDL_EventType_SDL_KEYUP, SDL_EventType_SDL_MOUSEBUTTONDOWN,
    SDL_EventType_SDL_MOUSEBUTTONUP, SDL_EventType_SDL_MOUSEMOTION, SDL_EventType_SDL_QUIT,
    SDL_FreeSurface, SDL_GetError, SDL_Init, SDL_PollEvent, SDL_QueryTexture, SDL_Quit, SDL_Rect,
    SDL_RenderClear, SDL_RenderCopy, SDL_RenderDrawLine, SDL_RenderDrawRect, SDL_RenderFillRect,
    SDL_RenderPresent, SDL_RenderSetScale, SDL_Renderer,
    SDL_RendererFlags_SDL_RENDERER_ACCELERATED, SDL_RendererFlags_SDL_RENDERER_PRESENTVSYNC,
    SDL_Scancode_SDL_SCANCODE_ESCAPE, SDL_SetHint, SDL_SetRenderDrawBlendMode,
    SDL_SetRenderDrawColor, SDL_SetWindowModalFor, SDL_Texture, SDL_Window,
    SDL_WindowFlags_SDL_WINDOW_ALLOW_HIGHDPI, TTF_Init, TTF_OpenFont, TTF_RenderUTF8_Blended,
    TTF_SizeText, TTF_SizeUTF8, _TTF_Font, SDL_INIT_VIDEO, SDL_WINDOWPOS_UNDEFINED_MASK,
};

pub struct MySdl {
    pub texture: *mut SDL_Texture,
    pub renderer: *mut SDL_Renderer,
    pub window: *mut SDL_Window,
    pub font: *mut _TTF_Font,
    pub huge_font: *mut _TTF_Font,
}

impl MySdl {
    pub fn init_sdl(is_mac: bool) -> Self {
        println!("ss: {SQUARE_SIZE}");
        unsafe {
            if SDL_Init(SDL_INIT_VIDEO) < 0 {
                panic!("failed to initialize sdl2 with video");
            };

            if TTF_Init() < 0 {
                let err = SDL_GetError();
                let str = CStr::from_ptr(err as *const _).to_str().unwrap().to_owned();
                panic!("Couldn't initialize SDL TTF: {str}");
            }

            let window_flags = SDL_WindowFlags_SDL_WINDOW_ALLOW_HIGHDPI;

            let title = CString::new("Dr K Dos").expect("CString::new failed");

            let window = SDL_CreateWindow(
                title.as_ptr(),
                SDL_WINDOWPOS_UNDEFINED_MASK as i32,
                SDL_WINDOWPOS_UNDEFINED_MASK as i32,
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                window_flags,
            );

            let linear = CString::new("linear").unwrap();
            let quality = CString::new("SDL_RENDER_SCALE_QUALITY").unwrap();
            SDL_SetHint(quality.as_ptr(), linear.as_ptr());

            let renderer_flags = SDL_RendererFlags_SDL_RENDERER_PRESENTVSYNC;

            let renderer = SDL_CreateRenderer(window, -1, renderer_flags);
            let scale = if is_mac { 2.0 } else { 1.0 };
            SDL_RenderSetScale(renderer, scale, scale);

            IMG_Init(
                (IMG_InitFlags_IMG_INIT_PNG | IMG_InitFlags_IMG_INIT_JPG)
                    .try_into()
                    .unwrap(),
            );

            let file = CString::new("resources/skyline-packer-output.png").unwrap();
            let texture = IMG_LoadTexture(renderer, file.as_ptr());

            let font_path = CString::new("font/EnterCommand.ttf").expect("CString::new failed");
            let font = TTF_OpenFont(font_path.as_ptr(), 28 as c_int);

            if font.is_null() {
                let err = SDL_GetError();
                let str = CStr::from_ptr(err as *const _).to_str().unwrap().to_owned();
                panic!("{str}");
            }

            let huge_font = TTF_OpenFont(font_path.as_ptr(), 60 as c_int);

            if huge_font.is_null() {
                let err = SDL_GetError();
                let str = CStr::from_ptr(err as *const _).to_str().unwrap().to_owned();
                panic!("{str}");
            }

            SDL_SetRenderDrawBlendMode(renderer, SDL_BlendMode_SDL_BLENDMODE_BLEND);

            MySdl {
                font,
                texture,
                renderer,
                window,
                huge_font,
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

    pub fn draw_fps(&self, fps: u64) {
        let str = format!("{:#?}", fps);
        let text = CString::new(str).expect("CString::new failed");
        let texture = self.get_text(text.as_ptr());
        let x = 0;
        let y = 0;
        self.blit(texture, x, y);
    }

    pub fn draw_victory_text(&self) {
        let str = String::from("VICTORY! PLAY AGAIN? [y/n]");
        let text = CString::new(str).expect("CString::new failed");
        let texture = self.get_text(text.as_ptr());
        let x = SCREEN_WIDTH / 4;
        let y = SQUARE_SIZE;
        self.blit(texture, x, y);
    }

    pub fn draw_defeat_text(&self) {
        let str = String::from("DEFEAT! PLAY AGAIN OR YOU HAD ENOUGH? [y/n]");
        let text = CString::new(str).expect("CString::new failed");
        let texture = self.get_text(text.as_ptr());
        let x = SCREEN_WIDTH / 4;
        let y = SQUARE_SIZE / 2;
        self.blit(texture, x, y);
    }

    pub fn draw_num_bad_guys(&self, n: usize) {
        let str = format!("{:#?}", n);
        let text = CString::new(str).expect("CString::new failed");
        let texture = self.get_text(text.as_ptr());
        let x = 0;
        let y = 0;
        self.blit(texture, x, y);
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn get_text(&self, text: *const i8) -> *mut SDL_Texture {
        let font = self.font;

        let color = SDL_WHITE;

        unsafe {
            let surface = TTF_RenderUTF8_Blended(font, text, color);
            let texture = SDL_CreateTextureFromSurface(self.renderer, surface);
            SDL_FreeSurface(surface);
            texture
        }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn blit(&self, texture: *mut SDL_Texture, x: i32, y: i32) {
        let mut dest = SDL_Rect { x, y, w: 0, h: 0 };
        unsafe {
            SDL_QueryTexture(
                texture,
                ptr::null_mut(),
                ptr::null_mut(),
                &mut dest.w,
                &mut dest.h,
            );

            SDL_RenderCopy(self.renderer, texture, ptr::null(), &dest);
        }
    }

    pub fn draw_button(&self, button: &TextButton) {
        unsafe {
            let SDL_Color { r, g, b, .. } = SDL_WHITE;
            SDL_SetRenderDrawColor(self.renderer, r, g, b, 255);

            SDL_RenderFillRect(self.renderer, &button.rect);

            let SDL_Color { r, g, b, .. } = SDL_BLACK;
            SDL_SetRenderDrawColor(self.renderer, r, g, b, 255);

            let rect = button.rect.shrink(2);

            SDL_RenderFillRect(self.renderer, &rect);

            let texture = self.get_text(button.text.as_ptr());

            let (x, y) = button.text_pos;

            self.blit(texture, x, y);
        }
    }

    pub fn get_text_size(&self, text: &CString) -> (i32, i32) {
        let mut w = 0;
        let mut h = 0;
        unsafe {
            TTF_SizeUTF8(self.font, text.as_ptr(), &mut w, &mut h);
        }

        (w, h)
    }
}
