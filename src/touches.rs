use crate::{pos::Pos, ButtonKind, ImageButton, Msg};

pub struct Touches {
    pub down: Option<Pos>,
    pub current: Option<Pos>,
    pub dragged: bool,
}

impl Touches {
    pub fn init() -> Touches {
        Touches {
            down: None,
            current: None,
            dragged: false,
        }
    }

    pub fn assign_down(&mut self, x: i32, y: i32) {
        self.down = Some(Pos(x, y));
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
