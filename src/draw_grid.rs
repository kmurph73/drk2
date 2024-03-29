use crate::{
    colors::LINE_COLOR,
    globals::Globals,
    my_sdl::{MySdl, SDL_Rect},
    prelude::{COLS, ROWS, TOPSET},
    Image,
};

pub fn draw_line(sdl: &MySdl, line: &Vec<(i32, i32)>) {
    let w = 1;
    let h = 1;
    sdl.set_draw_color_tuple(LINE_COLOR);

    for (x, y) in line {
        let rect = SDL_Rect { x: *x, y: *y, w, h };
        sdl.fill_rect(&rect);
    }
}

fn draw_rows(
    sdl: &MySdl,
    Globals {
        square_size,
        left_x,
        ..
    }: &Globals,
) {
    let x = *left_x;
    let x2 = (square_size * COLS) + left_x;

    let w = x2 - x;

    let screen_rows = ROWS + 1;

    for i in 2..screen_rows {
        let y = i * square_size + TOPSET;

        sdl.draw_horizontal_line(x, y - 1, w);
    }
}

fn draw_cols(
    sdl: &MySdl,
    Globals {
        square_size,
        left_x,
        ..
    }: &Globals,
) {
    let y = square_size * 2 + TOPSET;
    let y2 = square_size * ROWS + TOPSET;

    let h = y2 - y;

    let cols = COLS + 1;

    for i in 0..cols {
        let x = (square_size * i) + left_x;

        sdl.draw_vertical_line(x, y, h);
    }
}

pub fn draw_grid(sdl: &MySdl, globals: &Globals) {
    sdl.set_draw_color_tuple((40, 40, 40, 255));

    draw_rows(sdl, globals);
    draw_cols(sdl, globals);
}

pub fn draw_menu_btn(sdl: &MySdl, Image { srcrect, dstrect }: &Image) {
    sdl.draw_image(srcrect, dstrect);
}

pub fn draw_images(sdl: &MySdl, imgs: &[Image]) {
    for i in imgs {
        sdl.draw_image(&i.srcrect, &i.dstrect);
    }
}
