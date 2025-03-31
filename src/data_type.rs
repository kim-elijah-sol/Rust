pub fn data_type () {
    // 정수형 타입

    // i8 은 Signed Integer 로 -128 ~ 127 값을 저장할 수 있음. 
    let x: i8 = -128;
    let y: i8 = 127;

    println!("{} {}", x, y);

    // u8 은 Unsigned Integer 로 0 ~ 255 값을 저장할 수 있음.
    let x: u8 = 0;
    let y: u8 = 255;

    println!("{} {}", x, y);

    // isize 및 usize 는 컴퓨터 환경에 따라 32bit, 64bit 로 계산된다.
    let x: isize = 0;
    let y: usize = 0;

    println!("{} {}", x, y);

    // 부동 소수점 타입

    // 타입 선언을 해주지 않으면 기본적으로 f64 로 할당됨.
    let x = 2.25;
    let y: f32 = 32.12;

    println!("{} {}", x, y);
}