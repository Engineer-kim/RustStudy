fn main() {
    let mut vec: Vec<i32> = Vec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("{:?}", vec); // Output: [1, 2, 3]

    let element = vec[1];
    println!("Element at index 1: {}", element); // Output: Element at index 1: 2

    vec.pop();
    println!("{:?}", vec); // Output: [1, 2]


    let mut newVector = vec![1, 2, 3, 4, 5, 6];  // 벡터는 인덱스 0부터 가짐

    let third: &i32 = &newVector[2];
    println!("The third element is {third}");
    newVector.push(7);
    println!("{:?}", newVector);
    let third: Option<&i32> = newVector.get(6);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}
