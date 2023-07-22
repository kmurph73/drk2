use crate::{
    colors::{SDL_BLACK, SDL_WHITE},
    draw_game::draw_image,
    globals::Globals,
    img_consts::LEVEL_IMG,
    my_sdl::{
        MySdl, SDL_Color, SDL_Rect, SDL_RenderCopy, SDL_RenderFillRect, SDL_SetRenderDrawColor,
    },
    number_images::NumberImages,
    util::tuple_to_rect,
    Image, ImageButton,
};

pub fn draw_modal(sdl: &MySdl, buttons: &Vec<ImageButton>, txt: &Image, globals: &Globals) {
    unsafe {
        let SDL_Color { r, g, b, .. } = SDL_BLACK;
        SDL_SetRenderDrawColor(sdl.renderer, r, g, b, 150);

        let rect = SDL_Rect {
            x: 0,
            y: 0,
            w: globals.window_width,
            h: globals.window_height,
        };
        SDL_RenderFillRect(sdl.renderer, &rect);

        let SDL_Color { r, g, b, .. } = SDL_WHITE;
        SDL_SetRenderDrawColor(sdl.renderer, r, g, b, 255);

        let rect = globals.help_modal;
        SDL_RenderFillRect(sdl.renderer, &rect);

        let SDL_Color { r, g, b, .. } = SDL_BLACK;
        SDL_SetRenderDrawColor(sdl.renderer, r, g, b, 255);

        let rect = rect.shrink(5);

        SDL_RenderFillRect(sdl.renderer, &rect);

        for ImageButton {
            srcrect, dstrect, ..
        } in buttons
        {
            draw_image(srcrect, dstrect, sdl)
        }

        draw_image(&txt.srcrect, &txt.dstrect, sdl)
    }
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
    let srcrect = SDL_Rect::new(0, 0, 1428, 2000);
    let padding = globals.square_size;
    let half = padding / 2;

    let ratio = (globals.window_width - padding) as f64 / srcrect.w as f64;
    let dw = srcrect.w as f64 * ratio;
    let dh = srcrect.h as f64 * ratio;

    let dstrect = SDL_Rect::new(half, half, dw as i32, dh as i32);

    unsafe {
        SDL_RenderCopy(sdl.renderer, sdl.about_texture, &srcrect, &dstrect);
    }
}
