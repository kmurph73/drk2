#[derive(Clone, Copy, Debug)]
pub struct Pos(pub i32, pub i32);

impl Pos {
    pub fn top_left_px(&self, square_size: i32) -> Pos {
        let Pos(x, y) = self;

        let y = (y + 2) * square_size;

        let x = (x + 1) * square_size;

        Pos(x, y)
    }
}
