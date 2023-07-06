use crate::{
    colors::{SDL_BLACK, SDL_WHITE},
    draw_game::draw_image,
    globals::Globals,
    my_sdl::{MySdl, SDL_Color, SDL_Rect, SDL_RenderFillRect, SDL_SetRenderDrawColor},
    util::get_level_image,
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
    globals: &Globals,
) {
    let srcrect = get_level_image(level);
    let r = &image_buttons[0].dstrect;
    let container = SDL_Rect {
        x: 0,
        y: r.y,
        w: globals.window_width,
        h: r.h,
    };

    let (width, height) = (srcrect.w / 2, srcrect.h / 2);

    let (x, y) = container.center(width, height);

    let dstrect = SDL_Rect::new(x, y, width, height);

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
