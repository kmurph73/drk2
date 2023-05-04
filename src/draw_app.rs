use crate::{
    dot::Dot,
    img_consts::CONNECTOR_IMG,
    my_sdl::{MySdl, SDL_Rect, SDL_RenderCopy},
    piece::Piece,
    pos::Pos,
    util::tuple_to_rect,
};

fn draw_dot(dot: &Dot, sdl: &MySdl, square_size: i32, img_divisor: i32) {
    let srcrect = dot.img_rect();
    let SDL_Rect { w, h, .. } = srcrect;

    let Pos(x, y) = dot.tile.top_left_px(square_size);

    let dest = SDL_Rect {
        x: x + 2,
        y: y + 2,
        w: w / img_divisor,
        h: h / img_divisor,
    };

    unsafe {
        SDL_RenderCopy(sdl.renderer, sdl.texture, &srcrect, &dest);
    }
}

fn draw_connector(lhs: &Dot, sdl: &MySdl, square_size: i32, img_divisor: i32) {
    // let half_square = square_size / 2;
    let quarter_square = square_size / 4;

    let Pos(x, y) = lhs.tile.top_left_px(square_size);

    let x = x + square_size - quarter_square + 5;
    let y = y + quarter_square + 5;

    let (_img_x, _img_y, w, h) = CONNECTOR_IMG;

    let dstrect = SDL_Rect {
        x,
        y,
        w: w / img_divisor,
        h: h / img_divisor,
    };

    let srcrect = tuple_to_rect(CONNECTOR_IMG);

    unsafe {
        SDL_RenderCopy(sdl.renderer, sdl.texture, &srcrect, &dstrect);
    }
}

fn draw_piece(piece: &Piece, sdl: &MySdl, square_size: i32, img_divisor: i32) {
    draw_dot(&piece.lhs, sdl, square_size, img_divisor);
    draw_dot(&piece.rhs, sdl, square_size, img_divisor);
    draw_connector(&piece.lhs, sdl, square_size, img_divisor);
}

pub fn draw_app(
    sdl: &MySdl,
    piece: &Piece,
    squares: &[Option<Dot>],
    square_size: i32,
    img_divisor: i32,
) {
    draw_piece(piece, sdl, square_size, img_divisor);

    for dot in squares.iter().flatten() {
        draw_dot(dot, sdl, square_size, img_divisor);
    }
}
