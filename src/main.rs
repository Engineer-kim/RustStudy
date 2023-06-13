use std::alloc::System;
use std::sync::Condvar;
use std::task::Context;
use example::{example, example2};
use practice02::{numberOne  , numberTwo , numberThree };
use crate::practice02::numberFour;
use crate::practice02::cloneFuntion;


mod example;
mod practice02;

#[derive(Debug)]
struct Country {
    population: i32,
    a : String,
    b : String,
    c : String,
}
//
// fn add_numbers(a: i32, b: i32) -> i32 {
//     a + b
// }
//
// fn add_number_unit(a: i32, b: i32) {
//     a + b;
// }
//
fn calculate_length(s: &String) -> usize {  //&  빌림을통해 객체 값만 읽기 가능
    s.len()
}

fn main() {

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    // let res_ok = do_this(true).unwrap();
    // println!("res = {}", res_ok);
    // let res_err = do_this(false).expect("Hello, This is Err!");
    // println!("res_err = {}", res_err);            // 1
    anotherFuntion(5);
    test();
    example();
    example2();
    numberOne();
    numberTwo();
    numberThree();
    numberFour();
    println!("-------------------");
    cloneFuntion();
    let korea = Country {
        population: 50___________________________000_000,
        a: "korea".to_string(),
        b: String::from("test"),
        c: String::into("test2".to_string())
    };

    println!("{:#?}", korea);
}
fn anotherFuntion (x: i32){
    println!("The value of x is: {}", x);
}
fn test(){
    let x = 5;
    // 렉시컬 규칙(변수 이름이 중복될 경우, 가장 안쪽 스코프에서 선언된 변수가 우선적으로 사용)
    // 위의 x 값과 중괄호 안의 X 갑은 독립적이다
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);  //result ::::: 4
}

fn looopFun(){
    loop {
        println!("loop Funtion ~!!!!!!!!!!")
    }

}


//iter() 키워드는 배열의 요소를 반복하는 반복자 생성