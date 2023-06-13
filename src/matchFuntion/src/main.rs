use::std::io;

fn main() {
    println!("Hello, world!");
    println!("Enter a number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: u32 = input.trim().parse().expect("Invalid input");

    matchesFuntion(number);
}


fn matchesFuntion(number: u32){

    match number {
        1 => println!("One"),
        2 | 3 | 5 => println!("Prime number"),
        4 | 6 | 8 | 10 => println!("Even number"),
        _ => println!("Other number"),  //else
    }
}