/**
 * 이거 오류납니다.
 * x 와 y 가 얼마나 살아있는지 모르기 때문에 컴파일러에서 오류로 잡아낸다.
 */
// pub fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

/**
 * 'a 와 같은 형식으로 어떤 참조가 얼마나 살아있는지 표시한다.
 */
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}