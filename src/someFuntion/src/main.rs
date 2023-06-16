fn main() {
    println!("Hello, world!");
    let option_value = create_option_value();

    match option_value {
        Some(value) => println!("Option value: {}", value),
        None => println!("Option value is None"),
    }
}
fn create_option_value() -> Option<i32> {
    let value: i32 = 42;

    let option_value = Some(value);
    // `Some` 변형을 사용하여 `Option<i32>` 값을 생성

   return  option_value
    // 생성된 `Option<i32>` 값을 반환
}
