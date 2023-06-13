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
        match user.gender {
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


