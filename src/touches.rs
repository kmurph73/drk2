use crate::{pos::Pos, ButtonKind, ImageButton, MetaMsg};

pub struct Mouse {
    pub ts: u64,
    pub pos: Pos,
}

pub struct Touches {
    pub down: Option<Mouse>,
    pub dragged: bool,
    pub velocity: Option<(f32, f32)>,
    pub touches: Vec<Mouse>,
}

impl Touches {
    pub fn init() -> Touches {
        Touches {
            down: None,
            dragged: false,
            velocity: None,
            touches: Vec::new(),
        }
    }

    pub fn assign_down(&mut self, x: i32, y: i32, ts: u64) {
        let mouse = Mouse { ts, pos: Pos(x, y) };
        self.down = Some(mouse);
    }

    pub fn earliest_touch(&self, ts: u64) -> Option<(&Mouse, usize)> {
        self.touches.iter().enumerate().find_map(|m| {
            let (idx, mouse) = m;

            let delta_ms = ts - mouse.ts;

            if delta_ms < 100 {
                return Some((mouse, idx));
            }

            None
        })
    }

    pub fn assign_motion(&mut self, x: i32, y: i32, ts: u64) {
        if self.down.is_none() {
            return;
        }

        let current = Mouse { ts, pos: Pos(x, y) };

        if let Some((prev, idx)) = self.earliest_touch(ts) {
            let dx = (prev.pos.0 - current.pos.0) as f32;
            let dy = (prev.pos.1 - current.pos.1) as f32;

            let dt = (current.ts - prev.ts) as f32;

            let vx = dx / dt;
            let vy = dy / dt;

            self.velocity = Some((vx, vy));
            self.touches.drain(0..idx);
        } else {
            self.velocity = None;
        }

        self.touches.push(current);
    }

    pub fn latest(&self) -> Option<&Mouse> {
        self.touches.last()
    }

    pub fn clear(&mut self) {
        self.down = None;
        self.dragged = false;
        self.touches.clear();
    }

    pub fn moved_piece(&mut self, ts: u64) {
        if let Some(current) = &self.latest() {
            let down = Mouse {
                pos: current.pos,
                ts,
            };

            self.down = Some(down);
        } else {
            self.down = None;
        }
        self.touches.clear();
        self.dragged = true;
    }

    pub fn check_level_change(&self, btns: &[ImageButton]) -> Option<MetaMsg> {
        if let Some(mouse) = &self.down {
            let Pos(x, y) = mouse.pos;
            for btn in btns {
                if btn.dstrect.contains(x, y) {
                    match btn.kind {
                        ButtonKind::LevelDown => return Some(MetaMsg::LevelDown),
                        ButtonKind::LevelUp => return Some(MetaMsg::LevelUp),
                        _ => {}
                    }
                }
            }
        }

        None
    }
}
