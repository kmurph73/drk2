use crate::{
    cmd::{Cmd, Direction},
    pos::Pos,
};

pub struct Touches {
    pub down: Option<Pos>,
    pub current: Option<Pos>,
}

impl Touches {
    pub fn process(&self) -> Vec<Cmd> {
        let diff = 50;
        let mut cmds = Vec::new();
        let Pos(x, y) = if let Some(down) = self.down {
            down
        } else {
            return cmds;
        };

        let Pos(current_x, current_y) = if let Some(current) = self.current {
            current
        } else {
            return cmds;
        };

        let delta_x = current_x - x;

        if delta_x > diff {
            let cmd = Cmd::Move(Direction::Right);
            cmds.push(cmd);
        } else if delta_x < -diff {
            let cmd = Cmd::Move(Direction::Left);
            cmds.push(cmd);
        }

        let delta_y = current_y - y;

        if delta_y > diff {
            let cmd = Cmd::Move(Direction::Down);
            cmds.push(cmd);
        }

        cmds
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
    }

    pub fn recenter(&mut self) {
        self.down = self.current;
        self.current = None;
    }
}
