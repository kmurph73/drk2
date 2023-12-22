use crate::{
    globals::Globals,
    img_consts::{
        ABOUT_BTN_IMG, LEVEL_IMG, MENU_BTN_IMG, MENU_LIGHT_BTN_IMG, MINUS_BTN_IMG,
        NEW_GAME_BTN_IMG, NEXT_LEVEL_BTN_IMG, PLAY_BTN_IMG, PLUS_BTN_IMG, RESUME_BTN_IMG,
    },
    load_save_settings::Settings,
    my_sdl::SDL_Rect,
    number_images::NumberImages,
    util::tuple_to_rect,
    ButtonKind, Image, ImageButton,
};

pub fn gen_level_text(globals: &Globals, settings: &Settings, x: i32) -> Vec<Image> {
    let mut arr = Vec::with_capacity(2);
    let offset = globals.square_size / 8;

    let dest_height = globals.square_size as f64 / 2.0;
    let (_x, _y, w, h) = LEVEL_IMG;
    let text_ratio = dest_height / h as f64;

    let dest_width = w as f64 * text_ratio;

    let srcrect = tuple_to_rect(LEVEL_IMG);
    let dstrect = SDL_Rect {
        x,
        y: offset,
        w: dest_width as i32,
        h: dest_height as i32,
    };

    let img = Image { srcrect, dstrect };
    arr.push(img);

    let srcrect = NumberImages::get_number_img(settings.level);
    let SDL_Rect { w, h, .. } = srcrect;

    let offset_y = dest_height / 5.0;

    let dest_height = dest_height + offset_y;
    let text_ratio = dest_height / h as f64;

    let dest_width = w as f64 * text_ratio;

    let (x, _y) = dstrect.top_right();
    let x = x + offset;

    let dstrect = SDL_Rect {
        x,
        y: offset - ((offset_y / 1.5) as i32),
        w: dest_width as i32,
        h: dest_height as i32,
    };

    let img = Image { srcrect, dstrect };
    arr.push(img);

    arr
}

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

pub fn gen_victory_buttons(
    Globals {
        square_size,
        window_width,
        help_modal,
        button_size,
        ..
    }: &Globals,
    settings: &Settings,
) -> Vec<ImageButton> {
    let mut buttons: Vec<ImageButton> = Vec::with_capacity(2);

    let y = help_modal.y + square_size * 3;

    let on_last_level = settings.level == 20;

    let (sx, sy, sw, sh) = if on_last_level {
        NEW_GAME_BTN_IMG
    } else {
        NEXT_LEVEL_BTN_IMG
    };

    let (dw, dh) = *button_size;

    let x = (window_width - dw) / 2;

    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);
    let dstrect = SDL_Rect::new(x, y, dw, dh);

    let kind = if settings.level == 20 {
        ButtonKind::NewGame
    } else {
        ButtonKind::NextLevel
    };

    let next_level = ImageButton {
        kind,
        srcrect,
        dstrect,
    };

    buttons.push(next_level);

    let y = y + square_size + dh;

    let (sx, sy, sw, sh) = MENU_BTN_IMG;
    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);

    let dstrect = SDL_Rect::new(x, y, dw, dh);

    let menu = ImageButton {
        kind: ButtonKind::Menu,
        srcrect,
        dstrect,
    };

    buttons.push(menu);

    buttons
}

pub fn gen_top_menu_btn(globals: &Globals) -> Image {
    let srcrect = tuple_to_rect(MENU_LIGHT_BTN_IMG);

    let w = globals.square_size * 4;
    let ratio = w as f64 / srcrect.w as f64;

    let h = ratio * (srcrect.h as f64);

    let x = (globals.window_width - globals.square_size * 4) - (globals.square_size / 2);
    let y = globals.square_size / 5;

    let dstrect = SDL_Rect::new(x, y, w, h as i32);

    Image { srcrect, dstrect }
}

pub fn gen_menu_buttons(globals: &Globals) -> Vec<ImageButton> {
    let mut buttons: Vec<ImageButton> = Vec::with_capacity(2);

    let (dw, dh) = globals.button_size;
    let (sx, sy, sw, sh) = PLAY_BTN_IMG;
    let srcrect = SDL_Rect::src_new(sx, sy, sw, sh);

    // let width = sw / 2;
    let dx = (globals.window_width - dw) / 2;

    let dy = globals.square_size * 2;
    // let x = 100; // (w - (sw / 2)) / 2;
    let dstrect = SDL_Rect::dst_new(dx, dy, dw, dh);

    let new_game = ImageButton {
        kind: ButtonKind::NewGame,
        srcrect,
        dstrect,
    };

    buttons.push(new_game);

    let srcrect = tuple_to_rect(ABOUT_BTN_IMG);
    let dy = dy + globals.square_size * 6;

    let dstrect = SDL_Rect::dst_new(dx, dy, dw, dh);

    let new_game = ImageButton {
        kind: ButtonKind::About,
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
