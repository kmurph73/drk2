use cmd::Cmd;
use draw_game::{draw_piece, draw_piece_connectors};
use get_dots_to_drop::calc_dots_to_drop;
use handle_cmds::handle_cmds;
use keyboard::{Keyboard, KeyboardState};
use my_sdl::MySdl;
use piece::Piece;
use prelude::{DROP_RATE_MS, SCREEN_WIDTH};
use util::{contains2, get_current_timestamp_millis, get_current_timestamp_seconds, is_mac};

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
    pub const DROP_RATE_MS: u128 = 400;
}

pub enum Msg {
    Nada,
    Quit,
}

#[derive(PartialEq)]
pub enum GameState {
    Normal,
    PieceLanded(u128),
}

fn main() {
    let is_mac = is_mac();
    let sdl = MySdl::init_sdl(is_mac);

    let square_size = SCREEN_WIDTH / 10;

    // let mut rng = rand::thread_rng();
    let mut squares = test_scenario();
    let mut current_piece = Some(Piece::custom());
    let mut piece_just_landed = false;

    let mut state = GameState::Normal;

    let mut frame = 0;

    let mut dots_to_drop: Vec<usize> = Vec::new();

    // let initial_time = get_current_timestamp();

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

        if state == GameState::Normal {
            if let Some(piece) = &mut current_piece {
                let just_landed = handle_cmds(&new_cmds, piece, &squares);

                if just_landed {
                    state = GameState::PieceLanded(get_current_timestamp_millis());

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
        }

        // let current_time = get_current_timestamp();
        // let delta = current_time - initial_time;

        // if delta > 0 {
        //     let fps = frame / delta;
        //     println!("secs: {delta}");

        //     sdl.draw_fps(fps);
        // }

        draw_grid(&sdl, square_size);
        draw_dots(&sdl, &squares, square_size, img_divisor);
        if let Some(piece) = &current_piece {
            draw_piece(piece, &sdl, square_size, img_divisor);
        }

        draw_piece_connectors(&pieces, &sdl, square_size, img_divisor);

        match state {
            GameState::PieceLanded(last_drop) => {
                let delta = get_current_timestamp_millis() - last_drop;
                if delta > DROP_RATE_MS {
                    let to_drop = calc_dots_to_drop(&squares, &pieces);
                    if to_drop.is_empty() {
                        state = GameState::Normal;
                    } else {
                        for idx in to_drop {
                            if let Some(dot) = &mut squares[idx] {
                                dot.tile.1 += 1;
                            }
                        }

                        let indexes_to_remove = get_indexes_to_remove(&squares);

                        for idx in &indexes_to_remove {
                            squares[*idx] = None;
                        }

                        state = GameState::PieceLanded(get_current_timestamp_millis());
                    }
                }
            }
            GameState::Normal => {}
        }

        if piece_just_landed {}

        sdl.present();

        frame += 1;
    }

    sdl.quit();
}
