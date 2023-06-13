mod gender;
use gender::Gender;

fn main() {
    println!("Hello, world!");
    let user = User{
        name: String::from("ki mu han jin"),
        age: 24,
        is__employed : true,
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


}

struct User{
    name: String,
    age: u32,
    is__employed :  bool,
    gender: Gender,
}


