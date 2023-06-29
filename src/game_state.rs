use crate::{calc_dot_drop_dist::DroppingDot, prelude::BTN_HOLD_DELAY_MS};

#[derive(Debug, Eq, PartialEq)]
pub struct LevelChange {
    pub initial: u128,
    pub last: Option<u128>,
}

#[derive(Debug, PartialEq)]
pub enum GameState {
    PieceLanded,
    DroppingDots(Vec<Option<DroppingDot>>, Vec<i32>),
    DotsLanded(u128),
    Normal(u128),
    PreppingNextPiece(u128),
    Suspended,
    Victory,
    Defeat,
    Paused,
    Menu(Option<LevelChange>),
}

impl GameState {
    pub fn is_endgame(&self) -> bool {
        *self == GameState::Victory || *self == GameState::Defeat
    }

    pub fn is_paused(&self) -> bool {
        *self == GameState::Paused
    }

    pub fn is_suspended(&self) -> bool {
        *self == GameState::Suspended
    }

    pub fn is_normal(&self) -> bool {
        matches!(*self, GameState::Normal(_))
    }

    pub fn is_menu(&self) -> bool {
        matches!(*self, GameState::Menu(_))
    }

    pub fn can_level_change(&self, current_ts: u128) -> bool {
        match self {
            GameState::Menu(lvl) => {
                if let Some(lvl) = lvl {
                    if let Some(last) = lvl.last {
                        let delta = current_ts - last;

                        delta > BTN_HOLD_DELAY_MS
                    } else {
                        let delta = current_ts - lvl.initial;

                        delta > 500
                    }
                } else {
                    true
                }
            }
            _ => false,
        }
    }

    pub fn level_changed(&mut self, current_ts: u128) {
        if let GameState::Menu(Some(lvl)) = self {
            lvl.last = Some(current_ts);
        }
    }
}
