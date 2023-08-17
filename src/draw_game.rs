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

fn draw_dot(dot: &Dot, sdl: &MySdl, (offset_x, offset_y): (i32, i32), globals: &Globals) {
    let Globals {
        dot_size, dotset, ..
    } = globals;

    let srcrect = dot.img_rect();

    let Pos(x, y) = dot.tile.top_left_px(globals);

    let w = *dot_size;
    let h = w;

    let dest = SDL_Rect {
        x: x + offset_x + *dotset,
        y: y + TOPSET + offset_y + *dotset,
        w,
        h,
    };

    unsafe {
        SDL_RenderCopy(sdl.renderer, sdl.texture, &srcrect, &dest);
    }
}

fn draw_connector(
    piece: &Piece,
    sdl: &MySdl,
    (extra_x_offset, extra_y_offset): (i32, i32),
    globals: &Globals,
) {
    let (x, y) = piece.center_connector(globals, extra_x_offset, extra_y_offset);

    let dstrect = SDL_Rect {
        x,
        y,
        w: globals.connector_size,
        h: globals.connector_size,
    };

    let srcrect = tuple_to_rect(CONNECTOR_IMG);

    unsafe {
        SDL_RenderCopy(sdl.renderer, sdl.texture, &srcrect, &dstrect);
    }
}

pub fn draw_piece_connectors(pieces: &Vec<Piece>, sdl: &MySdl, globals: &Globals) {
    for piece in pieces {
        draw_connector(piece, sdl, (0, 0), globals);
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

        draw_connector(piece, sdl, (0, y_offset), globals);
    }
}

pub fn draw_piece(piece: &Piece, sdl: &MySdl, offset: (i32, i32), globals: &Globals) {
    draw_dot(&piece.lhs, sdl, offset, globals);
    draw_dot(&piece.rhs, sdl, offset, globals);
    draw_connector(piece, sdl, offset, globals);
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
