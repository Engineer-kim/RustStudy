fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
//위 코드에서 longest 함수의 라이프타임 파라미터 'a는 string1과 string2의 라이프타임이 동일하다는 것을 의미.
// 이를 통해 longest 함수가 반환하는 참조자가 더 오래 유효하도록 보장
fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    /// string1 과 String2 의 수명을 같게 하기 위함
    /// 같게 하는 이유: 참조자가 유효한 값을 참조하도록 보장  ==> 안전한 메모리 관리와 런타임 에러를 방지 (댕글링 포인터 예방)
    if string1.len() > string2.len() {
        string1
    } else {
        string2
    }
}