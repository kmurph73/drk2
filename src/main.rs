use my_sdl::MySdl;
use util::is_mac;

pub mod handle_events;
pub mod my_sdl;
pub mod util;

use crate::handle_events::handle_events;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 800;
    pub const SCREEN_HEIGHT: i32 = 1000;
}

pub enum Msg {
    Nada,
    Quit,
}

fn main() {
    let is_mac = is_mac();
    let sdl = MySdl::init_sdl(is_mac);

    'running: loop {
        sdl.clear();

        let msg = handle_events();

        if let Msg::Quit = msg {
            break 'running;
        }

        sdl.present();
    }

    sdl.quit();
}
