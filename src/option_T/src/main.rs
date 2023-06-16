fn main() {
    let x = 10;
    let y = 2;

    match divide(x, y) {
        //option<T> ==>
        //Some 은 Option<T> 엵거형 타입의 하나의 변형으로 Wrapper 이다 값 존재여부 나타냄
        Some(result) => println!("Result: {}"  ,result),
        None => println!("Cannot Divide byu Zero"),  // 값이 없는경우
    }
}
