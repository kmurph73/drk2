use crate::{
    img_consts::{
        EIGHTEEN_IMG, EIGHT_IMG, ELEVEN_IMG, FIFTEEN_IMG, FIVE_IMG, FOURTEEN_IMG, FOUR_IMG,
        NINETEEN_IMG, NINE_IMG, ONE_IMG, SEVENTEEN_IMG, SEVEN_IMG, SIXTEEN_IMG, SIX_IMG, TEN_IMG,
        THIRTEEN_IMG, THREE_IMG, TWELVE_IMG, TWENTY_IMG, TWO_IMG,
    },
    my_sdl::SDL_Rect,
};

pub struct NumberImages {
    pub one: (SDL_Rect, SDL_Rect),
    pub two: (SDL_Rect, SDL_Rect),
    pub three: (SDL_Rect, SDL_Rect),
    pub four: (SDL_Rect, SDL_Rect),
    pub five: (SDL_Rect, SDL_Rect),
    pub six: (SDL_Rect, SDL_Rect),
    pub seven: (SDL_Rect, SDL_Rect),
    pub eight: (SDL_Rect, SDL_Rect),
    pub nine: (SDL_Rect, SDL_Rect),
    pub ten: (SDL_Rect, SDL_Rect),
    pub eleven: (SDL_Rect, SDL_Rect),
    pub twelve: (SDL_Rect, SDL_Rect),
    pub thirteen: (SDL_Rect, SDL_Rect),
    pub fourteen: (SDL_Rect, SDL_Rect),
    pub fifteen: (SDL_Rect, SDL_Rect),
    pub sixteen: (SDL_Rect, SDL_Rect),
    pub seventeen: (SDL_Rect, SDL_Rect),
    pub eighteen: (SDL_Rect, SDL_Rect),
    pub nineteen: (SDL_Rect, SDL_Rect),
    pub twenty: (SDL_Rect, SDL_Rect),
}

pub fn make_number_rect(
    ratio: f64,
    (x, y, w, h): (i32, i32, i32, i32),
    modal: &SDL_Rect,
    lvl_btn_rect: &SDL_Rect,
) -> (SDL_Rect, SDL_Rect) {
    let dw = (w as f64 * ratio) as i32;
    let dh = (h as f64 * ratio) as i32;

    let (dx, _y) = modal.center(dw, dh);
    let (_x, dy) = lvl_btn_rect.center(dw, dh);

    let srcrect = SDL_Rect { x, y, w, h };
    let dstrect = SDL_Rect::dst_new(dx, dy, dw, dh);

    (srcrect, dstrect)
}

impl NumberImages {
    pub fn make_numbers(ratio: f64, modal: &SDL_Rect, lvl_btn_rect: &SDL_Rect) -> NumberImages {
        let one = make_number_rect(ratio, ONE_IMG, modal, lvl_btn_rect);
        let two = make_number_rect(ratio, TWO_IMG, modal, lvl_btn_rect);
        let three = make_number_rect(ratio, THREE_IMG, modal, lvl_btn_rect);
        let four = make_number_rect(ratio, FOUR_IMG, modal, lvl_btn_rect);
        let five = make_number_rect(ratio, FIVE_IMG, modal, lvl_btn_rect);
        let six = make_number_rect(ratio, SIX_IMG, modal, lvl_btn_rect);
        let seven = make_number_rect(ratio, SEVEN_IMG, modal, lvl_btn_rect);
        let eight = make_number_rect(ratio, EIGHT_IMG, modal, lvl_btn_rect);
        let nine = make_number_rect(ratio, NINE_IMG, modal, lvl_btn_rect);
        let ten = make_number_rect(ratio, TEN_IMG, modal, lvl_btn_rect);
        let eleven = make_number_rect(ratio, ELEVEN_IMG, modal, lvl_btn_rect);
        let twelve = make_number_rect(ratio, TWELVE_IMG, modal, lvl_btn_rect);
        let thirteen = make_number_rect(ratio, THIRTEEN_IMG, modal, lvl_btn_rect);
        let fourteen = make_number_rect(ratio, FOURTEEN_IMG, modal, lvl_btn_rect);
        let fifteen = make_number_rect(ratio, FIFTEEN_IMG, modal, lvl_btn_rect);
        let sixteen = make_number_rect(ratio, SIXTEEN_IMG, modal, lvl_btn_rect);
        let seventeen = make_number_rect(ratio, SEVENTEEN_IMG, modal, lvl_btn_rect);
        let eighteen = make_number_rect(ratio, EIGHTEEN_IMG, modal, lvl_btn_rect);
        let nineteen = make_number_rect(ratio, NINETEEN_IMG, modal, lvl_btn_rect);
        let twenty = make_number_rect(ratio, TWENTY_IMG, modal, lvl_btn_rect);

        NumberImages {
            one,
            two,
            three,
            four,
            five,
            six,
            seven,
            eight,
            nine,
            ten,
            eleven,
            twelve,
            thirteen,
            fourteen,
            fifteen,
            sixteen,
            seventeen,
            eighteen,
            nineteen,
            twenty,
        }
    }

    pub fn get_level_image(&self, level: usize) -> (&SDL_Rect, &SDL_Rect) {
        match level {
            1 => (&self.one.0, &self.one.1),
            2 => (&self.two.0, &self.two.1),
            3 => (&self.three.0, &self.three.1),
            4 => (&self.four.0, &self.four.1),
            5 => (&self.five.0, &self.five.1),
            6 => (&self.six.0, &self.six.1),
            7 => (&self.seven.0, &self.seven.1),
            8 => (&self.eight.0, &self.eight.1),
            9 => (&self.nine.0, &self.nine.1),
            10 => (&self.ten.0, &self.ten.1),
            11 => (&self.eleven.0, &self.eleven.1),
            12 => (&self.twelve.0, &self.twelve.1),
            13 => (&self.thirteen.0, &self.thirteen.1),
            14 => (&self.fourteen.0, &self.fourteen.1),
            15 => (&self.fifteen.0, &self.fifteen.1),
            16 => (&self.sixteen.0, &self.sixteen.1),
            17 => (&self.seventeen.0, &self.seventeen.1),
            18 => (&self.eighteen.0, &self.eighteen.1),
            19 => (&self.nineteen.0, &self.nineteen.1),
            20 => (&self.twenty.0, &self.twenty.1),
            _ => panic!("number 1-20 plz"),
        }
    }
}
