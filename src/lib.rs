use std::ffi::CString;

use crate::process_touches::process_touches;
use crate::random_scenario::random_scenario;
use calc_dot_drop_dist::{get_y_offsets, move_squares};
use check_level_change::check_level_change;
use cmd::Cmd;
use draw_game::{
    draw_dots_w_offsets, draw_piece, draw_piece_connectors, draw_piece_connectors_w_offsets,
};
use draw_menus::{draw_menu, draw_modal};
use game_state::GameState;
use gen_buttons::{
    gen_endgame_buttons, gen_help_buttons, gen_menu_buttons, gen_modal_text,
    gen_plus_minus_menu_buttons,
};
use handle_cmds::handle_cmds_mut;
use img_consts::{DEFEAT_IMG, PAUSED_IMG, VICTORY_IMG};
use keyboard::{Keyboard, KeyboardState};
use load_save_settings::load_settings;
use my_sdl::{MySdl, SDL_Rect};
use piece::Piece;
use pos::Pos;
use prelude::{LANDED_DELAY_MS, PIECE_DROP_MS_F64, PIECE_TRANSFER_MS_F64, TICK_RATE_MS, TOPSET};
use touches::Touches;
use util::{contains2, get_current_timestamp_millis, plot_line, tuple_to_rect};

pub mod blocks;
pub mod calc_dot_drop_dist;
pub mod calc_dots_to_drop;
pub mod check_level_change;
pub mod cmd;
pub mod colors;
pub mod dot;
pub mod draw_game;
pub mod draw_grid;
pub mod draw_menus;
pub mod easings;
pub mod game_state;
pub mod gen_buttons;
pub mod get_indexes_to_remove;
pub mod globals;
pub mod handle_cmds;
pub mod handle_events;
pub mod handle_keydown;
pub mod handle_keyup;
pub mod handle_mousedown;
pub mod handle_mouseup;
pub mod img_consts;
pub mod keyboard;
pub mod load_save_settings;
pub mod my_sdl;
pub mod my_sdl_rect;
pub mod mybindings;
pub mod piece;
pub mod pos;
pub mod process_touches;
pub mod random_scenario;
pub mod test_scenario;
pub mod touches;
pub mod util;

use crate::calc_dot_drop_dist::calc_dot_drop_dist;
use crate::draw_game::draw_dots;
use crate::draw_grid::draw_grid;
use crate::get_indexes_to_remove::get_indexes_to_remove;
use crate::handle_events::handle_events;

mod prelude {
    pub const TOPSET: i32 = 32;
    pub const COLS: i32 = 8;
    pub const ROWS: i32 = 16;
    pub const NUM_SQUARES: i32 = COLS * ROWS;

    pub const NUM_SQUARES_USIZE: usize = NUM_SQUARES as usize;
    pub const LANDED_DELAY_MS: u128 = 200;
    pub const TICK_RATE_MS: u128 = 800;
    pub const LEVEL_DEFAULT: usize = 10;
    pub const SPEED_DEFAULT: usize = 800;
    pub const SETTINGS_PATH: &str = "settings.json";

    pub const PIECE_DROP_MS: u128 = 40;
    pub const PIECE_DROP_MS_F64: f64 = PIECE_DROP_MS as f64;

    pub const PIECE_TRANSFER_MS: u128 = 200;
    pub const PIECE_TRANSFER_MS_F64: f64 = PIECE_TRANSFER_MS as f64;
    pub const DROP_MS: i32 = 100;
    pub const SNAP_MS: u128 = 120;
    pub const BTN_HOLD_DELAY_MS: u128 = 100;
}

pub enum ButtonKind {
    LevelUp,
    LevelDown,
    Resume,
    NewGame,
    Menu,
    Quit,
}

pub struct TextButton {
    pub kind: ButtonKind,
    pub text: CString,
    pub rect: SDL_Rect,
    pub text_pos: (i32, i32),
}

pub struct ImageButton {
    pub kind: ButtonKind,
    pub srcrect: SDL_Rect,
    pub dstrect: SDL_Rect,
}

pub struct Image {
    pub srcrect: SDL_Rect,
    pub dstrect: SDL_Rect,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Msg {
    LevelUp,
    LevelDown,
    PauseGame,
    ResumeGame,
    SuspendGame,
    NewGame,
    Nada,
    Quit,
    Menu,
    MouseUp,
    PieceLanded,
    DropPiece,
}

#[no_mangle]
pub extern "C" fn run_the_game() {
    let mut settings = load_settings();

    // let is_mac = is_mac();
    let (sdl, globals) = MySdl::init_sdl();

    let modal = globals.help_modal;
    let victory_image = gen_modal_text(&modal, tuple_to_rect(VICTORY_IMG));
    let defeat_image = gen_modal_text(&modal, tuple_to_rect(DEFEAT_IMG));
    let paused_image = gen_modal_text(&modal, tuple_to_rect(PAUSED_IMG));

    let help_buttons = gen_help_buttons(&sdl, &globals);
    let endgame_buttons = gen_endgame_buttons(&globals);
    let menu_buttons = gen_menu_buttons(&sdl, &globals);
    let y = menu_buttons[0].dstrect.y + 80;
    let level_buttons = gen_plus_minus_menu_buttons(y, &globals);

    let square_size = globals.square_size;
    let mut touches = Touches::init();

    let mut rng = rand::thread_rng();
    let mut squares = random_scenario(&mut rng, settings.level * 3);
    let on_deck = Piece::random_on_deck(&mut rng);
    let piece = Piece::random(&mut rng);

    let Pos(x0, y0) = on_deck.lhs.tile.top_left_px(square_size).add_y(TOPSET);
    // println!("{:#?}", on_deck.lhs.tile);
    let Pos(x1, y1) = piece.lhs.tile.top_left_px(square_size).add_y(TOPSET);
    let line = plot_line(x0, y0, x1, y1);
    let line_len_f64 = line.len() as f64;

    let mut on_deck_piece = Some(on_deck);
    let mut current_piece = Some(piece);

    // let mut state = GameState::Normal(get_current_timestamp_millis());
    let mut state = GameState::Menu(None);

    // let initial_time = get_current_timestamp();

    let mut pieces: Vec<Piece> = Vec::new();

    let mut keys = KeyboardState {
        pressed: Keyboard::init(false),
        enabled: Keyboard::init(true),
    };

    'running: loop {
        sdl.clear();

        let current_ts = get_current_timestamp_millis();

        let mut new_cmds: Vec<Cmd> = Vec::new();

        let msg = handle_events(
            &mut keys,
            &mut new_cmds,
            &mut touches,
            &state,
            &help_buttons,
            &menu_buttons,
            &endgame_buttons,
            current_ts,
            &sdl,
            &globals,
        );

        if state.is_normal() {
            process_touches(&mut touches, &mut new_cmds, current_ts, &globals);
        }

        match msg {
            Msg::Quit => {
                break 'running;
            }
            Msg::NewGame => {
                squares = random_scenario(&mut rng, settings.level * 3);
                touches.clear();
                pieces.clear();
                current_piece = Some(Piece::random(&mut rng));
                let od = Piece::random_on_deck(&mut rng);
                on_deck_piece = Some(od);

                state = GameState::Normal(current_ts);
            }
            Msg::Menu => {
                state = GameState::Menu(None);
            }
            Msg::PauseGame => {
                state = GameState::Paused;
            }
            Msg::ResumeGame => state = GameState::Normal(current_ts),
            Msg::LevelDown => {}
            Msg::LevelUp => {}
            Msg::Nada => {}
            Msg::PieceLanded => {}
            Msg::DropPiece => {}
            Msg::SuspendGame => {
                state = GameState::Suspended;
            }
            Msg::MouseUp => {
                if state.is_menu() {
                    state = GameState::Menu(None);
                }
            }
        }

        match state {
            GameState::PieceLanded => {
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
                touches.clear();

                let has_bad = squares.iter().flatten().any(|s| s.is_bad());
                if has_bad {
                    let dropping_dots =
                        calc_dot_drop_dist(&squares, &pieces, current_ts, square_size);
                    let has_falling_dots = dropping_dots.iter().any(|d| d.is_some());

                    if has_falling_dots {
                        let (y_offsets, _) = get_y_offsets(&dropping_dots, current_ts);
                        state = GameState::DroppingDots(dropping_dots, y_offsets);
                    } else {
                        state = GameState::DotsLanded(current_ts);
                    }
                } else {
                    touches.clear();
                    state = GameState::Victory;
                }
            }
            GameState::DroppingDots(dropping_dots, _) => {
                let (y_offsets, done) = get_y_offsets(&dropping_dots, current_ts);

                if done {
                    move_squares(&mut squares, &dropping_dots, &mut pieces);
                    state = GameState::DotsLanded(current_ts);
                } else {
                    state = GameState::DroppingDots(dropping_dots, y_offsets);
                }
            }
            GameState::DotsLanded(last_drop) => {
                let delta = get_current_timestamp_millis() - last_drop;
                if delta > LANDED_DELAY_MS {
                    let indexes_to_remove = get_indexes_to_remove(&squares);

                    if indexes_to_remove.is_empty() {
                        let has_bad = squares.iter().flatten().any(|s| s.is_bad());

                        if has_bad {
                            let has_piece_above_grid =
                                squares.iter().flatten().any(|d| d.above_grid());

                            if has_piece_above_grid {
                                touches.clear();
                                state = GameState::Defeat;
                            } else {
                                state = GameState::PreppingNextPiece(current_ts, Pos(0, 0));
                            }
                        } else {
                            touches.clear();
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
                            let dots =
                                calc_dot_drop_dist(&squares, &pieces, current_ts, square_size);
                            let has_falling_dots = dots.iter().any(|d| d.is_some());

                            if has_falling_dots {
                                let (y_offsets, _) = get_y_offsets(&dots, current_ts);
                                state = GameState::DroppingDots(dots, y_offsets);
                            } else {
                                state = GameState::DotsLanded(current_ts);
                            }
                        } else {
                            touches.clear();
                            state = GameState::Victory;
                        }
                    }
                }
            }
            GameState::Normal(last_tick) => {
                if let Some(piece) = &mut current_piece {
                    let msg = handle_cmds_mut(&new_cmds, piece, &squares);

                    match msg {
                        Msg::PieceLanded => {
                            state = GameState::PieceLanded;
                        }
                        Msg::DropPiece => {
                            if let Some(piece) = &current_piece {
                                let dist = piece.lowest_y_offset(&squares);
                                let dist_px = (dist * square_size) as f64;

                                state = GameState::DroppingPiece((dist, dist_px, current_ts, 0));
                            }
                        }
                        _ => {
                            let delta = current_ts - last_tick;

                            if delta > TICK_RATE_MS {
                                if piece.can_lower(&squares) {
                                    piece.lower_mut();

                                    state = GameState::Normal(current_ts);
                                } else {
                                    state = GameState::PieceLanded;
                                }
                            }
                        }
                    }
                }
            }
            GameState::Victory => {}
            GameState::Defeat => {}
            GameState::Paused => {}
            GameState::Suspended => {}
            GameState::DroppingPiece((dist, dist_px, ts, _)) => {
                if let Some(piece) = &mut current_piece {
                    let delta = current_ts - ts;
                    let delta = delta as f64;

                    let pct = delta / PIECE_DROP_MS_F64;

                    if pct < 1.0 {
                        let offset = dist_px * pct;
                        let offset = offset as i32;

                        state = GameState::DroppingPiece((dist, dist_px, ts, offset));
                    } else {
                        piece.add_mut(0, dist);
                        state = GameState::PieceLanded;
                    }
                }
            }
            GameState::Menu(_) => {
                if let Some((level, new_state)) =
                    check_level_change(current_ts, &touches, &state, &level_buttons, &settings)
                {
                    settings.level = level;
                    state = new_state;
                }
            }
            GameState::PreppingNextPiece(initial_tick, _last_offset) => {
                if let Some(on_deck) = &mut on_deck_piece {
                    let delta = current_ts - initial_tick;
                    let delta = delta as f64;

                    let pct: f64 = delta / PIECE_TRANSFER_MS_F64;

                    if pct < 1.0 {
                        let line_idx = line_len_f64 * pct;
                        let line_idx = line_idx as usize;
                        let (x, y) = line[line_idx];
                        let top_left = on_deck.lhs.tile.top_left_px(square_size);

                        let offset = top_left.abs_delta(x, y - TOPSET);

                        state = GameState::PreppingNextPiece(initial_tick, offset);
                    } else {
                        on_deck.originate_mut();
                        current_piece = Some(on_deck.clone());
                        on_deck_piece = Some(Piece::random_on_deck(&mut rng));

                        state = GameState::Normal(current_ts);
                    }
                }
            }
        }

        if state.is_menu() {
            draw_menu(
                &sdl,
                &menu_buttons,
                &level_buttons,
                settings.level,
                &globals,
            );
        } else {
            // draw_line(&sdl, &line);
            draw_grid(&sdl, square_size);

            match &state {
                GameState::DroppingDots(_, y_offsets) => {
                    draw_dots_w_offsets(&sdl, &squares, y_offsets, &globals);
                }
                _ => {
                    draw_dots(&sdl, &squares, (0, 0), &globals);
                }
            }

            if let Some(piece) = &on_deck_piece {
                match &state {
                    GameState::PreppingNextPiece(_ts, Pos(x, y)) => {
                        draw_piece(piece, &sdl, (*x, *y), &globals);
                    }
                    _ => {
                        draw_piece(piece, &sdl, (0, 0), &globals);
                    }
                }
            }

            if let Some(piece) = &current_piece {
                match &state {
                    GameState::DroppingPiece((_dist, _fdist, _ts, y_offset)) => {
                        draw_piece(piece, &sdl, (0, *y_offset), &globals);
                    }
                    _ => {
                        draw_piece(piece, &sdl, (0, 0), &globals);
                    }
                }
            }

            match &state {
                GameState::DroppingDots(_, y_offsets) => {
                    draw_piece_connectors_w_offsets(&pieces, &sdl, y_offsets, &globals);
                }
                _ => {
                    draw_piece_connectors(&pieces, &sdl, &globals);
                }
            }

            if state == GameState::Victory {
                draw_modal(&sdl, &endgame_buttons, &victory_image, &globals);
            } else if state == GameState::Defeat {
                draw_modal(&sdl, &endgame_buttons, &defeat_image, &globals);
            } else if state == GameState::Paused {
                draw_modal(&sdl, &help_buttons, &paused_image, &globals);
            }

            // let (x, y, w, h) = MENU_BTN;
            // let rect = SDL_Rect { x, y, w, h };
            // unsafe {
            //     SDL_RenderFillRect(sdl.renderer, &rect);
            // }
        }

        sdl.present();
    }

    sdl.quit();
}
