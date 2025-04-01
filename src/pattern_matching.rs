use std::{f32::consts::PI};

pub fn pattern_matching () {
    let my_number: i32 = 321;

    match my_number {
        0     => println!("0 입니다"),
        1 | 2 => println!("1 아니면 2 입니다"),
        3..10 => println!("3 ~ 10 사이 입니다"),
        _     => println!("뭘까")
    }
}
pub fn angle(vector: (f32, f32)) -> f32 {
    let pi = PI;
    match vector {
      (0.0, y) if y < 0.0 => 1.5 * pi,
      (0.0, _y) => 0.5 * pi,
      (x, y) => (y / x).atan()
    }
}