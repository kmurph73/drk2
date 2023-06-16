use crate::{
    cmd::{Cmd, Direction},
    pos::Pos,
    prelude::{DRAG_DIFF, DROP_DRAG_DIFF},
};

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

    #[allow(clippy::manual_range_contains)]
    pub fn process(&mut self, cmds: &mut Vec<Cmd>) {
        let mut cmd: Option<Cmd> = None;
        let diff = DRAG_DIFF;
        let Pos(x, y) = if let Some(down) = self.down {
            down
        } else {
            return;
        };

        let Pos(current_x, current_y) = if let Some(current) = self.current {
            current
        } else {
            return;
        };

        let delta_x = current_x - x;

        if delta_x > diff {
            cmd = Some(Cmd::Move(Direction::Right));
        } else if delta_x < -diff {
            cmd = Some(Cmd::Move(Direction::Left));
        }

        let delta_y = current_y - y;

        if delta_y > diff && delta_y > delta_x.abs() {
            cmd = Some(Cmd::Move(Direction::Down));
        } else if delta_y < -DROP_DRAG_DIFF && cmd.is_none() {
            cmd = Some(Cmd::DropPiece);
        }

        if let Some(cmd) = cmd {
            cmds.push(cmd);
            self.dragged = true;
            self.moved_piece();
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
}
