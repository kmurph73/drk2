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

    pub fn center(&self, width: i32, height: i32) -> (i32, i32) {
        let SDL_Rect { x, y, w, h } = *self;

        let x2 = x + ((w - width) / 2);
        let y2 = y + ((h - height) / 2);

        (x2, y2)
    }

    // https://stackoverflow.com/a/40687799/548170
    pub fn within(&self, x: i32, y: i32) -> bool {
        self.x <= x && x < self.x + self.w && self.y <= y && y < self.y + self.h
    }
}
