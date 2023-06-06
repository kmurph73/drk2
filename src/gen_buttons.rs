use std::ffi::CString;

use crate::{
    img_consts::{MINUS_SQUARE_IMG, PLUS_SQUARE_IMG},
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

pub fn gen_menu_buttons(sdl: &MySdl) -> Vec<TextButton> {
    let mut text_buttons: Vec<TextButton> = Vec::with_capacity(2);

    let (x, y, w, h) = HELP_MODAL;
    let modal_rect = SDL_Rect { x, y, w, h };

    let offset_y = 90;

    let y = modal_rect.y + offset_y;
    let x = modal_rect.x + SQUARE_SIZE;
    let w = modal_rect.w - SQUARE_SIZE * 2;
    let h = 60;

    let text = String::from("PLAY");
    let text = CString::new(text).expect("CString::new failed");
    let (width, height) = sdl.get_text_size(&text);
    let rect = SDL_Rect { x, y, w, h };

    let resume = TextButton {
        kind: ButtonKind::NewGame,
        text,
        rect,
        text_pos: rect.center(width, height),
    };

    text_buttons.push(resume);

    let srcrect = tuple_to_rect(MINUS_SQUARE_IMG);
    // let SDL_Rect { w, h, .. } = srcrect;

    let y = y + 200;

    // here

    let rect = SDL_Rect { x, y, w, h };

    let text = String::from("QUIT");
    let text = CString::new(text).expect("CString::new failed");
    let (width, height) = sdl.get_text_size(&text);
    let new_game = TextButton {
        kind: ButtonKind::Quit,
        text,
        rect,
        text_pos: rect.center(width, height),
    };

    text_buttons.push(new_game);

    text_buttons
}

pub fn gen_image_menu_buttons(sdl: &MySdl, y: i32) -> Vec<ImageButton> {
    let mut image_buttons: Vec<ImageButton> = Vec::with_capacity(2);

    let srcrect = tuple_to_rect(MINUS_SQUARE_IMG);
    let SDL_Rect { w, h, .. } = srcrect;
    let dstrect = SDL_Rect {
        x: SQUARE_SIZE,
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
        x: SCREEN_WIDTH - SQUARE_SIZE * 2,
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

    image_buttons
}
