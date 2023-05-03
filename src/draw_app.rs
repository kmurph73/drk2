use crate::{
    dot::Dot,
    my_sdl::{MySdl, SDL_Rect, SDL_RenderCopy},
    piece::Piece,
    pos::Pos,
};

fn draw_dot(dot: &Dot, sdl: &MySdl, square_size: i32, img_divisor: i32) {
    let srcrect = dot.img_rect();
    let SDL_Rect { w, h, .. } = srcrect;

    let Pos(x, y) = dot.tile.top_left_px(square_size);

    let dest = SDL_Rect {
        x,
        y,
        w: w / img_divisor,
        h: h / img_divisor,
    };

    unsafe {
        SDL_RenderCopy(sdl.renderer, sdl.texture, &srcrect, &dest);
    }
}

pub fn draw_app(
    sdl: &MySdl,
    piece: &Piece,
    squares: &[Option<Dot>],
    square_size: i32,
    img_divisor: i32,
) {
    draw_dot(&piece.lhs, sdl, square_size, img_divisor);
    draw_dot(&piece.rhs, sdl, square_size, img_divisor);

    for dot in squares.iter().flatten() {
        draw_dot(dot, sdl, square_size, img_divisor);
    }
}
