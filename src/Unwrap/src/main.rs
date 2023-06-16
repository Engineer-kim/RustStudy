fn main() {
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    let result1 = some_value.unwrap_or_else(|| {   //  ||  ==>    인자를 받지않는 클로저임 , 클로저를 나타내는 문법 , 익명함수로 코드블록을 캡쳐하는 독립적인 코드단위
        println!("Value is None");
        0  // 실제로 값이 있으므로 값을 출력함 0은 사용안됨
    });

    let result2 = none_value.unwrap_or_else(|| {
        println!("Value is None");
        panic!("Value is None");
    });

    println!("Result 1: {}", result1);  // Output: Result 1: 42
    println!("Result 2: {}", result2);  // This will panic!
}
