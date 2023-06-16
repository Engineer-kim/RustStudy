use std::fmt;

fn main() {
    println!("Hello, world!");
    let x = 5;

    // 클로저 정의
    let add_one = |num| num + 1;

    // 클로저 호출
    let result = add_one(x);

    println!("Result: {:?}", result);  // Output: Result: 6
}
//클로저 함수란 환경을 캡처할 수있ㄴ느 익명 함수 ==? 이게 무슨 뜻인지..?




