
// fn largest_i32(list: &[i32])  -> i32{   //i32 타입의 슬라이스를 매개변수로 받고 해당 슬라이스에서 가장 큰값 리턴
//     let mut largest = list[0];
//
//     for &item in list.iter(){
//         if item > largest{
//             largest = item;
//         }
//     }
//     largest
// }
//
// fn largest_char(list: &[char])  ->  char{
//     let mut largest = list[0];    // 가장 큰문자 리턴
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
 // 위 두함수가 구현로직은 동일하고 타입만 다르기 떄문에 아래와 같이 제네릭타입의 함수로 뺼수있다
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}



fn main() {
    //vec 는 가변길이의 배열

    //Vec! 는 초기값을 가지는 가변길의 배열을 만들수있따
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['a', 'b', 'c', 'a'];
    let result = largest(&chars);
    println!("The largest char is {}", result);
}