use crate::{
    game_state::{GameState, LevelChange},
    load_save_settings::Settings,
    touches::Touches,
    ImageButton, Msg,
};

pub fn check_level_change(
    current_ts: u64,
    touches: &Touches,
    state: &GameState,
    level_buttons: &[ImageButton],
    settings: &Settings,
) -> Option<(usize, GameState)> {
    let msg = touches.check_level_change(level_buttons);

    let dir: Option<i32> = match msg {
        Msg::LevelDown => Some(-1),
        Msg::LevelUp => Some(1),
        _ => return None,
    };

    if let Some(dir) = dir {
        if state.can_level_change(current_ts)
            && (dir == -1 && settings.level > 1 || dir == 1 && settings.level < 20)
        {
            let level = settings.level as i32;
            let level = level + dir;
            let level = level as usize;

            let level_change = match &state {
                GameState::Menu(lvl) => {
                    if let Some(lvl) = lvl {
                        Some(LevelChange {
                            initial: lvl.initial,
                            last: Some(current_ts),
                        })
                    } else {
                        Some(LevelChange {
                            initial: current_ts,
                            last: None,
                        })
                    }
                }
                _ => None,
            };

            level_change.map(|lvl| (level, GameState::Menu(Some(lvl))))
        } else {
            None
        }
    } else {
        None
    }
}
