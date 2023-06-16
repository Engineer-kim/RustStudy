use std::fmt::write;

fn main() {
    let john = Person{name:"john".to_string(), age :23};
    println!("{}" ,john);
}

struct  Person{
    name: String,
    age: i32
}
impl std::fmt::Display for Person{
    fn fmt(&self, formatter:&mut std::fmt::Formatter) -> std::fmt::Result{
        write!(formatter, "이름:{} 나이:{}", self.name, self.age)
    }
}