use crate::{
    dot::Dot,
    img_consts::CONNECTOR_IMG,
    my_sdl::{MySdl, SDL_Rect, SDL_RenderCopy},
    piece::Piece,
    pos::Pos,
    prelude::TOPSET,
    util::tuple_to_rect,
};

pub fn draw_image(srcrect: &SDL_Rect, dstrect: &SDL_Rect, sdl: &MySdl) {
    // let SDL_Rect { w, h, .. } = srcrect;

    // let dest = SDL_Rect {
    //     x,
    //     y,
    //     w: w / 2,
    //     h: h / 2,
    // };

    unsafe {
        SDL_RenderCopy(sdl.renderer, sdl.texture, srcrect, dstrect);
    }
}

fn draw_dot(dot: &Dot, sdl: &MySdl, square_size: i32, img_divisor: i32, offset_y: i32) {
    let srcrect = dot.img_rect();
    let SDL_Rect { w, h, .. } = srcrect;

    let Pos(x, y) = dot.tile.top_left_px(square_size);

    let dest = SDL_Rect {
        x: x + 2,
        y: y + 2 + TOPSET + offset_y,
        w: w / img_divisor,
        h: h / img_divisor,
    };

    unsafe {
        SDL_RenderCopy(sdl.renderer, sdl.texture, &srcrect, &dest);
    }
}

fn connector_offset(rotation: i32, square_size: i32) -> (i32, i32) {
    let n = 5;
    let half_square = square_size / 2;
    let x = square_size - n;
    let y = half_square - n;

    // 0 => (square_size - 5, half_square - 5),

    let (x, y) = match rotation {
        0 => (x, y),
        1 => (half_square - n, -n),
        2 => (-n, y),
        3 => (half_square - n, square_size - n),
        _ => panic!("{rotation} should be 0..3"),
    };

    (x, y)
}

fn draw_connector(lhs: &Dot, rotation: i32, sdl: &MySdl, square_size: i32, img_divisor: i32) {
    // let half_square = square_size / 2;
    // let quarter_square = square_size / 4;

    let Pos(x, y) = lhs.tile.top_left_px(square_size);

    let (x_offset, y_offset) = connector_offset(rotation, square_size);

    let x = x + x_offset;
    let y = y + y_offset;

    let (_img_x, _img_y, w, h) = CONNECTOR_IMG;

    let dstrect = SDL_Rect {
        x,
        y: y + TOPSET,
        w: w / img_divisor,
        h: h / img_divisor,
    };

    let srcrect = tuple_to_rect(CONNECTOR_IMG);

    unsafe {
        SDL_RenderCopy(sdl.renderer, sdl.texture, &srcrect, &dstrect);
    }
}

pub fn draw_piece_connectors(pieces: &Vec<Piece>, sdl: &MySdl, square_size: i32, img_divisor: i32) {
    for piece in pieces {
        draw_connector(&piece.lhs, piece.rotation, sdl, square_size, img_divisor);
    }
}

pub fn draw_piece(piece: &Piece, sdl: &MySdl, square_size: i32, img_divisor: i32) {
    draw_dot(&piece.lhs, sdl, square_size, img_divisor, 0);
    draw_dot(&piece.rhs, sdl, square_size, img_divisor, 0);
    draw_connector(&piece.lhs, piece.rotation, sdl, square_size, img_divisor);
}

pub fn draw_dots_w_offsets(
    sdl: &MySdl,
    squares: &[Option<Dot>],
    square_size: i32,
    img_divisor: i32,
    y_offsets: &[i32],
) {
    for (idx, dot) in squares.iter().enumerate() {
        let dot = if let Some(dot) = dot {
            dot
        } else {
            continue;
        };

        let offset_y = y_offsets[idx];

        draw_dot(dot, sdl, square_size, img_divisor, offset_y);
    }
}

pub fn draw_dots(sdl: &MySdl, squares: &[Option<Dot>], square_size: i32, img_divisor: i32) {
    for dot in squares.iter().flatten() {
        draw_dot(dot, sdl, square_size, img_divisor, 0);
    }
}
