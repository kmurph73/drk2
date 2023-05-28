pub struct Keyboard {
    pub esc: bool,
    pub x: bool,
    pub c: bool,
    pub up: bool,
    pub right: bool,
    pub down: bool,
    pub left: bool,
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
        }
    }
}