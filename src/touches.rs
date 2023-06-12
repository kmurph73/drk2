use crate::{
    cmd::{Cmd, Direction},
    pos::Pos,
};

pub struct Touches {
    pub down: Option<Pos>,
    pub current: Option<Pos>,
    pub dragged: bool,
}

const DRAG_DIFF: i32 = 10;

impl Touches {
    pub fn default() -> Touches {
        Touches {
            down: None,
            current: None,
            dragged: false,
        }
    }
    pub fn process(&mut self, cmds: &mut Vec<Cmd>) {
        let diff = 50;
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

        let mut moved_piece = false;

        let delta_x = current_x - x;

        if delta_x > diff {
            let cmd = Cmd::Move(Direction::Right);
            cmds.push(cmd);
            moved_piece = true;
        } else if delta_x < -diff {
            let cmd = Cmd::Move(Direction::Left);
            cmds.push(cmd);
            moved_piece = true;
        }

        let delta_y = current_y - y;

        if delta_y > diff {
            let cmd = Cmd::Move(Direction::Down);
            cmds.push(cmd);
            moved_piece = true;
        }

        if delta_y < -diff {
            cmds.push(Cmd::DropPiece);
        }

        if delta_x > DRAG_DIFF
            || delta_x < -DRAG_DIFF
            || delta_y > DRAG_DIFF
            || delta_y < -DRAG_DIFF
        {
            self.dragged = true;
        }

        if moved_piece {
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
