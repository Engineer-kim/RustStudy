use std::io;

fn main() {
    println!("Enter a message:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let m = Message::Write(input.trim().to_string());
    m.call();
}

enum Message{
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl  Message {
    fn call(&self) {
        match self {
            Message::Write(msg) => {
                if let Ok(number) = msg.parse::<i32>() {
                    println!("Input is a number: {}", number);
                } else {
                    println!("Input is not a number: {}", msg);
                }
            }
            _ => {
                // 다른 종류의 메시지에 대한 처리를 여기에 작성
                println!("Unhandled message");
            }
        }
    }
}


