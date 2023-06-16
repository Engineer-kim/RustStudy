fn main() {
    println!("Hello, world!");

    let config_max = Some(3u8);   //3u8 ==> 3이라는 숫자의 부호 없는 8비트 정수를 의미 그냥 3 쓰면 i32 로 읽힘
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
