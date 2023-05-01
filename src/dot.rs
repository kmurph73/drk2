use crate::{
    img_consts::{
        BLUE_DOT_IMG, BLUE_KODAMA_IMG, GREEN_DOT_IMG, GREEN_KODAMA_IMG, ORANGE_DOT_IMG,
        ORANGE_KODAMA_IMG, RED_DOT_IMG, RED_KODAMA_IMG, YELLOW_DOT_IMG, YELLOW_KODAMA_IMG,
    },
    my_sdl::SDL_Rect,
    util::tuple_to_rect,
};

pub enum DotColor {
    Orange,
    Blue,
    Red,
    Green,
    Yellow,
}

pub enum DotType {
    Good,
    Bad,
}

pub struct Dot {
    pub tile: (i32, i32),
    pub color: DotColor,
    pub kind: DotType,
}

impl Dot {
    pub fn bad(tile: (i32, i32), color: DotColor) -> Dot {
        Dot {
            tile,
            color,
            kind: DotType::Bad,
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
