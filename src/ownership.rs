pub fn ownership (){
    let s1: String = String::from("hello");
    let s2: String = s1;  // s1의 소유권이 s2로 이동

    // println!("{}", s1); // 오류! s1은 더 이상 유효하지 않음
    println!("{}", s2); // 정상적으로 출력됨

    let ib1: String = String::from("hello");
    let ib2: &String = &ib1; // ib2 에 ib1 불변 대여 ( Immutable Borrowing )

    // 대여자와 피대여자 모두 사용 가능
    println!("{}", ib1);
    println!("{}", ib2);
    
    let mut mb1: String = String::from("hello");
    let mb2: &mut String = &mut mb1; // mb1을 가변 대여 ( Mutable Borrowing )

    mb2.push_str(", world!"); // mb2를 통해 mb1을 변경

    println!("{}", mb2); // 정상 출력
    // println!("{}", mb1); // 오류! mb1은 더 이상 사용할 수 없음
}