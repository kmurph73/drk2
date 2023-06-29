use std::ffi::CString;

use crate::process_touches::process_touches;
use crate::random_scenario::random_scenario;
use crate::test_scenario::test_scenario;
use calc_dot_drop_dist::{get_y_offsets, move_squares};
use check_level_change::check_level_change;
use cmd::Cmd;
use draw_game::{draw_dots_w_offsets, draw_piece, draw_piece_connectors};
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
use prelude::{HELP_MODAL, LANDED_DELAY_MS, SCREEN_WIDTH, TICK_RATE_MS};
use touches::Touches;
use util::{contains2, get_current_timestamp_millis, tuple_to_rect};

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
    pub const SCREEN_WIDTH: i32 = 375;
    pub const SCREEN_HEIGHT: i32 = 812;
    pub const TOPSET: i32 = 32;
    pub const COLS: i32 = 8;
    pub const ROWS: i32 = 16;
    pub const NUM_SQUARES: i32 = COLS * ROWS;
    pub const SQUARE_SIZE: i32 = SCREEN_WIDTH / (COLS + 2);
    pub const HELP_MODAL: (i32, i32, i32, i32) = (
        SQUARE_SIZE,
        SQUARE_SIZE * 2,
        SCREEN_WIDTH - SQUARE_SIZE * 2,
        SCREEN_HEIGHT - SQUARE_SIZE * 4,
    );
    pub const IMG_DIVISOR: i32 = 2;
    pub const NUM_SQUARES_USIZE: usize = NUM_SQUARES as usize;
    pub const LANDED_DELAY_MS: u128 = 200;
    pub const TICK_RATE_MS: u128 = 800;
    pub const LEVEL_DEFAULT: usize = 10;
    pub const SPEED_DEFAULT: usize = 800;
    pub const SETTINGS_PATH: &str = "settings.json";
    pub const MENU_BTN: (i32, i32, i32, i32) = (
        SCREEN_WIDTH / 2,
        0,
        SCREEN_WIDTH / 2,
        SQUARE_SIZE * 2 + TOPSET,
    );

    pub const DROP_MS: i32 = 150;
    pub const DRAG_DIFF: i32 = 23;
    pub const SNAP_MS: u128 = 115;
    pub const SNAP_DIST: i32 = 130;
    pub const DROP_DRAG_DIFF: i32 = 40;
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
}

#[no_mangle]
pub extern "C" fn run_the_game() {
    let mut settings = load_settings();

    let modal = tuple_to_rect(HELP_MODAL);
    let victory_image = gen_modal_text(&modal, tuple_to_rect(VICTORY_IMG));
    let defeat_image = gen_modal_text(&modal, tuple_to_rect(DEFEAT_IMG));
    let paused_image = gen_modal_text(&modal, tuple_to_rect(PAUSED_IMG));

    // let is_mac = is_mac();
    let sdl = MySdl::init_sdl();
    let help_buttons = gen_help_buttons();
    let endgame_buttons = gen_endgame_buttons();
    let menu_buttons = gen_menu_buttons();
    let y = menu_buttons[0].dstrect.y + 80;
    let level_buttons = gen_plus_minus_menu_buttons(y);

    let square_size = SCREEN_WIDTH / 10;
    let mut touches = Touches::init();

    let mut rng = rand::thread_rng();
    let mut squares = test_scenario();
    let mut on_deck_piece = Some(Piece::custom_on_deck());
    let mut current_piece = Some(Piece::custom());

    // let mut state = GameState::Normal(get_current_timestamp_millis());
    let mut state = GameState::Menu(None);

    // let initial_time = get_current_timestamp();

    let mut pieces: Vec<Piece> = Vec::new();

    let img_divisor = 2; // if is_mac { 2 } else { 1 };

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
        );

        if state.is_normal() {
            process_touches(&mut touches, &mut new_cmds, current_ts);
        }

        match msg {
            Msg::Quit => {
                break 'running;
            }
            Msg::NewGame => {
                squares = test_scenario();
                touches.clear();
                pieces.clear();
                current_piece = Some(Piece::custom());
                on_deck_piece = Some(Piece::custom_on_deck());
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
                    let dropping_dots = calc_dot_drop_dist(&squares, &pieces, current_ts);
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
                    move_squares(&mut squares, &dropping_dots);
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
                                state = GameState::PreppingNextPiece(current_ts);
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
                            let dots = calc_dot_drop_dist(&squares, &pieces, current_ts);
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

                    if msg == Msg::PieceLanded {
                        state = GameState::PieceLanded;
                    } else {
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
            GameState::Victory => {}
            GameState::Defeat => {}
            GameState::Paused => {}
            GameState::Suspended => {}
            GameState::Menu(_) => {
                if let Some((level, new_state)) =
                    check_level_change(current_ts, &touches, &state, &level_buttons, &settings)
                {
                    settings.level = level;
                    state = new_state;
                }
            }
            GameState::PreppingNextPiece(_last_tick) => {
                if let Some(on_deck) = &mut on_deck_piece {
                    on_deck.originate_mut();
                    current_piece = Some(on_deck.clone());
                    on_deck_piece = Some(Piece::custom_on_deck());

                    state = GameState::Normal(current_ts);
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

        if state.is_menu() {
            draw_menu(&sdl, &menu_buttons, &level_buttons, settings.level);
        } else {
            draw_grid(&sdl, square_size);

            match &state {
                GameState::DroppingDots(_, y_offsets) => {
                    draw_dots_w_offsets(&sdl, &squares, square_size, img_divisor, y_offsets);
                }
                _ => {
                    draw_dots(&sdl, &squares, square_size, img_divisor);
                }
            }

            if let Some(piece) = &on_deck_piece {
                draw_piece(piece, &sdl, square_size, img_divisor);
            }

            if let Some(piece) = &current_piece {
                draw_piece(piece, &sdl, square_size, img_divisor);
            }

            draw_piece_connectors(&pieces, &sdl, square_size, img_divisor);

            if state == GameState::Victory {
                draw_modal(&sdl, &endgame_buttons, &victory_image);
            } else if state == GameState::Defeat {
                draw_modal(&sdl, &endgame_buttons, &defeat_image);
            } else if state == GameState::Paused {
                draw_modal(&sdl, &help_buttons, &paused_image);
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
