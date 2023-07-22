use crate::{
    my_sdl::{
        MySdl, SDL_Rect, SDL_RenderCopy, SDL_RenderDrawLine, SDL_RenderFillRect,
        SDL_SetRenderDrawColor,
    },
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
    let x1 = square_size;
    let x2 = square_size * 9;

    let screen_rows = ROWS + 1;

    for i in 2..screen_rows {
        let y1 = i * square_size + TOPSET;
        let y2 = i * square_size + TOPSET;

        unsafe {
            SDL_RenderDrawLine(sdl.renderer, x1, y1, x2, y2);
        }
    }
}

fn draw_cols(sdl: &MySdl, square_size: i32) {
    let y1 = square_size * 2 + TOPSET;
    let y2 = square_size * ROWS + TOPSET;

    let cols = COLS + 2;

    for i in 1..cols {
        let x1 = square_size * i;
        let x2 = square_size * i;

        unsafe {
            SDL_RenderDrawLine(sdl.renderer, x1, y1, x2, y2);
        }
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
