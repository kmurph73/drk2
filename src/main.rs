use cmd::Cmd;
use handle_cmds::handle_cmds;
use keyboard::{Keyboard, KeyboardState};
use my_sdl::MySdl;
use piece::Piece;
use prelude::SCREEN_WIDTH;
use util::is_mac;

pub mod cmd;
pub mod dot;
pub mod draw_app;
pub mod draw_grid;
pub mod handle_cmds;
pub mod handle_events;
pub mod handle_keydown;
pub mod handle_keyup;
pub mod img_consts;
pub mod keyboard;
pub mod my_sdl;
pub mod piece;
pub mod pos;
pub mod process_events;
pub mod random_scenario;
pub mod test_scenario;
pub mod util;

use crate::draw_app::draw_app;
use crate::draw_grid::draw_grid;
use crate::handle_events::handle_events;
use crate::process_events::process_events;
use crate::test_scenario::test_scenario;

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

    // let mut rng = rand::thread_rng();
    let squares = test_scenario();
    let mut piece = Piece::custom();

    let img_divisor = if is_mac { 2 } else { 1 };

    let mut keys = KeyboardState {
        pressed: Keyboard::init(false),
        enabled: Keyboard::init(true),
    };

    'running: loop {
        sdl.clear();

        let mut new_cmds: Vec<Cmd> = Vec::new();

        let msg = handle_events(&mut keys, &mut new_cmds);

        if let Msg::Quit = msg {
            break 'running;
        }

        let events = handle_cmds(&new_cmds, &piece, &squares);

        process_events(&events, &mut piece);

        draw_grid(&sdl, square_size);
        draw_app(&sdl, &piece, &squares, square_size, img_divisor);

        sdl.present();
    }

    sdl.quit();
}
