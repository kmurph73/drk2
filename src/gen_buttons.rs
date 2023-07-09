use crate::{
    globals::Globals,
    img_consts::{
        MENU_BTN_IMG, MINUS_SQUARE_IMG, NEW_GAME_BTN_IMG, PLAY_BTN_IMG, PLUS_SQUARE_IMG,
        RESUME_BTN_IMG,
    },
    my_sdl::{MySdl, SDL_Rect},
    util::tuple_to_rect,
    ButtonKind, Image, ImageButton,
};

pub fn gen_modal_text(modal: &SDL_Rect, srcrect: SDL_Rect) -> Image {
    let y = modal.y + 20;

    let (x, _y) = modal.center(srcrect.w, srcrect.h);

    let dstrect = SDL_Rect::new(x, y, srcrect.w, srcrect.h);

    Image { srcrect, dstrect }
}

pub fn gen_endgame_buttons(globals: &Globals) -> Vec<ImageButton> {
    let mut buttons: Vec<ImageButton> = Vec::with_capacity(3);

    let modal_rect = globals.help_modal;

    let offset_y = 100;

    let (sx, sy, sw, sh) = MENU_BTN_IMG;
    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);

    let x = (globals.window_width - sw) / 2;
    let y = offset_y + modal_rect.y;

    let (dw, dh) = globals.button_size;
    let dstrect = SDL_Rect::new(x, y, dw, dh);

    let menu = ImageButton {
        kind: ButtonKind::Menu,
        srcrect,
        dstrect,
    };

    buttons.push(menu);

    let y = y + offset_y;

    let (sx, sy, sw, sh) = NEW_GAME_BTN_IMG;
    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);

    // let width = sw / 2;
    let x = (globals.window_width - sw) / 2;

    let y = y + offset_y;
    // let x = 100; // (w - (sw / 2)) / 2;
    let dstrect = SDL_Rect::new(x, y, sw, sh);

    let resume = ImageButton {
        kind: ButtonKind::NewGame,
        srcrect,
        dstrect,
    };

    buttons.push(resume);

    buttons
}

pub fn gen_help_buttons(sdl: &MySdl, globals: &Globals) -> Vec<ImageButton> {
    let mut buttons: Vec<ImageButton> = Vec::with_capacity(3);

    let modal_rect = globals.help_modal;

    let offset_y = 100;

    let y = modal_rect.y + offset_y;

    let (sx, sy, sw, sh) = NEW_GAME_BTN_IMG;
    let x = (globals.window_width - sw) / 2;

    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);
    let dstrect = SDL_Rect::new(x, y, sw, sh);

    let new_game = ImageButton {
        kind: ButtonKind::NewGame,
        srcrect,
        dstrect,
    };

    buttons.push(new_game);

    let y = y + offset_y;

    let (sx, sy, sw, sh) = MENU_BTN_IMG;
    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);

    let dstrect = SDL_Rect::new(x, y, sw, sh);

    let menu = ImageButton {
        kind: ButtonKind::Menu,
        srcrect,
        dstrect,
    };

    buttons.push(menu);

    let (sx, sy, sw, sh) = RESUME_BTN_IMG;
    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);

    // let width = sw / 2;

    let y = y + offset_y;
    // let x = 100; // (w - (sw / 2)) / 2;
    let dstrect = SDL_Rect::new(x, y, sw, sh);

    let resume = ImageButton {
        kind: ButtonKind::Resume,
        srcrect,
        dstrect,
    };

    buttons.push(resume);

    buttons
}

pub fn gen_menu_buttons(sdl: &MySdl, globals: &Globals) -> Vec<ImageButton> {
    let mut buttons: Vec<ImageButton> = Vec::with_capacity(2);

    let (sx, sy, sw, sh) = PLAY_BTN_IMG;
    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);

    // let width = sw / 2;
    let x = (globals.window_width - (sw)) / 2;

    let y = 200;
    // let x = 100; // (w - (sw / 2)) / 2;
    let dstrect = SDL_Rect::new(x, y, sw, sh);

    let play = ImageButton {
        kind: ButtonKind::NewGame,
        srcrect,
        dstrect,
    };

    buttons.push(play);

    buttons
}

pub fn gen_plus_minus_menu_buttons(y: i32, globals: &Globals) -> Vec<ImageButton> {
    let mut image_buttons: Vec<ImageButton> = Vec::with_capacity(2);

    let srcrect = tuple_to_rect(MINUS_SQUARE_IMG);
    let w = globals.square_size * 2;
    let h = w;
    let x_offset = 10;
    let dstrect = SDL_Rect {
        x: x_offset,
        y,
        w,
        h,
    };

    let btn = ImageButton {
        kind: ButtonKind::LevelDown,
        srcrect,
        dstrect,
    };

    image_buttons.push(btn);

    let srcrect = tuple_to_rect(PLUS_SQUARE_IMG);
    let dstrect = SDL_Rect {
        x: globals.window_width - x_offset - w,
        y,
        w,
        h,
    };

    let btn = ImageButton {
        kind: ButtonKind::LevelUp,
        srcrect,
        dstrect,
    };

    image_buttons.push(btn);

    image_buttons
}
