use crate::{
    dot::Dot,
    globals::Globals,
    img_consts::CONNECTOR_IMG,
    my_sdl::{MySdl, SDL_Rect, SDL_RenderCopy},
    piece::Piece,
    pos::Pos,
    prelude::TOPSET,
    util::tuple_to_rect,
};

pub fn draw_image(srcrect: &SDL_Rect, dstrect: &SDL_Rect, sdl: &MySdl) {
    unsafe {
        SDL_RenderCopy(sdl.renderer, sdl.texture, srcrect, dstrect);
    }
}

fn draw_dot(
    dot: &Dot,
    sdl: &MySdl,
    (offset_x, offset_y): (i32, i32),
    Globals {
        square_size,
        dot_size,
        dotset,
        ..
    }: &Globals,
) {
    let srcrect = dot.img_rect();

    let Pos(x, y) = dot.tile.top_left_px(*square_size);

    let w = *dot_size;
    let h = w;

    let dest = SDL_Rect {
        x: x + 2 + offset_x + *dotset,
        y: y + 2 + TOPSET + offset_y + *dotset,
        w,
        h,
    };

    unsafe {
        SDL_RenderCopy(sdl.renderer, sdl.texture, &srcrect, &dest);
    }
}

fn connector_offset(rotation: i32, square_size: i32, connector_size: i32) -> (i32, i32) {
    let half_conn = connector_size / 2;
    let half_square = square_size / 2;

    let x = square_size - half_conn;
    let y = half_square - half_conn;

    // 0 => (square_size - 5, half_square - 5),

    let (x, y) = match rotation {
        0 => (x + 1, y + 1),
        1 => (half_square - half_conn, -half_conn),
        2 => (-half_conn, y),
        3 => (half_square - half_conn, square_size - half_conn),
        _ => panic!("{rotation} should be 0..3"),
    };

    (x, y)
}

fn draw_connector(
    lhs: &Dot,
    rotation: i32,
    sdl: &MySdl,
    (extra_x_offset, extra_y_offset): (i32, i32),
    Globals {
        square_size,
        connector_size,
        ..
    }: &Globals,
) {
    let square_size = *square_size;
    let connector_size = *connector_size;

    let Pos(x, y) = lhs.tile.top_left_px(square_size);

    let (x_offset, y_offset) = connector_offset(rotation, square_size, connector_size);

    let x = x + x_offset;
    let y = y + y_offset;

    let (_img_x, _img_y, _w, _h) = CONNECTOR_IMG;

    let w = connector_size;
    let h = w;

    let dstrect = SDL_Rect {
        x: x + extra_x_offset,
        y: y + TOPSET + extra_y_offset,
        w,
        h,
    };

    let srcrect = tuple_to_rect(CONNECTOR_IMG);

    unsafe {
        SDL_RenderCopy(sdl.renderer, sdl.texture, &srcrect, &dstrect);
    }
}

pub fn draw_piece_connectors(pieces: &Vec<Piece>, sdl: &MySdl, globals: &Globals) {
    for piece in pieces {
        draw_connector(&piece.lhs, piece.rotation, sdl, (0, 0), globals);
    }
}

pub fn draw_piece_connectors_w_offsets(
    pieces: &Vec<Piece>,
    sdl: &MySdl,
    y_offsets: &[i32],
    globals: &Globals,
) {
    for piece in pieces {
        let (lhs, _rhs) = piece.indexes();
        let y_offset = y_offsets[lhs];

        draw_connector(&piece.lhs, piece.rotation, sdl, (0, y_offset), globals);
    }
}

pub fn draw_piece(piece: &Piece, sdl: &MySdl, offset: (i32, i32), globals: &Globals) {
    draw_dot(&piece.lhs, sdl, offset, globals);
    draw_dot(&piece.rhs, sdl, offset, globals);
    draw_connector(&piece.lhs, piece.rotation, sdl, offset, globals);
}

pub fn draw_dots_w_offsets(
    sdl: &MySdl,
    squares: &[Option<Dot>],
    y_offsets: &[i32],
    globals: &Globals,
) {
    for (idx, dot) in squares.iter().enumerate() {
        let dot = if let Some(dot) = dot {
            dot
        } else {
            continue;
        };

        let offset_y = y_offsets[idx];

        draw_dot(dot, sdl, (0, offset_y), globals);
    }
}

pub fn draw_dots(sdl: &MySdl, squares: &[Option<Dot>], offset: (i32, i32), globals: &Globals) {
    for dot in squares.iter().flatten() {
        draw_dot(dot, sdl, offset, globals);
    }
}
