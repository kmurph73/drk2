use crate::{
    img_consts::{BASE_DOT_SIZE, CONNECTOR_IMG, NEW_GAME_BTN_IMG},
    my_sdl::SDL_Rect,
    prelude::{COLS, ROWS},
    util::percent_of,
};

#[derive(Debug)]
pub struct Globals {
    pub window_width: i32,
    pub window_height: i32,
    pub square_size: i32,
    pub dotset: i32,
    pub topset: i32,
    pub left_x: i32,
    pub drag_diff: i32,
    pub drag_drop_diff: i32,
    pub snap_dist: i32,
    pub dot_size: i32,
    pub connector_size: i32,
    pub button_size: (i32, i32),
    pub help_modal: SDL_Rect,
    pub menu_btn: SDL_Rect,
    pub ratio: f64,
}

pub fn get_int(ratio: f64, (_x, _y, w, _h): (i32, i32, i32, i32)) -> i32 {
    (w as f64 * ratio) as i32
}

pub fn get_size(ratio: f64, (_x, _y, w, h): (i32, i32, i32, i32)) -> (i32, i32) {
    (((w as f64 * ratio) as i32), ((h as f64 * ratio) as i32))
}

impl Globals {
    pub fn make(window_width: i32, window_height: i32) -> Globals {
        let square_size = window_width / (COLS + 2);

        let square_size = if square_size * (ROWS + 1) > window_height {
            window_height / (ROWS + 1)
        } else {
            square_size
        };

        let left_x = (window_width - (square_size * COLS)) / 2;

        let topset = percent_of(square_size, 0.25);
        let drag_diff = percent_of(square_size, 0.69);
        let drag_drop_diff = percent_of(square_size, 0.70);
        let snap_dist = percent_of(square_size, 1.75);
        let dot_size = percent_of(square_size, 0.90);
        let ratio = (dot_size as f64) / (BASE_DOT_SIZE as f64);

        let dotset = (square_size - dot_size) / 2;

        let connector_size = get_int(ratio, CONNECTOR_IMG);
        let button_size = get_size(ratio, NEW_GAME_BTN_IMG);

        let (x, y, w, h) = (
            square_size,
            square_size * 2,
            window_width - square_size * 2,
            window_height - square_size * 4,
        );

        let help_modal = SDL_Rect::new(x, y, w, h);

        let (x, y, w, h) = (
            window_width / 2,
            0,
            window_width / 2,
            square_size * 2 + topset,
        );

        let menu_btn = SDL_Rect::new(x, y, w, h);

        Globals {
            left_x,
            window_height,
            window_width,
            square_size,
            dotset,
            topset,
            drag_diff,
            drag_drop_diff,
            snap_dist,
            dot_size,
            connector_size,
            button_size,
            help_modal,
            menu_btn,
            ratio,
        }
    }
}
