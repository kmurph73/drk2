use crate::my_sdl::{SDL_Color, SDL_Rect};

impl SDL_Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> SDL_Rect {
        SDL_Rect { x, y, w, h }
    }

    pub fn src_new(sx: i32, sy: i32, sw: i32, sh: i32) -> SDL_Rect {
        SDL_Rect {
            x: sx,
            y: sy,
            w: sw,
            h: sh,
        }
    }

    pub fn dst_new(dx: i32, dy: i32, dw: i32, dh: i32) -> SDL_Rect {
        SDL_Rect {
            x: dx,
            y: dy,
            w: dw,
            h: dh,
        }
    }

    pub fn shrink(&self, n: i32) -> SDL_Rect {
        let SDL_Rect { x, y, w, h } = &self;

        SDL_Rect {
            x: x + n,
            y: y + n,
            w: w - n * 2,
            h: h - n * 2,
        }
    }

    pub fn top_right(&self) -> (i32, i32) {
        (self.x + self.w, self.y)
    }

    pub fn center(&self, width: i32, height: i32) -> (i32, i32) {
        let SDL_Rect { x, y, w, h } = *self;

        let x2 = x + ((w - width) / 2);
        let y2 = y + ((h - height) / 2);

        (x2, y2)
    }

    // https://stackoverflow.com/a/40687799/548170
    pub fn contains(&self, x: i32, y: i32) -> bool {
        self.x <= x && x < self.x + self.w && self.y <= y && y < self.y + self.h
    }
}

impl SDL_Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> SDL_Color {
        SDL_Color { r, g, b, a }
    }
}
