use crate::pos::Pos;

#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn is_horizontal(&self) -> bool {
        match self {
            Direction::Up => false,
            Direction::Down => false,
            Direction::Left => true,
            Direction::Right => true,
        }
    }
}

#[derive(Debug)]
pub enum Cmd {
    Move(Direction),
    Rotate,
    DropPiece,
}

pub enum Event {
    Move((Pos, Pos)),
    Rotate((Pos, Pos, i32)),
    DropPiece((Pos, Pos)),
}
