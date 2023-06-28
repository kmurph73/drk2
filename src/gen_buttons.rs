use crate::{
    img_consts::{
        MENU_BTN_IMG, MINUS_SQUARE_IMG, NEW_GAME_BTN_IMG, PLAY_BTN_IMG, PLUS_SQUARE_IMG,
        RESUME_BTN_IMG,
    },
    my_sdl::SDL_Rect,
    prelude::{HELP_MODAL, IMG_DIVISOR, SCREEN_WIDTH},
    util::tuple_to_rect,
    ButtonKind, Image, ImageButton,
};

pub fn gen_modal_text(modal: &SDL_Rect, srcrect: SDL_Rect) -> Image {
    let y = modal.y + 20;

    let (x, _y) = modal.center(srcrect.w / IMG_DIVISOR, srcrect.h / IMG_DIVISOR);

    let dstrect = SDL_Rect::new(x, y, srcrect.w / IMG_DIVISOR, srcrect.h / IMG_DIVISOR);

    Image { srcrect, dstrect }
}

pub fn gen_endgame_buttons() -> Vec<ImageButton> {
    let mut buttons: Vec<ImageButton> = Vec::with_capacity(3);

    let modal_rect = tuple_to_rect(HELP_MODAL);

    let offset_y = 100;

    let (sx, sy, sw, sh) = MENU_BTN_IMG;
    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);

    let x = (SCREEN_WIDTH - (sw / IMG_DIVISOR)) / 2;
    let y = offset_y + modal_rect.y;

    let dstrect = SDL_Rect::new(x, y, sw / IMG_DIVISOR, sh / IMG_DIVISOR);

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
    let x = (SCREEN_WIDTH - (sw / IMG_DIVISOR)) / 2;

    let y = y + offset_y;
    // let x = 100; // (w - (sw / 2)) / 2;
    let dstrect = SDL_Rect::new(x, y, sw / IMG_DIVISOR, sh / IMG_DIVISOR);

    let resume = ImageButton {
        kind: ButtonKind::NewGame,
        srcrect,
        dstrect,
    };

    buttons.push(resume);

    buttons
}

pub fn gen_help_buttons() -> Vec<ImageButton> {
    let mut buttons: Vec<ImageButton> = Vec::with_capacity(3);

    let modal_rect = tuple_to_rect(HELP_MODAL);

    let offset_y = 100;

    let y = modal_rect.y + offset_y;

    let (sx, sy, sw, sh) = NEW_GAME_BTN_IMG;
    let x = (SCREEN_WIDTH - (sw / IMG_DIVISOR)) / 2;

    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);
    let dstrect = SDL_Rect::new(x, y, sw / IMG_DIVISOR, sh / IMG_DIVISOR);

    let new_game = ImageButton {
        kind: ButtonKind::NewGame,
        srcrect,
        dstrect,
    };

    buttons.push(new_game);

    let y = y + offset_y;

    let (sx, sy, sw, sh) = MENU_BTN_IMG;
    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);

    let dstrect = SDL_Rect::new(x, y, sw / IMG_DIVISOR, sh / IMG_DIVISOR);

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
    let dstrect = SDL_Rect::new(x, y, sw / IMG_DIVISOR, sh / IMG_DIVISOR);

    let resume = ImageButton {
        kind: ButtonKind::Resume,
        srcrect,
        dstrect,
    };

    buttons.push(resume);

    buttons
}

pub fn gen_menu_buttons() -> Vec<ImageButton> {
    let mut buttons: Vec<ImageButton> = Vec::with_capacity(2);

    let (sx, sy, sw, sh) = PLAY_BTN_IMG;
    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);

    // let width = sw / 2;
    let x = (SCREEN_WIDTH - (sw / IMG_DIVISOR)) / 2;

    let y = 200;
    // let x = 100; // (w - (sw / 2)) / 2;
    let dstrect = SDL_Rect::new(x, y, sw / IMG_DIVISOR, sh / IMG_DIVISOR);

    let play = ImageButton {
        kind: ButtonKind::NewGame,
        srcrect,
        dstrect,
    };

    buttons.push(play);

    buttons
}

pub fn gen_plus_minus_menu_buttons(y: i32) -> Vec<ImageButton> {
    let mut image_buttons: Vec<ImageButton> = Vec::with_capacity(2);

    let srcrect = tuple_to_rect(MINUS_SQUARE_IMG);
    let SDL_Rect { w, h, .. } = srcrect;
    let x_offset = 10;
    let dstrect = SDL_Rect {
        x: x_offset,
        y,
        w: w / IMG_DIVISOR,
        h: h / IMG_DIVISOR,
    };

    let btn = ImageButton {
        kind: ButtonKind::LevelDown,
        srcrect,
        dstrect,
    };

    image_buttons.push(btn);

    let srcrect = tuple_to_rect(PLUS_SQUARE_IMG);
    let SDL_Rect { w, h, .. } = srcrect;
    let dstrect = SDL_Rect {
        x: SCREEN_WIDTH - x_offset - (w / IMG_DIVISOR),
        y,
        w: w / IMG_DIVISOR,
        h: h / IMG_DIVISOR,
    };

    let btn = ImageButton {
        kind: ButtonKind::LevelUp,
        srcrect,
        dstrect,
    };

    image_buttons.push(btn);

    image_buttons
}
