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
}