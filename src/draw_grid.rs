use crate::{
    my_sdl::{MySdl, SDL_Rect, SDL_RenderCopy, SDL_RenderFillRect, SDL_SetRenderDrawColor},
    prelude::{COLS, ROWS, TOPSET},
    Image,
};

pub fn draw_line(sdl: &MySdl, line: &Vec<(i32, i32)>) {
    let w = 1;
    let h = 1;
    unsafe {
        SDL_SetRenderDrawColor(sdl.renderer, 220, 220, 220, 255);
    }
    for (x, y) in line {
        let rect = SDL_Rect { x: *x, y: *y, w, h };
        unsafe {
            SDL_RenderFillRect(sdl.renderer, &rect);
        }
    }
}

fn draw_rows(sdl: &MySdl, square_size: i32) {
    let x = square_size;
    let x2 = square_size * 9;

    let w = x2 - x;

    let screen_rows = ROWS + 1;

    for i in 2..screen_rows {
        let y = i * square_size + TOPSET;

        sdl.draw_horizontal_line(x, y - 1, w);
    }
}

fn draw_cols(sdl: &MySdl, square_size: i32) {
    let y = square_size * 2 + TOPSET;
    let y2 = square_size * ROWS + TOPSET;

    let h = y2 - y;

    let cols = COLS + 2;

    for i in 1..cols {
        let x = square_size * i;

        sdl.draw_vertical_line(x, y, h);
    }
}

pub fn draw_grid(sdl: &MySdl, square_size: i32) {
    unsafe {
        SDL_SetRenderDrawColor(sdl.renderer, 40, 40, 40, 255);
    }

    draw_rows(sdl, square_size);
    draw_cols(sdl, square_size);
}

pub fn draw_menu_btn(sdl: &MySdl, Image { srcrect, dstrect }: &Image) {
    unsafe {
        SDL_RenderCopy(sdl.renderer, sdl.texture, srcrect, dstrect);
    }
}

pub fn draw_images(sdl: &MySdl, imgs: &[Image]) {
    for i in imgs {
        unsafe {
            SDL_RenderCopy(sdl.renderer, sdl.texture, &i.srcrect, &i.dstrect);
        }
    }
}
