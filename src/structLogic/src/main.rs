mod gender;
mod struct01;

use crate::struct01::area;


use gender::Gender;

fn main() {
    println!("Hello, world!");
    let user = User {
        name: String::from("ki mu han jin"),
        age: 24,
        is__employed: true,
        gender: Gender::Male,
    };

    if user.name == "ki mu han jin" {
        match user.gender {   //if let 으로 대체가능 if let 은 하나의 패턴만 처리할때 사용
            /*  ex)if let Gender::Male = gender {
                  println!("Male");
              } else {
              println!("Not Male");
              }
  */
            Gender::Male => println!("Kimu hanjin is Male"),
            _ => println!("The Other Male"),
        }
    } else {
        println!("The Other Male");
    }

    let rect1 = (30, 50);  //튜플로서 고정된 크기를 가지면 서로 다른 타입이 들어갈수있다
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1.0, rect1.1)
    );
}

struct User{
    name: String,
    age: u32,
    is__employed :  bool,
    gender: Gender,
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //&self   현재 Rectangle 인스턴스에 대한 불변 참조를 가리키는 매개변수
    fn area(&self)  -> u32 {
        self.width* self.height
    }

}
