fn main() {
 function1()
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn function1(){
    let novel = String::from("Call me ki mu hanjin");   //문자열을 생성하고 초기화
    let first_sentence = novel.split(".")
        .next()  //다음 요소를 가져온다
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
