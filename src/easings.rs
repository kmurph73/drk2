pub fn ease_in_circ(n: f64) -> f64 {
    let one = 1_f64;
    let two = 2_f64;

    one - f64::sqrt(one - n.powf(two))
}

pub fn ease_in_sine(n: f64) -> f64 {
    let one = 1_f64;
    let two = 2_f64;
    let pi = std::f64::consts::PI;

    one - f64::cos((n * pi) / two)
}
// function easeInCirc(x: number): number {
// return 1 - Math.sqrt(1 - Math.pow(x, 2));
// }
