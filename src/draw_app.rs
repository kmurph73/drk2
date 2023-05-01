use crate::{
    dot::Dot,
    my_sdl::{MySdl, SDL_Rect},
};

pub fn draw_app(sdl: &MySdl, squares: &Vec<Option<Dot>>) {
    for square in squares {
        if let Some(dot) = square {
            let rect = dot.img_rect();
            let SDL_Rect { w, h, .. } = rect;

            // let dest = SDL_Rect {
            //     x: building.rect.x,
            //     y: building.rect.y,
            //     w: w / 2,
            //     h: h / 2,
            // };
        }
    }
    // let pylon_img = tuple_to_rect(PYLON_IMG);
    // let SDL_Rect { w, h, .. } = pylon_img;

    // let dest = SDL_Rect {
    //     x: building.rect.x,
    //     y: building.rect.y,
    //     w: w / 2,
    //     h: h / 2,
    // };

    // SDL_RenderCopy(
    //     sdl.renderer,
    //     sdl.texture,
    //     &pylon_img,
    //     &dest.adjust(camera_offset),
    // );
}
