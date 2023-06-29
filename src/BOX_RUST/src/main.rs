fn main() {
    let  box_value : Box<i32> = Box::new(42);
    println!("Value: {}", *boxed_value);  //* 를 붙여야 컴파일 오류가 안남 주소에 접근하는 포인터이기 떄문
}
