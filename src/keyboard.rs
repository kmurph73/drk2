pub struct Keyboard {
    pub esc: bool,
    pub x: bool,
    pub c: bool,
    pub y: bool,
    pub n: bool,
    pub up: bool,
    pub p: bool,
    pub r: bool,
    pub right: bool,
    pub down: bool,
    pub left: bool,
    pub space: bool,
}
pub struct KeyboardState {
    pub pressed: Keyboard,
    pub enabled: Keyboard,
}

impl Keyboard {
    pub fn init(default: bool) -> Keyboard {
        Keyboard {
            esc: default,
            x: default,
            c: default,
            up: default,
            right: default,
            down: default,
            left: default,
            y: default,
            n: default,
            p: default,
            r: default,
            space: default,
        }
    }
}
