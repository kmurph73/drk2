use crate::{
    colors::{SDL_BLACK, SDL_WHITE},
    draw_game::draw_image,
    globals::Globals,
    img_consts::LEVEL_IMG,
    my_sdl::{MySdl, SDL_Rect},
    number_images::NumberImages,
    util::tuple_to_rect,
    Image, ImageButton,
};

pub fn draw_modal(sdl: &MySdl, buttons: &Vec<ImageButton>, txt: &Image, globals: &Globals) {
    sdl.set_draw_color(SDL_BLACK);

    let rect = SDL_Rect {
        x: 0,
        y: 0,
        w: globals.window_width,
        h: globals.window_height,
    };
    sdl.fill_rect(&rect);

    sdl.set_draw_color(SDL_WHITE);

    let rect = globals.help_modal;
    sdl.fill_rect(&rect);

    sdl.set_draw_color(SDL_BLACK);

    let rect = rect.shrink(5);
    sdl.fill_rect(&rect);

    for ImageButton {
        srcrect, dstrect, ..
    } in buttons
    {
        draw_image(srcrect, dstrect, sdl)
    }

    draw_image(&txt.srcrect, &txt.dstrect, sdl)
}

pub fn draw_menu(
    sdl: &MySdl,
    buttons: &Vec<ImageButton>,
    image_buttons: &Vec<ImageButton>,
    level: usize,
    number_images: &NumberImages,
    globals: &Globals,
) {
    let (srcrect, dstrect) = number_images.get_level_image(level);

    draw_image(srcrect, dstrect, sdl);

    let srcrect = tuple_to_rect(LEVEL_IMG);

    let dy = dstrect.y - globals.square_size / 2;
    let dw = (globals.ratio * srcrect.w as f64) as i32;
    let dx = (globals.window_width - dw) / 2;
    let dh = (globals.ratio * srcrect.h as f64) as i32;

    let dstrect = SDL_Rect::dst_new(dx, dy, dw, dh);

    draw_image(&srcrect, &dstrect, sdl);

    for ImageButton {
        srcrect, dstrect, ..
    } in buttons
    {
        draw_image(srcrect, dstrect, sdl);
    }

    for ImageButton {
        srcrect, dstrect, ..
    } in image_buttons
    {
        draw_image(srcrect, dstrect, sdl);
    }
}

pub fn draw_about(sdl: &MySdl, globals: &Globals) {
    let srcrect = SDL_Rect::new(0, 0, 1428, 2246);
    let padding = globals.square_size;
    let half_padding = padding / 2;

    let screen_width = globals.window_width - padding;
    let screen_height = globals.window_height - padding;

    let ratio = screen_width as f64 / srcrect.w as f64;
    let dw = srcrect.w as f64 * ratio;
    let dh = srcrect.h as f64 * ratio;

    let dstrect = if dh as i32 >= screen_height {
        let ratio = screen_height as f64 / srcrect.h as f64;
        let dw = srcrect.w as f64 * ratio;
        let dh = srcrect.h as f64 * ratio;

        SDL_Rect::new(half_padding, half_padding, dw as i32, dh as i32)
    } else {
        SDL_Rect::new(half_padding, half_padding, dw as i32, dh as i32)
    };

    sdl.draw_about_image(&srcrect, &dstrect);
}
