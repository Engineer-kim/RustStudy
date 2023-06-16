pub fn matchesFuntion(number: u32){

    match number {
        1 => println!("One"),
        2 | 3 | 5 => println!("Prime number"),
        4 | 6 | 8 | 10 => println!("Even number"),
        _ => println!("Other number"),  //else
    }
}