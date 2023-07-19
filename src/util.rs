use std::time::{SystemTime, UNIX_EPOCH};

use crate::{my_sdl::SDL_Rect, prelude::COLS};

pub fn percent_of(n: i32, pct: f64) -> i32 {
    let n = (n as f64) * pct;

    n as i32
}

pub fn plot_line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
    let mut arr: Vec<(i32, i32)> = Vec::new();
    let dx = x1 - x0;
    let dy = y1 - y0;
    let mut d = 2 * dy - dx;
    let mut y = y0;

    for x in x0..x1 {
        arr.push((x, y));
        if d > 0 {
            y += 1;
            d -= 2 * dx;
        }

        d += 2 * dy;
    }

    arr
}

pub fn is_mac() -> bool {
    true
    // let info = os_info::get();

    // matches!(info.os_type(), Type::Macos)
}

// pub fn is_linux() -> bool {
//     let info = os_info::get();

//     matches!(info.os_type(), Type::Fedora | Type::Ubuntu)
// }

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * COLS) + x) as usize
}

pub fn tuple_to_rect((x, y, w, h): (i32, i32, i32, i32)) -> SDL_Rect {
    SDL_Rect { x, y, w, h }
}

pub fn contains2<T: std::cmp::PartialEq>(arr: &Vec<T>, a: &T, b: &T) -> bool {
    for i in arr {
        if a == i || b == i {
            return true;
        }
    }

    false
}

// https://www.reddit.com/r/rust/comments/qjh00f/fill_a_vecoptiont_with_none_without_requiring/
pub fn empty_array<T>(capacity: usize) -> Vec<Option<T>> {
    let mut vec = Vec::with_capacity(capacity);

    for _ in 0..capacity {
        vec.push(None);
    }

    vec
}

pub fn get_current_timestamp_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

pub fn get_current_timestamp_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}
