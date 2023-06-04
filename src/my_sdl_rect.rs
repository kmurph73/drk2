use crate::my_sdl::SDL_Rect;

impl SDL_Rect {
    pub fn shrink(&self, n: i32) -> SDL_Rect {
        let SDL_Rect { x, y, w, h } = &self;

        SDL_Rect {
            x: x + n,
            y: y + n,
            w: w - n * 2,
            h: h - n * 2,
        }
    }
}
