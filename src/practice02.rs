pub fn numberOne() {
    let mut x = 5;
    if x >= 5 {
        println!("The value of x is  5보다 크다:{}", x);
    } else {
        x = 6;
        println!("The value of x is maybe six :{}", x);
    }
}

pub fn numberTwo() {   //mut 와 차이가 있다
    let x = 5;
    let x = x+ 1;
    let x = x *2;

    println!("The value of x is :{}"  , x);

    //&str ==> 문자열 슬라이스 의미 (불변)
    let mut spaces = " ";
    let spaces_len = spaces.len();
    //spaces = spaces.len(); //usize 로써 크기가달라질수있음
}
pub fn  numberThree(){
    let guess: u32 = "42".parse().expect("fail to parsse");
}

pub fn numberFour(){
  let tup = (500 , 6.4 ,1);
  let  (x,y,z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
    println!("The value of z is: {}", z);
}


pub fn cloneFuntion(){
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}