use os_info::Type;

use crate::{my_sdl::SDL_Rect, prelude::COLS};

pub fn is_mac() -> bool {
    let info = os_info::get();

    matches!(info.os_type(), Type::Macos)
}

pub fn is_linux() -> bool {
    let info = os_info::get();

    matches!(info.os_type(), Type::Fedora | Type::Ubuntu)
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * COLS) + x) as usize
}

pub fn tuple_to_rect((x, y, w, h): (i32, i32, i32, i32)) -> SDL_Rect {
    SDL_Rect { x, y, w, h }
}
