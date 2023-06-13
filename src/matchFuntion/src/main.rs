mod matchesFunction;

use::std::io;

fn main() {
    println!("Hello, world!");
    println!("Enter a number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: u32 = input.trim().parse().expect("Invalid input");

    matchesFuntion(number);
}


