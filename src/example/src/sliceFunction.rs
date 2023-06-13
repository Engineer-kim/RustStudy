pub fn fist_world(s: &String) -> usize { // &참조에 접근 값접근 아님
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {  //iter 는 컬렉션의 각요서에 ㅂ순차적으로 접근 가능하도록
        //enumerate 는  이터레이터에 대핸 인덱스와 해당 요소를 함께 반환하는 메서드 ==> 반복문내에서 요소의 인덱스 접근 할수있도록
        if item == b' ' {
            return i;
        }
    }
     s.len()
}
// s = input
// String = char[]
// char + char = String
// String = char[]
// for loop -> Char[] , if Hello => H , e, l, l, o
// if you insert to input bobo // bytes  =  String.asBytes = [b,o,b,o] // 4
// item = bytes[0] = b
// if item == b' ' false
// not exists empty -> String length return
// return = 4
// let result = first_world(bobo)
// println!("{:?}", result); < 4
// 문자열 조각== > 문자열 조각(&str)은 Rust에서 문자열을 참조하는 타입  크기 고정 및 불변  == 주소값
//(s: &String)  ==> String 타입의 변수를 참조하는 매개변수, 문자열을 전달시에는 String 타입의 변수사용해야함
//(s: &str)  ==> 문자열 조각을 참조하는 매개변수 입니다 ;;  불변 참조로ㅅ써 안전하고 효율적인 접근가능
