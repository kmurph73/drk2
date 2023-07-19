use crate::{
    globals::Globals,
    img_consts::{
        MENU_BTN_IMG, MINUS_BTN_IMG, NEW_GAME_BTN_IMG, PLAY_BTN_IMG, PLUS_BTN_IMG, RESUME_BTN_IMG,
    },
    my_sdl::SDL_Rect,
    util::tuple_to_rect,
    ButtonKind, Image, ImageButton,
};

pub fn gen_modal_text(modal: &SDL_Rect, srcrect: SDL_Rect, ratio: f64, square_size: i32) -> Image {
    let dy = modal.y + square_size / 2;

    let (dw, dh) = ((srcrect.w as f64 * ratio), (srcrect.h as f64 * ratio));
    let (dw, dh) = (dw as i32, dh as i32);

    let (dx, _y) = modal.center(dw, dh);

    let dstrect = SDL_Rect::dst_new(dx, dy, dw, dh);

    Image { srcrect, dstrect }
}

pub fn gen_endgame_buttons(globals: &Globals) -> Vec<ImageButton> {
    let mut buttons: Vec<ImageButton> = Vec::with_capacity(3);

    let modal_rect = globals.help_modal;

    let offset_y = globals.square_size;

    let (dw, dh) = globals.button_size;
    let (sx, sy, sw, sh) = MENU_BTN_IMG;

    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);

    let x = (globals.window_width - dw) / 2;
    let y = offset_y + modal_rect.y + globals.square_size * 2;

    let dstrect = SDL_Rect::new(x, y, dw, dh);

    let menu = ImageButton {
        kind: ButtonKind::Menu,
        srcrect,
        dstrect,
    };

    buttons.push(menu);

    let y = y + dh + globals.square_size;

    let (sx, sy, sw, sh) = NEW_GAME_BTN_IMG;
    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);

    let (dw, dh) = globals.button_size;

    // let width = sw / 2;
    let x = (globals.window_width - dw) / 2;

    let y = y + dh + offset_y;
    // let x = 100; // (w - (sw / 2)) / 2;
    let dstrect = SDL_Rect::new(x, y, dw, dh);

    let new_game = ImageButton {
        kind: ButtonKind::NewGame,
        srcrect,
        dstrect,
    };

    buttons.push(new_game);

    buttons
}

pub fn gen_help_buttons(globals: &Globals) -> Vec<ImageButton> {
    let mut buttons: Vec<ImageButton> = Vec::with_capacity(3);

    let modal_rect = globals.help_modal;

    let offset_y = globals.square_size;

    let y = modal_rect.y + offset_y * 3;

    let (sx, sy, sw, sh) = NEW_GAME_BTN_IMG;

    let (dw, dh) = globals.button_size;

    let x = (globals.window_width - dw) / 2;

    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);
    let dstrect = SDL_Rect::new(x, y, dw, dh);

    let new_game = ImageButton {
        kind: ButtonKind::NewGame,
        srcrect,
        dstrect,
    };

    buttons.push(new_game);

    let y = y + offset_y + dh;

    let (sx, sy, sw, sh) = MENU_BTN_IMG;
    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);

    let dstrect = SDL_Rect::new(x, y, dw, dh);

    let menu = ImageButton {
        kind: ButtonKind::Menu,
        srcrect,
        dstrect,
    };

    buttons.push(menu);

    let (sx, sy, sw, sh) = RESUME_BTN_IMG;
    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);

    // let width = sw / 2;

    let (dw, dh) = globals.button_size;

    let y = y + offset_y + dh;
    // let x = 100; // (w - (sw / 2)) / 2;
    let dstrect = SDL_Rect::new(x, y, dw, dh);

    let resume = ImageButton {
        kind: ButtonKind::Resume,
        srcrect,
        dstrect,
    };

    buttons.push(resume);

    buttons
}

pub fn gen_menu_buttons(globals: &Globals) -> Vec<ImageButton> {
    let mut buttons: Vec<ImageButton> = Vec::with_capacity(2);

    let (dw, dh) = globals.button_size;
    let (sx, sy, sw, sh) = PLAY_BTN_IMG;
    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);

    // let width = sw / 2;
    let dx = (globals.window_width - (dw)) / 2;

    let dy = globals.square_size * 2;
    // let x = 100; // (w - (sw / 2)) / 2;
    let dstrect = SDL_Rect::dst_new(dx, dy, dw, dh);

    let new_game = ImageButton {
        kind: ButtonKind::NewGame,
        srcrect,
        dstrect,
    };

    buttons.push(new_game);

    buttons
}

pub fn gen_plus_minus_menu_buttons(y: i32, globals: &Globals) -> Vec<ImageButton> {
    let mut image_buttons: Vec<ImageButton> = Vec::with_capacity(2);
    let ratio = globals.ratio;

    let srcrect = tuple_to_rect(MINUS_BTN_IMG);
    let dw = srcrect.w as f64 * ratio;
    let dw = dw as i32;
    // let w = globals.square_size * 2;
    // let h = w;
    let x_offset = globals.square_size;
    let dstrect = SDL_Rect {
        x: x_offset,
        y,
        w: dw,
        h: dw,
    };

    let btn = ImageButton {
        kind: ButtonKind::LevelDown,
        srcrect,
        dstrect,
    };

    image_buttons.push(btn);

    let srcrect = tuple_to_rect(PLUS_BTN_IMG);
    let dstrect = SDL_Rect {
        x: globals.window_width - x_offset - dw,
        y,
        w: dw,
        h: dw,
    };

    let btn = ImageButton {
        kind: ButtonKind::LevelUp,
        srcrect,
        dstrect,
    };

    image_buttons.push(btn);

    image_buttons
}
