use crate::{my_sdl::SDL_Rect, util::percent_of};

pub struct Globals {
    pub window_width: i32,
    pub window_height: i32,
    pub square_size: i32,
    pub topset: i32,
    pub drag_diff: i32,
    pub drag_drop_diff: i32,
    pub snap_dist: i32,
    pub dot_size: i32,
    pub connector_size: i32,
    pub button_size: (i32, i32),
    pub help_modal: SDL_Rect,
    pub menu_btn: SDL_Rect,
}

impl Globals {
    pub fn make(window_width: i32, window_height: i32, square_size: i32) -> Globals {
        let btn_dims = 82.0 / 325.0;
        let topset = percent_of(square_size, 0.25);
        let drag_diff = percent_of(square_size, 0.5);
        let drag_drop_diff = percent_of(square_size, 0.65);
        let snap_dist = square_size;
        let dot_size = percent_of(square_size, 0.9);
        let connector_size = percent_of(square_size, 0.28);
        let button_width = square_size * 6;
        let button_height = (button_width as f64) * btn_dims;
        let button_height = button_height as i32;

        let button_size = (button_width, button_height);

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
            window_height,
            window_width,
            square_size,
            topset,
            drag_diff,
            drag_drop_diff,
            snap_dist,
            dot_size,
            connector_size,
            button_size,
            help_modal,
            menu_btn,
        }
    }
}
