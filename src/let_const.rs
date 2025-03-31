pub fn let_const () {
    // let 으로 선언하면 값을 재할당 할 수 없지만,
    let a = "hello world!";

    println!("{}", a);

    // 재선언할 수 있다.
    let a = "hello world욥";

    println!("{}", a);

    // const 로 선언하면 런타임 중 값이 변경될 수 없는 상수가 선언됨.
    // 컨벤션에 의해 UPPER_CASE 로 작성해야함.
    // 타입 명시가 필수임.
    const HI_TEXT: &str = "hihi";

    println!("{}", HI_TEXT);
}