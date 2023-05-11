use cmd::Cmd;
use draw_game::{draw_piece, draw_piece_connectors};
use get_dots_to_drop::calc_dots_to_drop;
use handle_cmds::handle_cmds;
use keyboard::{Keyboard, KeyboardState};
use my_sdl::MySdl;
use piece::Piece;
use prelude::SCREEN_WIDTH;
use util::{contains2, is_mac};

pub mod cmd;
pub mod dot;
pub mod draw_game;
pub mod draw_grid;
pub mod get_dots_to_drop;
pub mod get_indexes_to_remove;
pub mod handle_cmds;
pub mod handle_events;
pub mod handle_keydown;
pub mod handle_keyup;
pub mod img_consts;
pub mod keyboard;
pub mod my_sdl;
pub mod piece;
pub mod pos;
pub mod random_scenario;
pub mod test_scenario;
pub mod util;

use crate::draw_game::draw_dots;
use crate::draw_grid::draw_grid;
use crate::get_indexes_to_remove::get_indexes_to_remove;
use crate::handle_events::handle_events;
use crate::test_scenario::test_scenario;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 600;
    pub const SCREEN_HEIGHT: i32 = 1000;
    pub const COLS: i32 = 8;
    pub const ROWS: i32 = 14;
    pub const NUM_SQUARES: i32 = COLS * ROWS;
    pub const NUM_SQUARES_USIZE: usize = NUM_SQUARES as usize;
}

pub enum Msg {
    Nada,
    Quit,
}

pub enum GameState {
    Normal,
    PieceLanded,
}

fn main() {
    let is_mac = is_mac();
    let sdl = MySdl::init_sdl(is_mac);

    let square_size = SCREEN_WIDTH / 10;

    // let mut rng = rand::thread_rng();
    let mut squares = test_scenario();
    let mut current_piece = Some(Piece::custom());

    let mut pieces: Vec<Piece> = Vec::new();

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

        if let Some(piece) = &mut current_piece {
            let just_landed = handle_cmds(&new_cmds, piece, &squares);

            if just_landed {
                let lhs_idx = piece.lhs.idx();
                let rhs_idx = piece.rhs.idx();
                squares[lhs_idx] = Some(piece.lhs.clone());
                squares[rhs_idx] = Some(piece.rhs.clone());
                let indexes_to_remove = get_indexes_to_remove(&squares);

                for idx in &indexes_to_remove {
                    squares[*idx] = None;
                }

                let needs_piece = !contains2(&indexes_to_remove, &lhs_idx, &rhs_idx);

                if needs_piece {
                    pieces.push(current_piece.unwrap().clone());
                }

                current_piece = None;
            }
        }

        draw_grid(&sdl, square_size);
        draw_dots(&sdl, &squares, square_size, img_divisor);
        if let Some(piece) = &current_piece {
            draw_piece(piece, &sdl, square_size, img_divisor);
        }

        draw_piece_connectors(&pieces, &sdl, square_size, img_divisor);

        let to_drop = calc_dots_to_drop(&squares, &pieces);
        for idx in to_drop {
            if let Some(dot) = &mut squares[idx] {
                dot.tile.1 += 1;
            }
        }

        sdl.present();
    }

    sdl.quit();
}
