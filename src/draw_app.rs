use crate::{
    dot::Dot,
    my_sdl::{MySdl, SDL_Rect, SDL_RenderCopy},
    pos::Pos,
};

pub fn draw_app(sdl: &MySdl, squares: &[Option<Dot>], square_size: i32) {
    for dot in squares.iter().flatten() {
        let srcrect = dot.img_rect();
        let SDL_Rect { w, h, .. } = srcrect;

        let Pos(x, y) = dot.tile.top_left_px(square_size);

        let dest = SDL_Rect {
            x,
            y,
            w: w / 2,
            h: h / 2,
        };

        unsafe {
            SDL_RenderCopy(sdl.renderer, sdl.texture, &srcrect, &dest);
        }
    }
}
