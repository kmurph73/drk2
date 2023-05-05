use crate::pos::Pos;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn offset(&self) -> Pos {
        let (x, y) = match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        Pos(x, y)
    }
}

pub enum Cmd {
    Move(Direction),
    Rotate,
}
