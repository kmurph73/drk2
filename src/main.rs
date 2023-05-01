use my_sdl::MySdl;
use prelude::SCREEN_WIDTH;
use util::is_mac;

pub mod draw_grid;
pub mod handle_events;
pub mod my_sdl;
pub mod random_scenario;
pub mod util;

use crate::draw_grid::draw_grid;
use crate::handle_events::handle_events;
use crate::prelude::NUM_SQUARES;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 600;
    pub const SCREEN_HEIGHT: i32 = 1000;
    pub const COLS: i32 = 8;
    pub const ROWS: i32 = 14;
    pub const NUM_SQUARES: i32 = COLS * ROWS;
}

pub enum DotColor {
    Orange,
    Blue,
    Red,
    Green,
    Yellow,
}

pub enum DotType {
    Good,
    Bad,
}

pub struct Dot {
    pub color: DotColor,
    pub kind: DotType,
}

pub enum Msg {
    Nada,
    Quit,
}

fn main() {
    let is_mac = is_mac();
    let sdl = MySdl::init_sdl(is_mac);

    let square_size = SCREEN_WIDTH / 10;

    'running: loop {
        sdl.clear();

        let msg = handle_events();

        if let Msg::Quit = msg {
            break 'running;
        }

        draw_grid(&sdl, square_size);

        sdl.present();
    }

    sdl.quit();
}
