use std::ffi::CString;

use crate::{
    globals::{self, Globals},
    my_sdl::SDL_Log,
    pos::Pos,
    prelude::SNAP_MS,
    ButtonKind, ImageButton, Msg,
};

pub struct Touches {
    pub down: Option<Pos>,
    pub current: Option<Pos>,
    pub dragged: bool,
    pub snap_x: Option<(i32, u128)>,
}

pub enum Snap {
    Right,
    Left,
    Clear,
}

impl Touches {
    pub fn init() -> Touches {
        Touches {
            down: None,
            current: None,
            dragged: false,
            snap_x: None,
        }
    }

    pub fn assign_down(&mut self, x: i32, y: i32, current_ts: u128) {
        self.down = Some(Pos(x, y));
        self.snap_x = Some((x, current_ts));
    }

    #[allow(clippy::comparison_chain)]
    pub fn check_snap(&self, current_x: i32, current_ts: u128, globals: &Globals) -> Option<Snap> {
        if let Some((snap_x, ts)) = self.snap_x {
            let delta = current_ts - ts;

            if delta > SNAP_MS {
                return Some(Snap::Clear);
            } else {
                let delta_x = current_x - snap_x;

                if delta_x < 0 && delta_x < -globals.snap_dist {
                    unsafe {
                        let str = CString::new("left").unwrap();
                        SDL_Log(str.as_ptr());
                    }
                    return Some(Snap::Left);
                } else if delta_x > 0 && delta_x > globals.snap_dist {
                    unsafe {
                        let str = CString::new("right").unwrap();
                        SDL_Log(str.as_ptr());
                    }
                    return Some(Snap::Right);
                }
            }
        }

        None
    }

    pub fn assign_motion(&mut self, x: i32, y: i32) {
        if self.down.is_none() {
            return;
        }

        self.current = Some(Pos(x, y));
    }

    pub fn clear(&mut self) {
        self.down = None;
        self.current = None;
        self.dragged = false;
    }

    pub fn moved_piece(&mut self) {
        self.down = self.current;
        self.current = None;
        self.dragged = true;
    }

    pub fn check_level_change(&self, btns: &[ImageButton]) -> Msg {
        let Pos(x, y) = if let Some(down) = self.down {
            down
        } else {
            return Msg::Nada;
        };

        for btn in btns {
            if btn.dstrect.contains(x, y) {
                match btn.kind {
                    ButtonKind::LevelDown => return Msg::LevelDown,
                    ButtonKind::LevelUp => return Msg::LevelUp,
                    _ => {}
                }
            }
        }

        Msg::Nada
    }
}
