use crate::dot::Dot;

pub struct Blocks {
    pub grid: Vec<bool>,
}

impl Blocks {
    pub fn new(squares: &[Option<Dot>]) -> Blocks {
        let grid = squares.iter().map(|d| d.is_some()).collect();

        Blocks { grid }
    }

    pub fn blocks(&self, i: usize) -> bool {
        self.grid[i]
    }

    pub fn passable(&self, i: usize) -> bool {
        !self.grid[i]
    }

    pub fn set_both_block(&mut self, i: usize, i2: usize) {
        self.grid[i] = true;
        self.grid[i2] = true;
    }

    pub fn set_passable(&mut self, i: usize) {
        self.grid[i] = false;
    }

    pub fn set_blocks(&mut self, i: usize) {
        self.grid[i] = true;
    }

    pub fn set_both_passable(&mut self, i: usize, i2: usize) {
        self.grid[i] = false;
        self.grid[i2] = false;
    }
}
