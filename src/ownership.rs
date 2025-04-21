pub fn ownership (){
    let s1 = String::from("hello");
    let s2 = s1;  // s1의 소유권이 s2로 이동

    // println!("{}", s1); // 오류! s1은 더 이상 유효하지 않음
    println!("{}", s2); // 정상적으로 출력됨
}