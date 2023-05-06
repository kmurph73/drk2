use rand::rngs::ThreadRng;
use rand::Rng;

use crate::{
    img_consts::{
        BLUE_DOT_IMG, BLUE_KODAMA_IMG, GREEN_DOT_IMG, GREEN_KODAMA_IMG, ORANGE_DOT_IMG,
        ORANGE_KODAMA_IMG, RED_DOT_IMG, RED_KODAMA_IMG, YELLOW_DOT_IMG, YELLOW_KODAMA_IMG,
    },
    my_sdl::SDL_Rect,
    pos::Pos,
    util::tuple_to_rect,
};

#[derive(Debug)]
pub enum DotColor {
    Orange,
    Blue,
    Red,
    Green,
    Yellow,
}

impl DotColor {
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

#[derive(Debug)]
pub enum DotType {
    Good,
    Bad,
}

#[derive(Debug)]
pub struct Dot {
    pub tile: Pos,
    pub color: DotColor,
    pub kind: DotType,
}

impl Dot {
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
