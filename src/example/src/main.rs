mod sliceFunction;  //모듈 정의

use crate::sliceFunction::fist_world; // 프로젝트내에서 다른 모듈에 접근 가능

fn main() {
    let mut s = String::from("Hello world!");
    let value = String::from("babo");
    let word = fist_world(&s);
    println!("{}", word);
}
