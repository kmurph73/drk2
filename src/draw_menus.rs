use crate::{
    colors::{SDL_BLACK, SDL_WHITE},
    draw_game::draw_image,
    my_sdl::{MySdl, SDL_Color, SDL_Rect, SDL_RenderFillRect, SDL_SetRenderDrawColor},
    prelude::{HELP_MODAL, SCREEN_HEIGHT, SCREEN_WIDTH},
    util::get_level_image,
    ImageButton,
};

pub fn draw_modal(sdl: &MySdl, buttons: &Vec<ImageButton>) {
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

        for ImageButton {
            srcrect, dstrect, ..
        } in buttons
        {
            draw_image(srcrect, dstrect, sdl)
        }
    }
}

pub fn draw_menu(
    sdl: &MySdl,
    buttons: &Vec<ImageButton>,
    image_buttons: &Vec<ImageButton>,
    level: usize,
) {
    let srcrect = get_level_image(level);
    let r = &image_buttons[0].dstrect;
    let container = SDL_Rect {
        x: 0,
        y: r.y,
        w: SCREEN_WIDTH,
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
