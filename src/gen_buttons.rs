use std::ffi::CString;

use crate::{
    img_consts::{MINUS_SQUARE_IMG, PLAY_BTN_IMG, PLUS_SQUARE_IMG, QUIT_BTN_IMG, RESUME_BTN_IMG},
    my_sdl::{MySdl, SDL_Rect},
    prelude::{HELP_MODAL, SCREEN_WIDTH, SQUARE_SIZE},
    util::tuple_to_rect,
    ButtonKind, ImageButton, TextButton,
};

pub fn gen_help_buttons(sdl: &MySdl) -> Vec<TextButton> {
    let mut buttons: Vec<TextButton> = Vec::with_capacity(3);

    let (x, y, w, h) = HELP_MODAL;
    let modal_rect = SDL_Rect { x, y, w, h };

    let offset_y = 90;

    let y = modal_rect.y + offset_y;
    let x = modal_rect.x + SQUARE_SIZE;
    let w = modal_rect.w - SQUARE_SIZE * 2;
    let h = 60;

    let text = String::from("RESUME");
    let text = CString::new(text).expect("CString::new failed");
    let (width, height) = sdl.get_text_size(&text);
    let rect = SDL_Rect { x, y, w, h };

    let resume = TextButton {
        kind: ButtonKind::Resume,
        text,
        rect,
        text_pos: rect.center(width, height),
    };

    buttons.push(resume);

    let y = y + offset_y;

    let rect = SDL_Rect { x, y, w, h };

    let text = String::from("NEW GAME");
    let text = CString::new(text).expect("CString::new failed");
    let (width, height) = sdl.get_text_size(&text);
    let new_game = TextButton {
        kind: ButtonKind::NewGame,
        text,
        rect,
        text_pos: rect.center(width, height),
    };

    buttons.push(new_game);

    let y = y + offset_y;

    let rect = SDL_Rect { x, y, w, h };

    let text = String::from("MENU");
    let text = CString::new(text).expect("CString::new failed");
    let (width, height) = sdl.get_text_size(&text);

    let new_game = TextButton {
        kind: ButtonKind::Menu,
        text,
        rect,
        text_pos: rect.center(width, height),
    };

    buttons.push(new_game);

    let y = y + offset_y;

    let rect = SDL_Rect { x, y, w, h };

    let text = String::from("F THIS GAME");
    let text = CString::new(text).expect("CString::new failed");
    let (width, height) = sdl.get_text_size(&text);

    let new_game = TextButton {
        kind: ButtonKind::Quit,
        text,
        rect,
        text_pos: rect.center(width, height),
    };

    buttons.push(new_game);

    buttons
}

pub fn gen_endgame_buttons(sdl: &MySdl) -> Vec<TextButton> {
    let mut buttons: Vec<TextButton> = Vec::with_capacity(2);

    let (x, y, w, h) = HELP_MODAL;
    let modal_rect = SDL_Rect { x, y, w, h };

    let offset_y = 90;

    let y = modal_rect.y + offset_y;
    let x = modal_rect.x + SQUARE_SIZE;
    let w = modal_rect.w - SQUARE_SIZE * 2;
    let h = 60;

    let text = String::from("PLAY AGAIN");
    let text = CString::new(text).expect("CString::new failed");
    let (width, height) = sdl.get_text_size(&text);
    let rect = SDL_Rect { x, y, w, h };

    let resume = TextButton {
        kind: ButtonKind::NewGame,
        text,
        rect,
        text_pos: rect.center(width, height),
    };

    buttons.push(resume);

    let y = y + offset_y;

    let rect = SDL_Rect { x, y, w, h };

    let text = String::from("MENU");
    let text = CString::new(text).expect("CString::new failed");
    let (width, height) = sdl.get_text_size(&text);
    let new_game = TextButton {
        kind: ButtonKind::Menu,
        text,
        rect,
        text_pos: rect.center(width, height),
    };

    buttons.push(new_game);

    let y = y + offset_y;

    let rect = SDL_Rect { x, y, w, h };

    let text = String::from("I'M DONESKY");
    let text = CString::new(text).expect("CString::new failed");
    let (width, height) = sdl.get_text_size(&text);
    let new_game = TextButton {
        kind: ButtonKind::Quit,
        text,
        rect,
        text_pos: rect.center(width, height),
    };

    buttons.push(new_game);

    buttons
}

pub fn gen_menu_buttons() -> Vec<ImageButton> {
    let mut buttons: Vec<ImageButton> = Vec::with_capacity(2);

    let offset_y = 200;

    let (sx, sy, sw, sh) = PLAY_BTN_IMG;
    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);

    // let width = sw / 2;
    let x = (SCREEN_WIDTH - (sw / 2)) / 2;

    let y = 50;
    // let x = 100; // (w - (sw / 2)) / 2;
    let dstrect = SDL_Rect::new(x, y, sw / 2, sh / 2);

    let play = ImageButton {
        kind: ButtonKind::NewGame,
        srcrect,
        dstrect,
    };

    buttons.push(play);

    let (sx, sy, sw, sh) = RESUME_BTN_IMG;
    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);

    let y = y + offset_y;
    let dstrect = SDL_Rect::new(x, y, sw / 2, sh / 2);
    let resume = ImageButton {
        kind: ButtonKind::Resume,
        srcrect,
        dstrect,
    };

    buttons.push(resume);

    let (sx, sy, sw, sh) = QUIT_BTN_IMG;
    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);

    let y = y + 100;

    let dstrect = SDL_Rect::new(x, y, sw / 2, sh / 2);

    let quit = ImageButton {
        kind: ButtonKind::Quit,
        srcrect,
        dstrect,
    };

    buttons.push(quit);

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
        w: w / 2,
        h: h / 2,
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
        x: SCREEN_WIDTH - x_offset - (w / 2),
        y,
        w: w / 2,
        h: h / 2,
    };

    let btn = ImageButton {
        kind: ButtonKind::LevelUp,
        srcrect,
        dstrect,
    };

    image_buttons.push(btn);

    image_buttons
}
