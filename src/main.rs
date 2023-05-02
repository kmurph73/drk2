use my_sdl::MySdl;
use prelude::SCREEN_WIDTH;
use random_scenario::random_scenario;
use util::is_mac;

pub mod dot;
pub mod draw_app;
pub mod draw_grid;
pub mod handle_events;
pub mod img_consts;
pub mod my_sdl;
pub mod pos;
pub mod random_scenario;
pub mod util;

use crate::draw_app::draw_app;
use crate::draw_grid::draw_grid;
use crate::handle_events::handle_events;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 600;
    pub const SCREEN_HEIGHT: i32 = 1000;
    pub const COLS: i32 = 8;
    pub const ROWS: i32 = 14;
    pub const NUM_SQUARES: i32 = COLS * ROWS;
}

pub enum Msg {
    Nada,
    Quit,
}

fn main() {
    let is_mac = is_mac();
    let sdl = MySdl::init_sdl(is_mac);

    let square_size = SCREEN_WIDTH / 10;

    let squares = random_scenario();

    'running: loop {
        sdl.clear();

        let msg = handle_events();

        if let Msg::Quit = msg {
            break 'running;
        }

        draw_grid(&sdl, square_size);
        draw_app(&sdl, &squares, square_size);

        sdl.present();
    }

    sdl.quit();
}
