fn multiply(a: i32, b: i32) -> i32 {
    return a * b
}

fn main() {
    // let 으로 선언하면 불변 값임
    let start: i32 = 1;
    let end: i32 = 10;
    let rest: i32 = 0;

    // mut 를 붙히면 변수가 되...
    let mut a: i32 = 5;
    println!("변경 전 a : {}", a);
    a = 6;
    assert_eq!(a, 6);

    // for 문
    // start ~ end-1 의 값으로 순환함
    for i in start..end {

        for j in start..end {
            let result = multiply(i, j);

            if result % 2 == rest {
                println!("{} * {} = {}", i, j, multiply(i, j))
            }
        }
    }
}