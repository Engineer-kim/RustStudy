use std::fmt::{Debug, Display};

struct Pair<T> where T: Display + PartialOrd { // 동일한 타입을 가진 두개의 필드를 갖는다
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: Display + PartialOrd> Pair<T>{  //제네릭 구현을 정의 합니다 ,  이 구현은 T가 Display와 PartialOrd 트레이트를 구현하는 타입에 대해서만 유효
    fn cmp_display(&self){
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


fn main() {
    Pair.cmp_display();
}