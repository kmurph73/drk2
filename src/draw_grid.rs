use crate::{
    my_sdl::{MySdl, SDL_RenderDrawLine, SDL_SetRenderDrawColor},
    prelude::{COLS, ROWS},
};

fn draw_rows(sdl: &MySdl, square_size: i32) {
    let x1 = square_size * 1;
    let x2 = square_size * 9;

    let screen_rows = ROWS + 3;

    for i in 2..screen_rows {
        let y1 = i * square_size;
        let y2 = i * square_size;

        unsafe {
            SDL_RenderDrawLine(sdl.renderer, x1, y1, x2, y2);
        }
    }
}

fn draw_cols(sdl: &MySdl, square_size: i32) {
    let y1 = square_size * 2;
    let y2 = square_size * (ROWS + 2);

    for i in 1..10 {
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
