pub fn pattern_matching () {
    let my_number: i32 = 321;

    match my_number {
        0     => println!("0 입니다"),
        1 | 2 => println!("1 아니면 2 입니다"),
        3..10 => println!("3 ~ 10 사이 입니다"),
        _     => println!("뭘까")
    }
}