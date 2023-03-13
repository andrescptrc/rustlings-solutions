// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.

#[derive(Debug)]
struct Coordinates {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
enum Message {
    Move(Coordinates),
    Echo(String),
    ChangeColor(Color),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move(Coordinates { x: 10, y: 30 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(Color(200, 255, 255)),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
