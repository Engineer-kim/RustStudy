trait Printable {  ///trait는 메서드의 집합
    fn print(&self); ///print의 메서드 ㄱ구현
}

struct Person {   ///구조체 정의
    name: String,
    age: u32,
}

impl Printable for Person {  ///printable 트레이트를 구현하기위해 impl 사용
    fn print(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
}

struct Car {
    brand: String,
    year: u32,
}

impl Printable for Car {
    fn print(&self) {
        println!("Brand: {}, Year: {}", self.brand, self.year);
    }
}

fn print_info<T: Printable>(item: T) {   /// T 타입에 대해 Printavgle 트ㄹ이트를 구현하는 아이템을 인자로 받는다  함수 내부에서 item의 print 메서드를 호출하여 정보를 출력 한다
    item.print();
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    let car = Car {
        brand: String::from("Tesla"),
        year: 2022,
    };

   /// Person과 Car 인스턴스를 생성

    print_info(person);
    print_info(car);
}
