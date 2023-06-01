use crate::random_scenario::random_scenario;
use cmd::Cmd;
use draw_game::{draw_piece, draw_piece_connectors};
use get_dots_to_drop::calc_dots_to_drop;
use handle_cmds::handle_cmds;
use keyboard::{Keyboard, KeyboardState};
use my_sdl::MySdl;
use piece::Piece;
use prelude::{DROP_RATE_MS, LANDED_DELAY_MS, SCREEN_WIDTH};
use util::{contains2, get_current_timestamp_millis, is_mac};

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

mod prelude {
    pub const SCREEN_WIDTH: i32 = 600;
    pub const SCREEN_HEIGHT: i32 = 1000;
    pub const COLS: i32 = 8;
    pub const ROWS: i32 = 16;
    pub const NUM_SQUARES: i32 = COLS * ROWS;
    pub const SQUARE_SIZE: i32 = SCREEN_WIDTH / (COLS + 2);
    pub const NUM_SQUARES_USIZE: usize = NUM_SQUARES as usize;
    pub const DROP_RATE_MS: u128 = 34;
    pub const LANDED_DELAY_MS: u128 = 200;
}

#[derive(Debug, Eq, PartialEq)]
pub enum Msg {
    NewGame,
    Nada,
    Quit,
}

#[derive(Debug, Eq, PartialEq)]
pub enum GameState {
    Normal,
    PieceLanded(u128),
    DroppingDots(u128),
    DotsLanded(u128),
    Victory,
}

fn main() {
    let is_mac = is_mac();
    let sdl = MySdl::init_sdl(is_mac);

    let square_size = SCREEN_WIDTH / 10;

    let num_bad_guys = 1;
    let mut rng = rand::thread_rng();
    let mut squares = random_scenario(&mut rng, num_bad_guys);
    let mut current_piece = Some(Piece::random(&mut rng));

    let mut state = GameState::Normal;

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

        let msg = handle_events(&mut keys, &mut new_cmds, &state);

        match msg {
            Msg::Quit => {
                break 'running;
            }
            Msg::NewGame => {
                squares = random_scenario(&mut rng, num_bad_guys);
                pieces.clear();
                current_piece = Some(Piece::random(&mut rng));
                state = GameState::Normal;
                continue;
            }
            Msg::Nada => {}
        }

        if state == GameState::Normal {
            if let Some(piece) = &mut current_piece {
                let land_piece = handle_cmds(&new_cmds, piece, &squares);

                if land_piece {
                    state = GameState::PieceLanded(get_current_timestamp_millis());
                }
            }
        }

        match state {
            GameState::PieceLanded(land_time) => {
                let piece = current_piece.expect("piece should be here");

                let lhs_idx = piece.lhs.idx();
                let rhs_idx = piece.rhs.idx();
                squares[lhs_idx] = Some(piece.lhs.clone());
                squares[rhs_idx] = Some(piece.rhs.clone());
                let indexes_to_remove = get_indexes_to_remove(&squares);

                for idx in &indexes_to_remove {
                    squares[*idx] = None;
                }

                pieces.retain(|p| {
                    let mut retain = true;

                    for idx in &indexes_to_remove {
                        if p.has_idx(*idx) {
                            retain = false;
                            break;
                        }
                    }

                    retain
                });

                let needs_piece = !contains2(&indexes_to_remove, &lhs_idx, &rhs_idx);

                if needs_piece {
                    pieces.push(piece.clone());
                }

                current_piece = None;

                state = GameState::DroppingDots(land_time);
            }
            GameState::DroppingDots(last_drop) => {
                let ts = get_current_timestamp_millis();
                let delta = ts - last_drop;

                if delta > DROP_RATE_MS {
                    let to_drop = calc_dots_to_drop(&squares, &pieces);
                    if to_drop.is_empty() {
                        state = GameState::DotsLanded(ts);
                    } else {
                        for idx in &to_drop {
                            let result = if let Some(dot) = &squares[*idx] {
                                let dot = dot.lower();
                                let next_idx = dot.idx();

                                Some((next_idx, dot.clone()))
                            } else {
                                None
                            };

                            if let Some((next_idx, dot)) = result {
                                squares[*idx] = None;
                                squares[next_idx] = Some(dot);
                            }
                        }

                        for piece in &mut pieces {
                            for idx in &to_drop {
                                if piece.has_idx(*idx) {
                                    piece.lower_mut();
                                }
                            }
                        }

                        state = GameState::DroppingDots(ts);
                    }
                }
            }
            GameState::DotsLanded(last_drop) => {
                let delta = get_current_timestamp_millis() - last_drop;
                if delta > LANDED_DELAY_MS {
                    let indexes_to_remove = get_indexes_to_remove(&squares);

                    if indexes_to_remove.is_empty() {
                        let has_bad = squares.iter().flatten().any(|s| s.is_bad());
                        if has_bad {
                            current_piece = Some(Piece::random(&mut rng));
                            state = GameState::Normal;
                        } else {
                            state = GameState::Victory;
                        }
                    } else {
                        for idx in &indexes_to_remove {
                            squares[*idx] = None;
                        }

                        pieces.retain(|p| {
                            let mut retain = true;

                            for idx in &indexes_to_remove {
                                if p.has_idx(*idx) {
                                    retain = false;
                                    break;
                                }
                            }

                            retain
                        });

                        let has_bad = squares.iter().flatten().any(|s| s.is_bad());

                        if has_bad {
                            state = GameState::DroppingDots(last_drop)
                        } else {
                            state = GameState::Victory;
                        }
                    }
                }
            }
            GameState::Normal => {}
            GameState::Victory => {}
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

        if state == GameState::Victory {
            sdl.draw_victory_text();
        }

        sdl.present();
    }

    sdl.quit();
}
