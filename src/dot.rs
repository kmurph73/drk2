use rand::rngs::ThreadRng;
use rand::Rng;

use crate::{
    blocks::Blocks,
    colors::{BLUE, GREEN, ORANGE, RED, YELLOW},
    img_consts::{
        BLUE_DOT_IMG, BLUE_KODAMA_IMG, GREEN_DOT_IMG, GREEN_KODAMA_IMG, ORANGE_DOT_IMG,
        ORANGE_KODAMA_IMG, RED_DOT_IMG, RED_KODAMA_IMG, YELLOW_DOT_IMG, YELLOW_KODAMA_IMG,
    },
    my_sdl::{SDL_Color, SDL_Rect},
    pos::Pos,
    prelude::{COLS, NUM_SQUARES_USIZE, ROWS},
    util::{map_idx, tuple_to_rect},
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DotColor {
    Orange,
    Blue,
    Red,
    Green,
    Yellow,
}

impl DotColor {
    pub fn color(&self) -> SDL_Color {
        match self {
            DotColor::Orange => ORANGE,
            DotColor::Blue => BLUE,
            DotColor::Red => RED,
            DotColor::Green => GREEN,
            DotColor::Yellow => YELLOW,
        }
    }

    pub fn random(rng: &mut ThreadRng) -> DotColor {
        let n = rng.gen_range(0..5);

        DotColor::from_number(n)
    }

    pub fn from_number(n: i32) -> DotColor {
        match n {
            0 => DotColor::Red,
            1 => DotColor::Green,
            2 => DotColor::Blue,
            3 => DotColor::Yellow,
            4 => DotColor::Orange,
            _ => panic!("number {n} doesnt correspond to a DotColor"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DotType {
    Good,
    Bad,
}

#[derive(Debug, Clone)]
pub struct Dot {
    pub tile: Pos,
    pub color: DotColor,
    pub kind: DotType,
}

impl Dot {
    pub fn color(&self) -> SDL_Color {
        self.color.color()
    }

    pub fn add_mut(&mut self, x: i32, y: i32) {
        self.tile.0 += x;
        self.tile.1 += y;
    }

    pub fn lower_than(&self, dot: &Dot) -> bool {
        self.tile.1 > dot.tile.1
    }

    pub fn lower(&self) -> Dot {
        Dot {
            tile: self.tile.add_y(1),
            color: self.color.clone(),
            kind: self.kind.clone(),
        }
    }

    pub fn can_drop2(&self, squares: &[Option<Dot>], ignore: usize) -> bool {
        let idx = self.tile.add_y(1).idx();

        idx < NUM_SQUARES_USIZE && (idx == ignore || squares[idx].is_none())
    }

    pub fn can_lower3(&self, blocks: &Blocks, ignore: usize) -> bool {
        let tile = self.tile.add_y(1);
        let idx = tile.idx();
        if ignore == idx {
            return true;
        }

        tile.1 < ROWS && blocks.passable(idx)
    }

    pub fn can_drop4(&self, blocks: &Blocks) -> bool {
        let idx = self.tile.add_y(1).idx();

        idx < NUM_SQUARES_USIZE && blocks.passable(idx)
    }

    pub fn above_grid(&self) -> bool {
        self.tile.1 < 2
    }

    pub fn is_good(&self) -> bool {
        self.kind == DotType::Good
    }

    pub fn is_bad(&self) -> bool {
        self.kind == DotType::Bad
    }

    pub fn idx(&self) -> usize {
        let Pos(x, y) = self.tile;

        map_idx(x, y)
    }

    pub fn lowest_y(&self, squares: &[Option<Dot>]) -> i32 {
        let x = self.tile.0;
        let mut y = self.tile.1 + 1;

        while y < ROWS {
            let idx = map_idx(x, y);
            if squares[idx].is_some() {
                return y - 1;
            }

            y += 1;
        }

        y - 1
    }

    pub fn greatest_x_offset(&self, squares: &[Option<Dot>]) -> i32 {
        let mut offset = 0;
        let mut x = self.tile.0 + 1;
        let y = self.tile.1;

        while x < COLS {
            let idx = map_idx(x, y);

            if squares[idx].is_some() {
                return offset;
            }

            offset += 1;
            x += 1;
        }

        offset
    }

    pub fn lowest_x_offset(&self, squares: &[Option<Dot>]) -> i32 {
        let mut offset = 0;
        let mut x = self.tile.0 - 1;
        let y = self.tile.1;

        while x >= 0 {
            let idx = map_idx(x, y);

            if squares[idx].is_some() {
                return offset;
            }

            offset -= 1;
            x -= 1;
        }

        offset
    }

    pub fn lowest_y_offset(&self, squares: &[Option<Dot>]) -> i32 {
        let mut offset = 0;
        let x = self.tile.0;
        let mut y = self.tile.1 + 1;

        while y < ROWS {
            let idx = map_idx(x, y);

            if squares[idx].is_some() {
                return offset;
            }

            offset += 1;
            y += 1;
        }

        offset
    }

    pub fn green(tile: Pos) -> Dot {
        Dot {
            tile,
            color: DotColor::Green,
            kind: DotType::Good,
        }
    }

    pub fn blue(tile: Pos) -> Dot {
        Dot {
            tile,
            color: DotColor::Blue,
            kind: DotType::Good,
        }
    }

    pub fn new_blue_bad(tile: Pos) -> Dot {
        Dot {
            tile,
            color: DotColor::Blue,
            kind: DotType::Bad,
        }
    }

    pub fn new_green_bad(tile: Pos) -> Dot {
        Dot {
            tile,
            color: DotColor::Green,
            kind: DotType::Bad,
        }
    }

    pub fn new_red_bad(tile: Pos) -> Dot {
        Dot {
            tile,
            color: DotColor::Red,
            kind: DotType::Bad,
        }
    }

    pub fn random_bad(rng: &mut ThreadRng, tile: Pos) -> Dot {
        let color = DotColor::random(rng);

        Dot {
            tile,
            color,
            kind: DotType::Bad,
        }
    }

    pub fn random_good(rng: &mut ThreadRng, tile: Pos) -> Dot {
        let color = DotColor::random(rng);

        Dot {
            tile,
            color,
            kind: DotType::Good,
        }
    }

    pub fn img_tuple(&self) -> (i32, i32, i32, i32) {
        match self.kind {
            DotType::Good => match self.color {
                DotColor::Orange => ORANGE_DOT_IMG,
                DotColor::Red => RED_DOT_IMG,
                DotColor::Green => GREEN_DOT_IMG,
                DotColor::Blue => BLUE_DOT_IMG,
                DotColor::Yellow => YELLOW_DOT_IMG,
            },
            DotType::Bad => match self.color {
                DotColor::Orange => ORANGE_KODAMA_IMG,
                DotColor::Red => RED_KODAMA_IMG,
                DotColor::Green => GREEN_KODAMA_IMG,
                DotColor::Blue => BLUE_KODAMA_IMG,
                DotColor::Yellow => YELLOW_KODAMA_IMG,
            },
        }
    }

    pub fn img_rect(&self) -> SDL_Rect {
        tuple_to_rect(self.img_tuple())
    }
}
