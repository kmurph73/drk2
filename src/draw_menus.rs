use std::ffi::CString;

use crate::{
    colors::{SDL_BLACK, SDL_WHITE},
    draw_game::draw_image,
    my_sdl::{MySdl, SDL_Color, SDL_Rect, SDL_RenderFillRect, SDL_SetRenderDrawColor},
    prelude::{HELP_MODAL, SCREEN_HEIGHT, SCREEN_WIDTH},
    ImageButton, TextButton,
};

pub fn draw_modal(sdl: &MySdl, buttons: &Vec<TextButton>, title: String) {
    unsafe {
        let SDL_Color { r, g, b, .. } = SDL_BLACK;
        SDL_SetRenderDrawColor(sdl.renderer, r, g, b, 150);

        let rect = SDL_Rect {
            x: 0,
            y: 0,
            w: SCREEN_WIDTH,
            h: SCREEN_HEIGHT,
        };
        SDL_RenderFillRect(sdl.renderer, &rect);

        let SDL_Color { r, g, b, .. } = SDL_WHITE;
        SDL_SetRenderDrawColor(sdl.renderer, r, g, b, 255);

        let (x, y, w, h) = HELP_MODAL;
        let rect = SDL_Rect { x, y, w, h };
        SDL_RenderFillRect(sdl.renderer, &rect);

        let SDL_Color { r, g, b, .. } = SDL_BLACK;
        SDL_SetRenderDrawColor(sdl.renderer, r, g, b, 255);

        let rect = rect.shrink(5);

        SDL_RenderFillRect(sdl.renderer, &rect);

        let text = CString::new(title).expect("CString::new failed");

        let texture = sdl.get_text(text.as_ptr());
        let x = rect.x + 140;
        let y = rect.y + 5;
        sdl.blit(texture, x, y);

        for button in buttons {
            sdl.draw_button(button);
        }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn draw_menu(
    sdl: &MySdl,
    buttons: &Vec<TextButton>,
    image_buttons: &Vec<ImageButton>,
    level: usize,
) {
    let str = String::from("Dr. Kodama");
    let text = CString::new(str).expect("CString::new failed");

    let (width, height) = sdl.get_text_size(&text);

    let rect = SDL_Rect {
        x: 0,
        y: 0,
        w: SCREEN_WIDTH,
        h: SCREEN_HEIGHT,
    };

    let (x, _y) = rect.center(width, height);

    let texture = sdl.get_text(text.as_ptr());
    sdl.blit(texture, x, 20);

    let y = buttons[0].rect.y;

    let text = format!("LEVEL: {}", level);
    let text = CString::new(text).expect("CString::new failed");
    let (width, height) = sdl.get_text_size(&text);
    let (x, _y) = rect.center(width, height);
    let texture = sdl.get_text(text.as_ptr());
    let y = y + 110;
    sdl.blit(texture, x, y);

    for button in buttons {
        sdl.draw_button(button);
    }

    for ImageButton {
        srcrect, dstrect, ..
    } in image_buttons
    {
        draw_image(srcrect, dstrect, sdl);
    }
}
